use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudSyncSettings {
    pub enabled: bool,
    pub account_id: Option<String>,
    pub sync_license_status: bool,
    pub sync_device_name: bool,
    pub sync_protection_status: bool,
    pub sync_scan_summaries: bool,
    pub sync_policy_settings: bool,
    pub sync_false_positive_reports: bool,
    pub upload_personal_files: bool,
}

impl Default for CloudSyncSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            account_id: None,
            sync_license_status: true,
            sync_device_name: true,
            sync_protection_status: true,
            sync_scan_summaries: true,
            sync_policy_settings: true,
            sync_false_positive_reports: true,
            upload_personal_files: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CloudSyncPayload {
    pub license_status: Option<String>,
    pub device_name: Option<String>,
    pub protection_status: Option<String>,
    pub scan_summary: Option<String>,
    pub policy_profile: Option<String>,
    pub false_positive_report_ids: Vec<String>,
    pub contains_personal_file_contents: bool,
}

impl CloudSyncPayload {
    pub fn validate_privacy(&self) -> Vec<String> {
        if self.contains_personal_file_contents {
            vec![
                "cloud sync payload must not contain personal file contents by default".to_string(),
            ]
        } else {
            Vec::new()
        }
    }
}
