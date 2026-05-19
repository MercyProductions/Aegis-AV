use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReputationVerdict {
    Trusted,
    Suspicious,
    Malicious,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileReputation {
    pub sha256: String,
    pub verdict: ReputationVerdict,
    pub vendor: Option<String>,
    pub first_seen_unix_seconds: Option<u64>,
    pub last_seen_unix_seconds: Option<u64>,
    pub user_approved: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReputationDatabase {
    pub known_files: HashMap<String, FileReputation>,
    pub trusted_vendors: Vec<String>,
    pub trusted_directories: Vec<PathBuf>,
    pub previously_scanned_hashes: HashMap<String, u64>,
}

impl Default for ReputationDatabase {
    fn default() -> Self {
        Self {
            known_files: HashMap::new(),
            trusted_vendors: vec!["Microsoft Windows".to_string()],
            trusted_directories: vec![
                PathBuf::from("C:/Windows"),
                PathBuf::from("C:/Program Files"),
                PathBuf::from("C:/Program Files (x86)"),
            ],
            previously_scanned_hashes: HashMap::new(),
        }
    }
}

impl ReputationDatabase {
    pub fn load_or_default(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        let raw = fs::read_to_string(path)
            .with_context(|| format!("failed to read reputation database {}", path.display()))?;
        serde_json::from_str(&raw)
            .with_context(|| format!("failed to parse reputation database {}", path.display()))
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let raw = serde_json::to_string_pretty(self)?;
        fs::write(path, raw)
            .with_context(|| format!("failed to write reputation database {}", path.display()))
    }

    pub fn reputation_for_hash(&self, sha256: &str) -> ReputationVerdict {
        self.known_files
            .get(&sha256.to_ascii_lowercase())
            .map(|entry| entry.verdict)
            .unwrap_or(ReputationVerdict::Unknown)
    }

    pub fn upsert_file(&mut self, reputation: FileReputation) {
        self.known_files
            .insert(reputation.sha256.to_ascii_lowercase(), reputation);
    }

    pub fn mark_scanned(&mut self, sha256: impl Into<String>, timestamp_unix_seconds: u64) {
        self.previously_scanned_hashes
            .insert(sha256.into().to_ascii_lowercase(), timestamp_unix_seconds);
    }

    pub fn is_trusted_directory(&self, path: &Path) -> bool {
        let normalized = path
            .to_string_lossy()
            .replace('\\', "/")
            .to_ascii_lowercase();
        self.trusted_directories.iter().any(|trusted| {
            normalized.starts_with(
                &trusted
                    .to_string_lossy()
                    .replace('\\', "/")
                    .to_ascii_lowercase(),
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persists_reputation_json() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("reputation.json");
        let mut db = ReputationDatabase::default();
        db.upsert_file(FileReputation {
            sha256: "ABC".to_string(),
            verdict: ReputationVerdict::Trusted,
            vendor: Some("Test Vendor".to_string()),
            first_seen_unix_seconds: Some(1),
            last_seen_unix_seconds: Some(2),
            user_approved: true,
            notes: None,
        });

        db.save(&path).expect("save");
        let loaded = ReputationDatabase::load_or_default(&path).expect("load");

        assert_eq!(
            loaded.reputation_for_hash("abc"),
            ReputationVerdict::Trusted
        );
    }
}
