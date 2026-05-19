use crate::performance::ScanPriority;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PolicyProfile {
    Balanced,
    Strict,
    Performance,
    SilentGamingMode,
    EnterpriseManaged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtectionPolicy {
    pub profile: PolicyProfile,
    pub auto_quarantine: bool,
    pub notification_level: NotificationLevel,
    pub scan_priority: ScanPriority,
    pub cpu_limit_percent: Option<u8>,
    pub real_time_scan_depth: ScanDepth,
    pub behavioral_sensitivity: Sensitivity,
    pub ransomware_protection: bool,
    pub update_behavior: UpdateBehavior,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationLevel {
    Verbose,
    ImportantOnly,
    CriticalOnly,
    Managed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanDepth {
    Light,
    Standard,
    Deep,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum Sensitivity {
    Low,
    Normal,
    High,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateBehavior {
    AutomaticVerifiedOnly,
    PromptBeforeApply,
    Managed,
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn policy_for(profile: PolicyProfile) -> ProtectionPolicy {
        match profile {
            PolicyProfile::Balanced => ProtectionPolicy {
                profile,
                auto_quarantine: false,
                notification_level: NotificationLevel::ImportantOnly,
                scan_priority: ScanPriority::Balanced,
                cpu_limit_percent: Some(40),
                real_time_scan_depth: ScanDepth::Standard,
                behavioral_sensitivity: Sensitivity::Normal,
                ransomware_protection: true,
                update_behavior: UpdateBehavior::AutomaticVerifiedOnly,
            },
            PolicyProfile::Strict => ProtectionPolicy {
                profile,
                auto_quarantine: true,
                notification_level: NotificationLevel::Verbose,
                scan_priority: ScanPriority::High,
                cpu_limit_percent: Some(70),
                real_time_scan_depth: ScanDepth::Deep,
                behavioral_sensitivity: Sensitivity::High,
                ransomware_protection: true,
                update_behavior: UpdateBehavior::AutomaticVerifiedOnly,
            },
            PolicyProfile::Performance => ProtectionPolicy {
                profile,
                auto_quarantine: false,
                notification_level: NotificationLevel::ImportantOnly,
                scan_priority: ScanPriority::Low,
                cpu_limit_percent: Some(20),
                real_time_scan_depth: ScanDepth::Light,
                behavioral_sensitivity: Sensitivity::Low,
                ransomware_protection: true,
                update_behavior: UpdateBehavior::PromptBeforeApply,
            },
            PolicyProfile::SilentGamingMode => ProtectionPolicy {
                profile,
                auto_quarantine: false,
                notification_level: NotificationLevel::CriticalOnly,
                scan_priority: ScanPriority::Low,
                cpu_limit_percent: Some(15),
                real_time_scan_depth: ScanDepth::Light,
                behavioral_sensitivity: Sensitivity::Low,
                ransomware_protection: true,
                update_behavior: UpdateBehavior::PromptBeforeApply,
            },
            PolicyProfile::EnterpriseManaged => ProtectionPolicy {
                profile,
                auto_quarantine: true,
                notification_level: NotificationLevel::Managed,
                scan_priority: ScanPriority::Balanced,
                cpu_limit_percent: Some(50),
                real_time_scan_depth: ScanDepth::Deep,
                behavioral_sensitivity: Sensitivity::High,
                ransomware_protection: true,
                update_behavior: UpdateBehavior::Managed,
            },
        }
    }

    pub fn validate(policy: &ProtectionPolicy) -> Vec<String> {
        let mut warnings = Vec::new();
        if matches!(policy.cpu_limit_percent, Some(0) | Some(101..=u8::MAX)) {
            warnings.push("CPU limit must be between 1 and 100 percent.".to_string());
        }
        if policy.auto_quarantine && policy.behavioral_sensitivity == Sensitivity::Low {
            warnings.push(
                "Auto quarantine with low behavior sensitivity can hide useful context."
                    .to_string(),
            );
        }
        if !policy.ransomware_protection && policy.profile != PolicyProfile::Performance {
            warnings.push(
                "Ransomware protection should stay enabled outside performance profiles."
                    .to_string(),
            );
        }
        warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strict_policy_enables_auto_quarantine() {
        let policy = PolicyEngine::policy_for(PolicyProfile::Strict);

        assert!(policy.auto_quarantine);
        assert_eq!(policy.behavioral_sensitivity, Sensitivity::High);
    }
}
