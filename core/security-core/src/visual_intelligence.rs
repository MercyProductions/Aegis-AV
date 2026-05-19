use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VisualIntelligenceSurface {
    ThreatHeatmap,
    RiskPulse,
    IntegrityRing,
    NetworkTopology,
    TimelineReplay,
    ProcessTree,
    DeviceHealth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualLayer {
    pub surface: VisualIntelligenceSurface,
    pub live: bool,
    pub supports_replay: bool,
    pub data_source: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VisualIntelligenceEngine {
    pub layers: Vec<VisualLayer>,
}

impl VisualIntelligenceEngine {
    pub fn active_live_layers(&self) -> usize {
        self.layers.iter().filter(|layer| layer.live).count()
    }
}
