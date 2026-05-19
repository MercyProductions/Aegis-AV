use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AegisModuleId {
    Core,
    Scanner,
    Realtime,
    Behavior,
    Quarantine,
    Diagnostics,
    Firewall,
    Network,
    ProcessMonitor,
    Ransomware,
    Updater,
    Ui,
    Telemetry,
    Engine,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleManifest {
    pub id: AegisModuleId,
    pub display_name: String,
    pub version: String,
    pub api_version: String,
    pub update_channel: String,
    pub can_hot_reload: bool,
    pub enabled: bool,
    pub dependencies: Vec<AegisModuleId>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleRegistry {
    modules: BTreeMap<AegisModuleId, ModuleManifest>,
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        let mut registry = Self {
            modules: BTreeMap::new(),
        };
        for module in default_modules() {
            registry.register(module);
        }
        registry
    }
}

impl ModuleRegistry {
    pub fn register(&mut self, manifest: ModuleManifest) -> Option<ModuleManifest> {
        self.modules.insert(manifest.id, manifest)
    }

    pub fn manifest(&self, id: AegisModuleId) -> Option<&ModuleManifest> {
        self.modules.get(&id)
    }

    pub fn set_enabled(&mut self, id: AegisModuleId, enabled: bool) -> bool {
        if let Some(module) = self.modules.get_mut(&id) {
            module.enabled = enabled;
            true
        } else {
            false
        }
    }

    pub fn active_modules(&self) -> Vec<&ModuleManifest> {
        self.modules
            .values()
            .filter(|module| module.enabled)
            .collect()
    }

    pub fn update_version(&mut self, id: AegisModuleId, version: impl Into<String>) -> bool {
        if let Some(module) = self.modules.get_mut(&id) {
            module.version = version.into();
            true
        } else {
            false
        }
    }

    pub fn dependency_warnings(&self) -> Vec<String> {
        let available: BTreeSet<_> = self.modules.keys().copied().collect();
        self.modules
            .values()
            .flat_map(|module| {
                module
                    .dependencies
                    .iter()
                    .filter(|dependency| !available.contains(dependency))
                    .map(|dependency| {
                        format!(
                            "{} depends on missing module {:?}",
                            module.display_name, dependency
                        )
                    })
            })
            .collect()
    }
}

pub fn default_modules() -> Vec<ModuleManifest> {
    use AegisModuleId::*;
    [
        (Core, "Aegis.Core", vec![]),
        (Scanner, "Aegis.Scanner", vec![Core]),
        (Realtime, "Aegis.Realtime", vec![Core, Scanner]),
        (Behavior, "Aegis.Behavior", vec![Core]),
        (Quarantine, "Aegis.Quarantine", vec![Core]),
        (Diagnostics, "Aegis.Diagnostics", vec![Core]),
        (Firewall, "Aegis.Firewall", vec![Core, Network]),
        (Network, "Aegis.Network", vec![Core]),
        (ProcessMonitor, "Aegis.ProcessMonitor", vec![Core]),
        (Ransomware, "Aegis.Ransomware", vec![Core, Behavior]),
        (Updater, "Aegis.Updater", vec![Core]),
        (Ui, "Aegis.UI", vec![Core]),
        (Telemetry, "Aegis.Telemetry", vec![Core]),
        (Engine, "Aegis.Engine", vec![Core, Scanner, Behavior]),
    ]
    .into_iter()
    .map(|(id, display_name, dependencies)| ModuleManifest {
        id,
        display_name: display_name.to_string(),
        version: "0.1.0".to_string(),
        api_version: "2026.05".to_string(),
        update_channel: "alpha".to_string(),
        can_hot_reload: !matches!(id, Core | Engine),
        enabled: true,
        dependencies,
        permissions: Vec::new(),
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_registry_has_core_modules() {
        let registry = ModuleRegistry::default();

        assert_eq!(registry.active_modules().len(), 14);
        assert!(registry.dependency_warnings().is_empty());
    }

    #[test]
    fn modules_can_be_independently_disabled() {
        let mut registry = ModuleRegistry::default();

        assert!(registry.set_enabled(AegisModuleId::Telemetry, false));
        assert!(!registry.manifest(AegisModuleId::Telemetry).unwrap().enabled);
    }
}
