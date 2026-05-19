use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustControlState {
    pub transparent_logging: bool,
    pub clear_permissions: bool,
    pub explainable_detections: bool,
    pub false_positive_controls: bool,
    pub easy_uninstall: bool,
    pub easy_exclusions: bool,
    pub privacy_controls: bool,
    pub minimal_telemetry: bool,
    pub user_controlled_features: bool,
}

impl Default for TrustControlState {
    fn default() -> Self {
        Self {
            transparent_logging: true,
            clear_permissions: true,
            explainable_detections: true,
            false_positive_controls: true,
            easy_uninstall: true,
            easy_exclusions: true,
            privacy_controls: true,
            minimal_telemetry: true,
            user_controlled_features: true,
        }
    }
}

pub struct TrustScoreEngine;

impl TrustScoreEngine {
    pub fn score(state: &TrustControlState) -> u8 {
        let controls = [
            state.transparent_logging,
            state.clear_permissions,
            state.explainable_detections,
            state.false_positive_controls,
            state.easy_uninstall,
            state.easy_exclusions,
            state.privacy_controls,
            state.minimal_telemetry,
            state.user_controlled_features,
        ];
        let passed = controls.iter().filter(|enabled| **enabled).count() as u8;
        ((passed as u16 * 100) / controls.len() as u16) as u8
    }
}
