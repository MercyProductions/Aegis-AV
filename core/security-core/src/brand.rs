use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BrandIdentity {
    pub voice: String,
    pub terminology: Vec<String>,
    pub visual_language: Vec<String>,
    pub motion_principles: Vec<String>,
    pub installer_experience: String,
    pub documentation_standard: String,
}

impl BrandIdentity {
    pub fn premium_default() -> Self {
        Self {
            voice: "technical, calm, confident, and user-controlled".to_string(),
            terminology: vec![
                "Security Center".to_string(),
                "Protection Layers".to_string(),
                "Incident Report".to_string(),
                "Trust Controls".to_string(),
            ],
            visual_language: vec![
                "dark navy foundation".to_string(),
                "silver separators".to_string(),
                "electric blue accents".to_string(),
                "industrial command surfaces".to_string(),
            ],
            motion_principles: vec![
                "subtle status motion".to_string(),
                "fast response feedback".to_string(),
                "no distracting alerts".to_string(),
            ],
            installer_experience: "premium, explicit, easy to uninstall".to_string(),
            documentation_standard: "clear, defensive, transparent, and privacy-first".to_string(),
        }
    }
}
