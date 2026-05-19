use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub signed_manifest: bool,
    pub enabled: bool,
    pub sandboxed: bool,
    pub permissions: Vec<PluginPermission>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission {
    ReadEvents,
    ReadDiagnostics,
    ReadNetworkMetadata,
    RequestScan,
    WriteReports,
    EnterpriseAdmin,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginRegistry {
    plugins: BTreeMap<String, PluginManifest>,
}

impl PluginRegistry {
    pub fn install(&mut self, manifest: PluginManifest) -> Vec<String> {
        let warnings = validate_manifest(&manifest);
        self.plugins.insert(manifest.plugin_id.clone(), manifest);
        warnings
    }

    pub fn disable(&mut self, plugin_id: &str) -> bool {
        if let Some(plugin) = self.plugins.get_mut(plugin_id) {
            plugin.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn enabled_plugins(&self) -> Vec<&PluginManifest> {
        self.plugins
            .values()
            .filter(|plugin| plugin.enabled)
            .collect()
    }
}

fn validate_manifest(manifest: &PluginManifest) -> Vec<String> {
    let mut warnings = Vec::new();
    if !manifest.sandboxed {
        warnings.push("plugin must run sandboxed before production enablement".to_string());
    }
    if !manifest.signed_manifest {
        warnings.push("plugin manifest is unsigned".to_string());
    }
    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_plugin_gets_warning() {
        let mut registry = PluginRegistry::default();
        let warnings = registry.install(PluginManifest {
            plugin_id: "network".to_string(),
            name: "Network Plugin".to_string(),
            version: "0.1.0".to_string(),
            signed_manifest: false,
            enabled: true,
            sandboxed: true,
            permissions: vec![PluginPermission::ReadNetworkMetadata],
        });

        assert_eq!(warnings, vec!["plugin manifest is unsigned"]);
    }
}
