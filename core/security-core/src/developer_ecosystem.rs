use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum DeveloperApi {
    AegisSdk,
    PluginApi,
    AutomationApi,
    DashboardApi,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeveloperApiSurface {
    pub api: DeveloperApi,
    pub version: String,
    pub permissions: Vec<String>,
    pub stable: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeveloperEcosystem {
    pub surfaces: Vec<DeveloperApiSurface>,
}

impl DeveloperEcosystem {
    pub fn stable_surfaces(&self) -> Vec<&DeveloperApiSurface> {
        self.surfaces
            .iter()
            .filter(|surface| surface.stable)
            .collect()
    }
}
