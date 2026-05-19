use crate::behavior::BehaviorRuleMatch;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxPolicy {
    pub filesystem_access: AccessLevel,
    pub network_access: AccessLevel,
    pub max_runtime_seconds: u32,
    pub destroy_environment_after_run: bool,
    pub allow_real_malware_samples: bool,
}

impl Default for SandboxPolicy {
    fn default() -> Self {
        Self {
            filesystem_access: AccessLevel::Limited,
            network_access: AccessLevel::Blocked,
            max_runtime_seconds: 90,
            destroy_environment_after_run: true,
            allow_real_malware_samples: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AccessLevel {
    Blocked,
    Limited,
    Observed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxSession {
    pub session_id: String,
    pub sample_path: PathBuf,
    pub policy: SandboxPolicy,
    pub started_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SandboxReport {
    pub session_id: String,
    pub sample_path: PathBuf,
    pub behavior_observed: Vec<BehaviorRuleMatch>,
    pub files_touched: Vec<PathBuf>,
    pub network_attempts: Vec<String>,
    pub environment_destroyed: bool,
    pub summary: String,
}

impl SandboxSession {
    pub fn simulate_harmless_report(&self) -> SandboxReport {
        SandboxReport {
            session_id: self.session_id.clone(),
            sample_path: self.sample_path.clone(),
            behavior_observed: Vec::new(),
            files_touched: vec![self.sample_path.clone()],
            network_attempts: Vec::new(),
            environment_destroyed: self.policy.destroy_environment_after_run,
            summary: "Harmless simulation completed without suspicious behavior.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_defaults_do_not_allow_real_malware() {
        let policy = SandboxPolicy::default();

        assert!(!policy.allow_real_malware_samples);
        assert_eq!(policy.network_access, AccessLevel::Blocked);
    }
}
