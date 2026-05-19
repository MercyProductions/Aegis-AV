use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityOperationsSnapshot {
    pub active_incidents: u32,
    pub recent_detections: u32,
    pub active_protection_layers: u8,
    pub device_health_score: u8,
    pub active_network_connections: u32,
    pub ransomware_alerts: u32,
    pub automation_events: u32,
    pub ecosystem_status: String,
}

impl SecurityOperationsSnapshot {
    pub fn attention_items(&self) -> u32 {
        self.active_incidents
            + self.recent_detections
            + self.ransomware_alerts
            + (100_u8.saturating_sub(self.device_health_score) as u32 / 10)
    }

    pub fn is_healthy(&self) -> bool {
        self.device_health_score >= 85 && self.active_incidents == 0 && self.ransomware_alerts == 0
    }
}
