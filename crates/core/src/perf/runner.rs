use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, mpsc};

use super::metrics::PerfMetrics;
use crate::client::send_request;
use crate::http::environment::EnvironmentChain;
use crate::http::request::Request;
use crate::transformers::request::transform_request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfConfig {
    pub duration: Duration,
    pub concurrency: usize,
    pub timeout: Duration,
}

impl Default for PerfConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_secs(60),
            concurrency: 10,
            timeout: Duration::from_secs(30),
        }
    }
}

pub struct PerfRunner {
    client: reqwest::Client,
    config: PerfConfig,
}

impl PerfRunner {
    pub fn new(client: reqwest::Client, config: PerfConfig) -> Self {
        Self { client, config }
    }

    pub async fn run(
        &self,
        request: Request,
        env: EnvironmentChain,
        progress: mpsc::Sender<PerfMetrics>,
    ) -> anyhow::Result<PerfMetrics> {
        let built_request = transform_request(self.client.clone(), request, env).await?;

        if built_request.try_clone().is_none() {
            anyhow::bail!("Request with file body not supported for performance testing");
        }

        let metrics = Arc::new(Mutex::new(PerfMetrics::new()));

        let mut tasks = Vec::new();
        let start_time = Instant::now();

        for _ in 0..self.config.concurrency {
            let client = self.client.clone();
            let request = built_request.try_clone().unwrap();
            let metrics = Arc::clone(&metrics);
            let timeout = self.config.timeout;
            let progress = progress.clone();
            let duration = self.config.duration;

            let task = tokio::spawn(async move {
                loop {
                    let request = request.try_clone().unwrap();
                    let metrics = Arc::clone(&metrics);
                    let client = client.clone();

                    if start_time.elapsed() >= duration {
                        return;
                    }

                    let result = tokio::time::timeout(timeout, send_request(client, request)).await;

                    let mut metrics = metrics.lock().await;
                    match result {
                        Ok(Ok(response)) => {
                            metrics.record_success(response.duration, response.status.as_u16());
                        }
                        Ok(Err(e)) => {
                            metrics.record_failure(e.to_string());
                        }
                        Err(_) => {
                            metrics.record_failure("Request timeout".to_string());
                        }
                    }

                    let mut snapshot = metrics.clone();
                    snapshot.total_duration = start_time.elapsed();
                    let _ = progress.send(snapshot).await;
                }
            });

            tasks.push(task);
        }

        for task in tasks {
            let _ = task.await;
        }

        let total_duration = start_time.elapsed();
        let mut final_metrics = metrics.lock().await.clone();
        final_metrics.total_duration = total_duration;

        Ok(final_metrics)
    }
}
