use futures::SinkExt;
use futures::channel::mpsc;
use iced::{Task, stream};
use std::sync::Arc;
use tokio::sync::oneshot;

use crate::state::{AppState, Tab};
use core::perf::{PerfMetrics, PerfRunner};
use core::persistence::request::read_request;

#[derive(Debug, Clone)]
pub enum PerfResult {
    Progress(Arc<PerfMetrics>),
    Completed(Arc<PerfMetrics>),
    Error(String),
    Cancelled,
}

pub fn start_benchmark<M: Send + Sync + 'static>(
    state: &mut AppState,
    msg: impl Fn(PerfResult) -> M + Send + Sync + 'static,
) -> Task<M> {
    let collection_request = match state.active_tab_mut() {
        Some(Tab::Perf(tab)) => tab.request,
        _ => return Task::none(),
    };

    let Some(collection_request) = collection_request else {
        return Task::none();
    };

    let Some(collection) = state.common.collections.get(collection_request.0) else {
        return Task::perform(
            async { PerfResult::Error("Collection not found".to_string()) },
            msg,
        );
    };

    let Some(request_ref) = state.common.collections.get_ref(collection_request) else {
        return Task::perform(
            async { PerfResult::Error("Request not found".to_string()) },
            msg,
        );
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

    let (cancel_tx, cancel_rx) = oneshot::channel();

    if let Some(Tab::Perf(tab)) = state.active_tab_mut() {
        tab.add_task(cancel_tx);
    }

    Task::run(
        stream::channel(100, |mut output: mpsc::Sender<PerfResult>| async move {
            let request = match read_request(&request_path).await {
                Ok(req) => req,
                Err(e) => {
                    let _ = output
                        .send(PerfResult::Error(format!("Failed to load request: {}", e)))
                        .await;
                    return;
                }
            };

            let runner = PerfRunner::new(client, config);

            let output_for_progress = output.clone();
            let result = runner
                .run(request, env_chain, cancel_rx, move |metrics| {
                    let mut output = output_for_progress.clone();
                    tokio::spawn(async move {
                        let _ = output.send(PerfResult::Progress(Arc::new(metrics))).await;
                    });
                })
                .await;

            match result {
                Ok(metrics) => {
                    let _ = output.send(PerfResult::Completed(Arc::new(metrics))).await;
                }
                Err(e) => {
                    if e.to_string().contains("cancelled") {
                        let _ = output.send(PerfResult::Cancelled).await;
                    } else {
                        let _ = output
                            .send(PerfResult::Error(format!("Benchmark failed: {}", e)))
                            .await;
                    }
                }
            }
        }),
        msg,
    )
}
