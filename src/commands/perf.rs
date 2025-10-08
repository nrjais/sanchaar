use core::http::EnvironmentChain;
use iced::Task;
use iced::task::{Straw, sipper};
use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::state::{AppState, Tab};
use core::perf::{PerfConfig, PerfMetrics, PerfRunner};
use core::persistence::request::read_request;

#[derive(Debug, Clone)]
pub enum PerfResult {
    Progress(PerfMetrics),
    Completed(Result<PerfMetrics, BenchmarkError>),
}

#[derive(Debug, Clone)]
pub enum BenchmarkError {
    Error(String),
    Cancelled,
}

pub fn benchmark(
    request_path: PathBuf,
    client: reqwest::Client,
    config: PerfConfig,
    env_chain: EnvironmentChain,
) -> impl Straw<PerfMetrics, PerfMetrics, BenchmarkError> {
    sipper(move |mut progress| async move {
        let request = match read_request(&request_path).await {
            Ok(req) => req,
            Err(e) => {
                return Err(BenchmarkError::Error(format!(
                    "Failed to load request: {}",
                    e
                )));
            }
        };

        let runner = PerfRunner::new(client, config);

        let (sender, mut receiver) = mpsc::channel(100);
        let handle = tokio::spawn(async move {
            while let Some(metrics) = receiver.recv().await {
                let _ = progress.send(metrics).await;
            }
        });

        let result = runner.run(request, env_chain, sender).await;

        handle.abort();

        match result {
            Ok(metrics) => Ok(metrics),
            Err(e) => Err(BenchmarkError::Error(e.to_string())),
        }
    })
}

pub fn start_benchmark(state: &mut AppState) -> Task<PerfResult> {
    let collection_request = match state.active_tab_mut() {
        Some(Tab::Perf(tab)) => tab.request,
        _ => return Task::none(),
    };

    let Some(collection_request) = collection_request else {
        return Task::none();
    };

    let Some(collection) = state.common.collections.get(collection_request.0) else {
        return Task::done(PerfResult::Completed(Err(BenchmarkError::Error(
            "Collection not found".to_string(),
        ))));
    };

    let Some(request_ref) = state.common.collections.get_ref(collection_request) else {
        return Task::done(PerfResult::Completed(Err(BenchmarkError::Error(
            "Request not found".to_string(),
        ))));
    };

    let request_path = request_ref.path.clone();
    let env_chain = collection.env_chain();
    let disable_ssl = collection.disable_ssl;

    let config = match state.active_tab_mut() {
        Some(Tab::Perf(tab)) => {
            tab.start_test();
            tab.config.clone()
        }
        _ => return Task::none(),
    };

    let client = if disable_ssl {
        state.common.client_no_ssl.clone()
    } else {
        state.common.client.clone()
    };

    let (task, handle) = Task::sip(
        benchmark(request_path, client, config, env_chain),
        PerfResult::Progress,
        PerfResult::Completed,
    )
    .abortable();

    if let Some(Tab::Perf(tab)) = state.active_tab_mut() {
        tab.add_task(handle);
    }

    task
}
