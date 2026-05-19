use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignatureRecord {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub signature_type: String,
    pub severity: String,
    pub description: String,
    pub recommended_action: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Default)]
pub struct SignatureDb {
    by_sha256: HashMap<String, SignatureRecord>,
}

impl SignatureDb {
    pub fn load_json(path: &Path) -> anyhow::Result<Self> {
        let data = fs::read_to_string(path)?;
        let records: Vec<SignatureRecord> = serde_json::from_str(&data)?;
        Ok(Self::from_records(records))
    }

    pub fn from_records(records: Vec<SignatureRecord>) -> Self {
        let by_sha256 = records
            .into_iter()
            .map(|record| (record.sha256.to_ascii_lowercase(), record))
            .collect();

        Self { by_sha256 }
    }

    pub fn find_sha256(&self, sha256: &str) -> Option<&SignatureRecord> {
        self.by_sha256.get(&sha256.to_ascii_lowercase())
    }

    pub fn len(&self) -> usize {
        self.by_sha256.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_sha256.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_sha256_case_insensitively() {
        let record = SignatureRecord {
            id: "AEGIS-TEST".to_string(),
            name: "Test.Signature".to_string(),
            signature_type: "test".to_string(),
            severity: "high".to_string(),
            description: "test".to_string(),
            recommended_action: "quarantine".to_string(),
            sha256: "ABC123".to_string(),
        };
        let db = SignatureDb::from_records(vec![record]);

        assert!(db.find_sha256("abc123").is_some());
    }
}
