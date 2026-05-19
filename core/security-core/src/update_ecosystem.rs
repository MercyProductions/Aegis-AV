use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DifferentialUpdate {
    pub from_version: String,
    pub to_version: String,
    pub module_id: String,
    pub patch_url: String,
    pub patch_sha256: String,
    pub signed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RollbackSnapshot {
    pub snapshot_id: String,
    pub version: String,
    pub module_versions: Vec<(String, String)>,
    pub created_at: String,
    pub snapshot_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OfflineUpdatePackage {
    pub package_id: String,
    pub channel: String,
    pub manifest_path: PathBuf,
    pub package_path: PathBuf,
    pub signed: bool,
    pub recovery_mode_supported: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateEcosystemAction {
    ApplyDifferential,
    RestoreRollbackSnapshot,
    ApplyEmergencyHotfix,
    LoadOfflinePackage,
    SwitchChannel(String),
    EnterRecoveryMode,
}

pub fn update_is_production_ready(update: &DifferentialUpdate) -> bool {
    update.signed && !update.patch_sha256.trim().is_empty()
}
