use iced::Task;
use std::sync::Arc;

use crate::state::{AppState, Tab};
use core::perf::PerfMetrics;

#[derive(Debug, Clone)]
pub enum PerfResult {
    Completed(Arc<PerfMetrics>),
    Error(String),
}

pub fn run_perf_test<M: Send + Sync + 'static>(
    state: &mut AppState,
    msg: impl Fn(PerfResult) -> M + Send + Sync + 'static,
) -> Task<M> {
    let Some(Tab::Perf(_tab)) = state.active_tab_mut() else {
        return Task::none();
    };

    Task::perform(
        async {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            PerfResult::Error("Performance testing not yet fully implemented".to_string())
        },
        msg,
    )
}
