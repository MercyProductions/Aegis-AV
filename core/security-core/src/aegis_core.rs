use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CoreSubsystem {
    ModuleManagement,
    EventRouting,
    Permissions,
    Settings,
    Telemetry,
    Automation,
    ServiceOrchestration,
    UiSynchronization,
    PluginLoading,
    DiagnosticsCoordination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoreCapability {
    pub subsystem: CoreSubsystem,
    pub enabled: bool,
    pub owner_module: String,
    pub health_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AegisCoreRuntime {
    pub version: String,
    pub local_first: bool,
    pub cloud_optional: bool,
    pub capabilities: Vec<CoreCapability>,
}

impl Default for AegisCoreRuntime {
    fn default() -> Self {
        Self {
            version: "0.1.0".to_string(),
            local_first: true,
            cloud_optional: true,
            capabilities: default_capabilities(),
        }
    }
}

impl AegisCoreRuntime {
    pub fn enabled_subsystems(&self) -> Vec<CoreSubsystem> {
        self.capabilities
            .iter()
            .filter(|capability| capability.enabled)
            .map(|capability| capability.subsystem)
            .collect()
    }

    pub fn average_health(&self) -> u8 {
        if self.capabilities.is_empty() {
            return 0;
        }
        let total = self
            .capabilities
            .iter()
            .map(|capability| capability.health_score as u16)
            .sum::<u16>();
        (total / self.capabilities.len() as u16) as u8
    }

    pub fn is_control_plane_ready(&self) -> bool {
        self.local_first
            && self.capabilities.iter().all(|capability| {
                capability.enabled
                    && capability.health_score >= 80
                    && !capability.owner_module.is_empty()
            })
    }
}

fn default_capabilities() -> Vec<CoreCapability> {
    [
        (CoreSubsystem::ModuleManagement, "Aegis.Core"),
        (CoreSubsystem::EventRouting, "Aegis.Core"),
        (CoreSubsystem::Permissions, "Aegis.Core"),
        (CoreSubsystem::Settings, "Aegis.Core"),
        (CoreSubsystem::Telemetry, "Aegis.Telemetry"),
        (CoreSubsystem::Automation, "Aegis.Engine"),
        (CoreSubsystem::ServiceOrchestration, "Aegis.Agent"),
        (CoreSubsystem::UiSynchronization, "Aegis.UI"),
        (CoreSubsystem::PluginLoading, "Aegis.PluginHost"),
        (CoreSubsystem::DiagnosticsCoordination, "Aegis.Diagnostics"),
    ]
    .into_iter()
    .map(|(subsystem, owner_module)| CoreCapability {
        subsystem,
        enabled: true,
        owner_module: owner_module.to_string(),
        health_score: 95,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_core_runtime_is_ready() {
        let runtime = AegisCoreRuntime::default();

        assert_eq!(runtime.capabilities.len(), 10);
        assert!(runtime.is_control_plane_ready());
        assert!(runtime
            .enabled_subsystems()
            .contains(&CoreSubsystem::EventRouting));
    }
}
