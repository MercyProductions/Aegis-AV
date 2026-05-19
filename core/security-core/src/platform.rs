use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformLayer {
    Platform,
    CoreEngine,
    Ui,
    Driver,
    Update,
    Event,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformCapability {
    pub layer: PlatformLayer,
    pub windows_status: String,
    pub portability_notes: String,
    pub hard_dependency: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrossPlatformPlan {
    pub capabilities: Vec<PlatformCapability>,
}

impl CrossPlatformPlan {
    pub fn windows_first() -> Self {
        Self {
            capabilities: vec![
                PlatformCapability {
                    layer: PlatformLayer::Platform,
                    windows_status: "active".to_string(),
                    portability_notes: "OS adapters isolate filesystem, process, and service APIs."
                        .to_string(),
                    hard_dependency: true,
                },
                PlatformCapability {
                    layer: PlatformLayer::CoreEngine,
                    windows_status: "portable".to_string(),
                    portability_notes:
                        "Scanner, signatures, policies, events, and reports stay OS-neutral."
                            .to_string(),
                    hard_dependency: false,
                },
                PlatformCapability {
                    layer: PlatformLayer::Driver,
                    windows_status: "planned".to_string(),
                    portability_notes:
                        "Kernel-level work requires separate signed platform implementations."
                            .to_string(),
                    hard_dependency: true,
                },
                PlatformCapability {
                    layer: PlatformLayer::Plugin,
                    windows_status: "contract".to_string(),
                    portability_notes:
                        "Plugins declare permissions and avoid platform-specific calls by default."
                            .to_string(),
                    hard_dependency: false,
                },
            ],
        }
    }
}
