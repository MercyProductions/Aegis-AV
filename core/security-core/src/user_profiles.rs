use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AegisUserProfile {
    HomeUser,
    PowerUser,
    Developer,
    Enterprise,
    SilentMode,
    GamingMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UiComplexity {
    Simple,
    Standard,
    Advanced,
    Managed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserProfileSettings {
    pub profile: AegisUserProfile,
    pub ui_complexity: UiComplexity,
    pub scan_depth: String,
    pub notification_style: String,
    pub automation_enabled: bool,
    pub diagnostics_visible: bool,
}

pub struct UserProfileEngine;

impl UserProfileEngine {
    pub fn settings_for(profile: AegisUserProfile) -> UserProfileSettings {
        match profile {
            AegisUserProfile::HomeUser => UserProfileSettings {
                profile,
                ui_complexity: UiComplexity::Standard,
                scan_depth: "balanced".to_string(),
                notification_style: "important".to_string(),
                automation_enabled: true,
                diagnostics_visible: false,
            },
            AegisUserProfile::PowerUser => UserProfileSettings {
                profile,
                ui_complexity: UiComplexity::Advanced,
                scan_depth: "deep".to_string(),
                notification_style: "detailed".to_string(),
                automation_enabled: true,
                diagnostics_visible: true,
            },
            AegisUserProfile::Developer => UserProfileSettings {
                profile,
                ui_complexity: UiComplexity::Advanced,
                scan_depth: "developer".to_string(),
                notification_style: "low_noise".to_string(),
                automation_enabled: true,
                diagnostics_visible: true,
            },
            AegisUserProfile::Enterprise => UserProfileSettings {
                profile,
                ui_complexity: UiComplexity::Managed,
                scan_depth: "policy".to_string(),
                notification_style: "managed".to_string(),
                automation_enabled: true,
                diagnostics_visible: true,
            },
            AegisUserProfile::SilentMode => UserProfileSettings {
                profile,
                ui_complexity: UiComplexity::Simple,
                scan_depth: "light".to_string(),
                notification_style: "critical_only".to_string(),
                automation_enabled: false,
                diagnostics_visible: false,
            },
            AegisUserProfile::GamingMode => UserProfileSettings {
                profile,
                ui_complexity: UiComplexity::Simple,
                scan_depth: "light".to_string(),
                notification_style: "critical_only".to_string(),
                automation_enabled: true,
                diagnostics_visible: false,
            },
        }
    }
}
