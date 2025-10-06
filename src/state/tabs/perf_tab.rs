use core::http::CollectionRequest;
use core::perf::{PerfConfig, PerfMetrics, PerfStats};
use core::utils::SendOnDrop;
use tokio::sync::oneshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfState {
    Idle,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug)]
pub struct PerfTab {
    pub name: String,
    pub request: Option<CollectionRequest>,
    pub config: PerfConfig,
    pub state: PerfState,
    pub metrics: Option<PerfMetrics>,
    pub stats: Option<PerfStats>,
    pub progress: u64,
    pub split_at: f32,
    pub cancel: SendOnDrop,
}

impl PerfTab {
    pub fn new() -> Self {
        Self {
            name: "Performance".to_owned(),
            request: None,
            config: PerfConfig::default(),
            state: PerfState::Idle,
            metrics: None,
            stats: None,
            progress: 0,
            split_at: 0.45,
            cancel: SendOnDrop::new(),
        }
    }

    pub fn set_split_at(&mut self, at: f32) {
        self.split_at = at.clamp(0.25, 0.70);
    }

    pub fn set_request(&mut self, request: CollectionRequest) {
        self.request = Some(request);
    }

    pub fn start_test(&mut self) {
        self.reset();
        self.state = PerfState::Running;
    }

    pub fn update_progress(&mut self, metrics: PerfMetrics) {
        self.progress = metrics.total_requests;
        let stats = metrics.calculate_stats();
        self.metrics = Some(metrics);
        self.stats = Some(stats);
    }

    pub fn complete_test(&mut self, metrics: PerfMetrics) {
        let stats = metrics.calculate_stats();
        self.metrics = Some(metrics);
        self.stats = Some(stats);
        self.state = PerfState::Completed;
    }

    pub fn fail_test(&mut self) {
        self.state = PerfState::Failed;
    }

    pub fn cancel_test(&mut self) {
        self.cancel_tasks();
        self.state = PerfState::Cancelled;
    }

    pub fn reset(&mut self) {
        self.cancel.cancel();
        self.state = PerfState::Idle;
        self.progress = 0;
        self.metrics = None;
        self.stats = None;
    }

    pub fn cancel_tasks(&mut self) {
        self.cancel.cancel();
    }

    pub fn add_task(&mut self, task: oneshot::Sender<()>) {
        self.cancel.with(task);
    }
}

impl Default for PerfTab {
    fn default() -> Self {
        Self::new()
    }
}
