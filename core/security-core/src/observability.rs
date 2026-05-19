use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservabilityMetric {
    pub name: String,
    pub value: String,
    pub status: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerformanceTrace {
    pub id: String,
    pub component: String,
    pub duration_ms: u64,
    pub memory_delta_kb: i64,
    pub events_captured: u32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservabilitySnapshot {
    pub metrics: Vec<ObservabilityMetric>,
    pub traces: Vec<PerformanceTrace>,
}

impl ObservabilitySnapshot {
    pub fn reliability_score(&self) -> u8 {
        if self.metrics.is_empty() {
            return 0;
        }
        let healthy = self
            .metrics
            .iter()
            .filter(|metric| metric.status.eq_ignore_ascii_case("healthy"))
            .count() as u16;
        ((healthy * 100) / self.metrics.len() as u16) as u8
    }
}
