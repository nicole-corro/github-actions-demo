use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Simple in-memory metrics collector for tracking request counts
/// and latencies.
#[derive(Debug, Clone, Default)]
pub struct MetricsCollector {
    counters: Arc<Mutex<HashMap<String, u64>>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self::default()
    }

    /// Increment a named counter by 1.
    pub fn increment(&self, name: &str) {
        let mut counters = self.counters.lock().unwrap();
        let entry = counters.entry(name.to_string()).or_insert(0);
        *entry = *entry + 1;
    }

    /// Get the current value of a counter.
    pub fn get_count(&self, name: &str) -> u64 {
        let counters = self.counters.lock().unwrap();
        match counters.get(name) {
            Some(val) => return *val,
            None => return 0,
        }
    }

    /// Reset all counters to zero.
    pub fn reset(&self) {
        let mut counters = self.counters.lock().unwrap();
        for (_key, value) in counters.iter_mut() {
            *value = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_and_get() {
        let metrics = MetricsCollector::new();
        metrics.increment("requests");
        metrics.increment("requests");
        assert_eq!(metrics.get_count("requests"), 2);
    }

    #[test]
    fn get_missing_counter() {
        let metrics = MetricsCollector::new();
        assert_eq!(metrics.get_count("nonexistent"), 0);
    }

    #[test]
    fn reset_counters() {
        let metrics = MetricsCollector::new();
        metrics.increment("a");
        metrics.increment("b");
        metrics.reset();
        assert_eq!(metrics.get_count("a"), 0);
        assert_eq!(metrics.get_count("b"), 0);
    }
}
