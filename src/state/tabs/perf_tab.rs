use core::http::{CollectionKey, CollectionRequest, RequestId};
use core::perf::{PerfConfig, PerfMetrics, PerfStats};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfState {
    Idle,
    Running,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct PerfTab {
    pub name: String,
    pub collection: Option<CollectionKey>,
    pub request: Option<RequestId>,
    pub config: PerfConfig,
    pub state: PerfState,
    pub metrics: Option<PerfMetrics>,
    pub stats: Option<PerfStats>,
    pub progress: u64,
    pub split_at: f32,
}

impl PerfTab {
    pub fn new() -> Self {
        Self {
            name: "Performance".to_owned(),
            collection: None,
            request: None,
            config: PerfConfig::default(),
            state: PerfState::Idle,
            metrics: None,
            stats: None,
            progress: 0,
            split_at: 0.45,
        }
    }

    pub fn set_split_at(&mut self, at: f32) {
        self.split_at = at.clamp(0.25, 0.70);
    }

    pub fn set_request(&mut self, request: CollectionRequest) {
        self.collection = Some(request.0);
        self.request = Some(request.1);
    }

    pub fn start_test(&mut self) {
        self.state = PerfState::Running;
        self.progress = 0;
        self.metrics = None;
        self.stats = None;
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

    pub fn reset(&mut self) {
        self.state = PerfState::Idle;
        self.progress = 0;
        self.metrics = None;
        self.stats = None;
    }
}

impl Default for PerfTab {
    fn default() -> Self {
        Self::new()
    }
}
