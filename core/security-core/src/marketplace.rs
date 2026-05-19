use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketplaceListing {
    pub id: String,
    pub name: String,
    pub publisher: String,
    pub version: String,
    pub permissions: Vec<String>,
    pub signed_manifest: bool,
    pub sandbox_required: bool,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MarketplaceValidation {
    pub listing_id: String,
    pub install_allowed: bool,
    pub reasons: Vec<String>,
}

pub struct MarketplaceRegistry;

impl MarketplaceRegistry {
    pub fn validate(listing: &MarketplaceListing) -> MarketplaceValidation {
        let mut reasons = Vec::new();
        if !listing.signed_manifest {
            reasons.push("Plugin manifest must be signed before install.".to_string());
        }
        if !listing.sandbox_required {
            reasons.push("Plugin must run with sandbox isolation.".to_string());
        }
        if listing.permissions.is_empty() {
            reasons.push("Plugin must declare permissions explicitly.".to_string());
        }

        MarketplaceValidation {
            listing_id: listing.id.clone(),
            install_allowed: reasons.is_empty(),
            reasons,
        }
    }
}
