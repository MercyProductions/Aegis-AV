use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentMode {
    LocalFirst,
    OptionalCloudSync,
    EnterpriseFleet,
    EdgeProcessing,
    ModularDeployment,
    OfflineMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributedCapability {
    pub mode: DeploymentMode,
    pub enabled: bool,
    pub privacy_preserving: bool,
    pub notes: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributedArchitecturePlan {
    pub capabilities: Vec<DistributedCapability>,
}

impl DistributedArchitecturePlan {
    pub fn supports_offline_operation(&self) -> bool {
        self.capabilities
            .iter()
            .any(|capability| capability.mode == DeploymentMode::OfflineMode && capability.enabled)
    }
}
