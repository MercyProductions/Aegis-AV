use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuarantineRecord {
    pub quarantine_id: String,
    pub original_path: PathBuf,
    pub stored_path: PathBuf,
    pub sha256: String,
    pub detection: String,
    pub date: String,
    pub size_bytes: Option<u64>,
    pub original_extension: Option<String>,
    pub preview: QuarantinePreview,
    pub restore_policy: RestorePolicy,
    pub cleanup_after_days: Option<u32>,
    pub exported_report_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuarantinePreview {
    pub file_name: Option<String>,
    pub mime_guess: Option<String>,
    pub detection_summary: String,
    pub recommended_action: String,
}

impl Default for QuarantinePreview {
    fn default() -> Self {
        Self {
            file_name: None,
            mime_guess: None,
            detection_summary: "No preview metadata available.".to_string(),
            recommended_action: "quarantine".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestorePolicy {
    pub validate_original_parent_exists: bool,
    pub prevent_overwrite: bool,
    pub allow_restore_and_exclude: bool,
    pub require_user_confirmation: bool,
}

impl Default for RestorePolicy {
    fn default() -> Self {
        Self {
            validate_original_parent_exists: true,
            prevent_overwrite: true,
            allow_restore_and_exclude: true,
            require_user_confirmation: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreValidation {
    pub allowed: bool,
    pub target_path: PathBuf,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentReport {
    pub quarantine_id: String,
    pub original_path: PathBuf,
    pub sha256: String,
    pub detection: String,
    pub quarantined_at: String,
    pub recommended_action: String,
    pub report_version: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct QuarantineIndex {
    records_by_hash: HashMap<String, QuarantineRecord>,
}

impl QuarantineIndex {
    pub fn insert(&mut self, record: QuarantineRecord) -> Option<QuarantineRecord> {
        self.records_by_hash
            .insert(record.sha256.to_ascii_lowercase(), record)
    }

    pub fn has_duplicate_hash(&self, sha256: &str) -> bool {
        self.records_by_hash
            .contains_key(&sha256.to_ascii_lowercase())
    }

    pub fn incident_report_for(&self, sha256: &str) -> Option<IncidentReport> {
        self.records_by_hash
            .get(&sha256.to_ascii_lowercase())
            .map(|record| IncidentReport {
                quarantine_id: record.quarantine_id.clone(),
                original_path: record.original_path.clone(),
                sha256: record.sha256.clone(),
                detection: record.detection.clone(),
                quarantined_at: record.date.clone(),
                recommended_action: record.preview.recommended_action.clone(),
                report_version: 1,
            })
    }
}

pub fn validate_restore_target(
    record: &QuarantineRecord,
    requested_target: Option<PathBuf>,
) -> RestoreValidation {
    let target_path = requested_target.unwrap_or_else(|| record.original_path.clone());
    let mut reasons = Vec::new();

    if record.restore_policy.validate_original_parent_exists {
        match target_path.parent() {
            Some(parent) if parent.exists() => {}
            Some(parent) => reasons.push(format!(
                "restore parent does not exist: {}",
                parent.display()
            )),
            None => reasons.push("restore target has no parent directory".to_string()),
        }
    }

    if record.restore_policy.prevent_overwrite && target_path.exists() {
        reasons.push(format!(
            "restore target already exists: {}",
            target_path.display()
        ));
    }

    RestoreValidation {
        allowed: reasons.is_empty(),
        target_path,
        reasons,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_prevents_duplicate_hashes() {
        let mut index = QuarantineIndex::default();
        index.insert(record("ABC"));

        assert!(index.has_duplicate_hash("abc"));
        assert!(index.incident_report_for("abc").is_some());
    }

    #[test]
    fn restore_validation_rejects_missing_parent() {
        let mut record = record("DEF");
        record.original_path = PathBuf::from("Z:/definitely-missing/file.exe");

        let validation = validate_restore_target(&record, None);

        assert!(!validation.allowed);
    }

    fn record(hash: &str) -> QuarantineRecord {
        QuarantineRecord {
            quarantine_id: "q_test".to_string(),
            original_path: PathBuf::from("C:/Users/User/Downloads/file.exe"),
            stored_path: PathBuf::from("C:/ProgramData/AegisAV/Quarantine/q_test.bin"),
            sha256: hash.to_string(),
            detection: "Test.Detection".to_string(),
            date: "2026-05-18T12:00:00Z".to_string(),
            size_bytes: Some(10),
            original_extension: Some("exe".to_string()),
            preview: QuarantinePreview::default(),
            restore_policy: RestorePolicy::default(),
            cleanup_after_days: Some(30),
            exported_report_path: None,
        }
    }
}
