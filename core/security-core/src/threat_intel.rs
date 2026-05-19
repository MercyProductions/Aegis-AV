use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatIntelSnapshot {
    pub signature_version: String,
    pub rule_pack_version: String,
    pub detection_categories: Vec<DetectionCategory>,
    pub recent_local_trends: Vec<String>,
    pub common_suspicious_locations: Vec<String>,
    pub blocked_event_types: BTreeMap<String, u32>,
    pub update_changelog: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DetectionCategory {
    pub name: String,
    pub count: u32,
    pub severity: String,
}

impl ThreatIntelSnapshot {
    pub fn top_blocked_event(&self) -> Option<(&String, &u32)> {
        self.blocked_event_types
            .iter()
            .max_by_key(|(_, count)| *count)
    }
}
