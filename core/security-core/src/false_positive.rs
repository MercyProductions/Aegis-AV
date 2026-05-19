use crate::reputation::{FileReputation, ReputationDatabase, ReputationVerdict};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FalsePositiveReport {
    pub report_id: String,
    pub detection_name: String,
    pub sha256: String,
    pub file_path: Option<PathBuf>,
    pub confidence_score: u8,
    pub reason_breakdown: Vec<String>,
    pub user_notes: String,
    pub created_at: String,
    pub include_file_upload: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrustAction {
    MarkAsTrusted,
    RestoreAndExclude,
    SubmitFalsePositiveReport,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AllowList {
    pub trusted_hashes: Vec<String>,
    pub trusted_paths: Vec<PathBuf>,
    pub user_notes: Vec<String>,
}

impl AllowList {
    pub fn trust_hash(&mut self, sha256: impl Into<String>, note: impl Into<String>) {
        let sha256 = sha256.into().to_ascii_lowercase();
        if !self.trusted_hashes.contains(&sha256) {
            self.trusted_hashes.push(sha256);
        }
        self.user_notes.push(note.into());
    }

    pub fn restore_and_exclude(
        &mut self,
        sha256: impl Into<String>,
        path: PathBuf,
        note: impl Into<String>,
    ) {
        self.trust_hash(sha256, note);
        if !self.trusted_paths.contains(&path) {
            self.trusted_paths.push(path);
        }
    }

    pub fn apply_to_reputation(
        &self,
        reputation_db: &mut ReputationDatabase,
        sha256: &str,
        now_unix_seconds: u64,
    ) {
        if self.trusted_hashes.contains(&sha256.to_ascii_lowercase()) {
            reputation_db.upsert_file(FileReputation {
                sha256: sha256.to_ascii_lowercase(),
                verdict: ReputationVerdict::Trusted,
                vendor: None,
                first_seen_unix_seconds: Some(now_unix_seconds),
                last_seen_unix_seconds: Some(now_unix_seconds),
                user_approved: true,
                notes: Some("User allowlist".to_string()),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_and_exclude_adds_hash_and_path() {
        let mut allowlist = AllowList::default();
        allowlist.restore_and_exclude("ABC", PathBuf::from("C:/safe.exe"), "approved");

        assert!(allowlist.trusted_hashes.contains(&"abc".to_string()));
        assert_eq!(allowlist.trusted_paths.len(), 1);
    }
}
