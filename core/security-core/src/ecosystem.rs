use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AegisIntegration {
    pub product: String,
    pub enabled: bool,
    pub status: String,
    pub deep_link: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EcosystemRegistry {
    pub integrations: Vec<AegisIntegration>,
}

impl EcosystemRegistry {
    pub fn default_aegis_suite() -> Self {
        Self {
            integrations: vec![
                integration("Aegis Diagnostics", true, "local module ready"),
                integration("Aegis Firewall", true, "visibility mode"),
                integration("Aegis VPN", false, "planned"),
                integration("Aegis Backup", false, "planned"),
                integration("Aegis Cloud", false, "optional sync"),
                integration("Aegis AI Assistant", true, "explain-only mode"),
                integration("Aegis Identity Protection", false, "roadmap"),
            ],
        }
    }
}

fn integration(product: &str, enabled: bool, status: &str) -> AegisIntegration {
    AegisIntegration {
        product: product.to_string(),
        enabled,
        status: status.to_string(),
        deep_link: None,
    }
}
