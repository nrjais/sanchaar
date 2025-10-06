use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Histogram bucket for efficient percentile calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct Histogram {
    /// Bucket boundaries in milliseconds
    buckets: Vec<f64>,
    /// Count of values in each bucket
    counts: Vec<AtomicU64>,
    /// Total count of all values
    total_count: AtomicU64,
    /// Sum of all values (for average calculation)
    sum_millis: AtomicU64,
    /// Minimum value seen
    min_millis: AtomicU64,
    /// Maximum value seen
    max_millis: AtomicU64,
}

impl Clone for Histogram {
    fn clone(&self) -> Self {
        let buckets = self.buckets.clone();
        let counts = self
            .counts
            .iter()
            .map(|c| AtomicU64::new(c.load(Ordering::Relaxed)))
            .collect();

        Self {
            buckets,
            counts,
            total_count: AtomicU64::new(self.total_count.load(Ordering::Relaxed)),
            sum_millis: AtomicU64::new(self.sum_millis.load(Ordering::Relaxed)),
            min_millis: AtomicU64::new(self.min_millis.load(Ordering::Relaxed)),
            max_millis: AtomicU64::new(self.max_millis.load(Ordering::Relaxed)),
        }
    }
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

impl Histogram {
    pub fn new() -> Self {
        let buckets = vec![
            1.0,
            2.0,
            5.0,
            10.0,
            25.0,
            50.0,
            100.0,
            200.0,
            300.0,
            500.0,
            700.0,
            900.0,
            1200.0,
            1500.0,
            2000.0,
            2500.0,
            3000.0,
            4000.0,
            5000.0,
            7000.0,
            10000.0,
            15000.0,
            20000.0,
            25000.0,
            30000.0,
            40000.0,
            50000.0,
            60000.0,
            f64::INFINITY,
        ];
        let counts = buckets.iter().map(|_| AtomicU64::new(0)).collect();

        Self {
            buckets,
            counts,
            total_count: AtomicU64::new(0),
            sum_millis: AtomicU64::new(0),
            min_millis: AtomicU64::new(u64::MAX),
            max_millis: AtomicU64::new(0),
        }
    }

    fn observe(&self, duration: Duration) {
        let millis = duration.as_millis() as u64;
        let millis_f64 = duration.as_secs_f64() * 1000.0;

        for (i, &boundary) in self.buckets.iter().enumerate() {
            if millis_f64 <= boundary {
                self.counts[i].fetch_add(1, Ordering::Relaxed);
                break;
            }
        }

        self.total_count.fetch_add(1, Ordering::Relaxed);
        self.sum_millis.fetch_add(millis, Ordering::Relaxed);

        let mut current_min = self.min_millis.load(Ordering::Relaxed);
        while millis < current_min {
            match self.min_millis.compare_exchange(
                current_min,
                millis,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_min = x,
            }
        }

        let mut current_max = self.max_millis.load(Ordering::Relaxed);
        while millis > current_max {
            match self.max_millis.compare_exchange(
                current_max,
                millis,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(x) => current_max = x,
            }
        }
    }

    fn percentile(&self, p: f64) -> Option<Duration> {
        let total = self.total_count.load(Ordering::Relaxed);
        if total == 0 {
            return None;
        }

        let target = (total as f64 * p) as u64;
        let mut cumulative = 0u64;

        for (i, count) in self.counts.iter().enumerate() {
            cumulative += count.load(Ordering::Relaxed);
            if cumulative >= target {
                let millis = self.buckets[i];
                return Some(Duration::from_millis(millis as u64));
            }
        }

        None
    }

    fn min(&self) -> Option<Duration> {
        let min_val = self.min_millis.load(Ordering::Relaxed);
        if min_val == u64::MAX {
            None
        } else {
            Some(Duration::from_millis(min_val))
        }
    }

    fn max(&self) -> Option<Duration> {
        let max_val = self.max_millis.load(Ordering::Relaxed);
        if max_val == 0 && self.total_count.load(Ordering::Relaxed) == 0 {
            None
        } else {
            Some(Duration::from_millis(max_val))
        }
    }

    fn average(&self) -> Option<Duration> {
        let total = self.total_count.load(Ordering::Relaxed);
        if total == 0 {
            return None;
        }

        let sum = self.sum_millis.load(Ordering::Relaxed);
        Some(Duration::from_millis(sum / total))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub total_duration: Duration,
    #[serde(skip, default = "Histogram::new")]
    histogram: Histogram,
    pub status_codes: HashMap<u16, u64>,
    pub errors: HashMap<String, u64>,
}

impl PerfMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_duration: Duration::ZERO,
            histogram: Histogram::new(),
            status_codes: HashMap::new(),
            errors: HashMap::new(),
        }
    }

    pub fn record_success(&mut self, duration: Duration, status_code: u16) {
        self.total_requests += 1;
        self.successful_requests += 1;
        self.histogram.observe(duration);
        *self.status_codes.entry(status_code).or_insert(0) += 1;
    }

    pub fn record_failure(&mut self, error: String) {
        self.total_requests += 1;
        self.failed_requests += 1;
        *self.errors.entry(error).or_insert(0) += 1;
    }

    pub fn calculate_stats(&self) -> PerfStats {
        let p50 = self.histogram.percentile(0.50);
        let p95 = self.histogram.percentile(0.95);
        let p99 = self.histogram.percentile(0.99);
        let min = self.histogram.min();
        let max = self.histogram.max();
        let avg = self.histogram.average();

        let requests_per_second = if !self.total_duration.is_zero() {
            self.total_requests as f64 / self.total_duration.as_secs_f64()
        } else {
            0.0
        };

        PerfStats {
            total_requests: self.total_requests,
            successful_requests: self.successful_requests,
            failed_requests: self.failed_requests,
            requests_per_second,
            p50,
            p95,
            p99,
            min,
            max,
            avg,
            status_codes: self.status_codes.clone(),
            errors: self.errors.clone(),
            total_duration: self.total_duration,
        }
    }
}

impl Default for PerfMetrics {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub requests_per_second: f64,
    pub p50: Option<Duration>,
    pub p95: Option<Duration>,
    pub p99: Option<Duration>,
    pub min: Option<Duration>,
    pub max: Option<Duration>,
    pub avg: Option<Duration>,
    pub status_codes: HashMap<u16, u64>,
    pub errors: HashMap<String, u64>,
    pub total_duration: Duration,
}
