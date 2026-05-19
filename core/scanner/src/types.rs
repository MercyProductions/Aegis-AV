use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanProfile {
    Quick,
    Full,
    Deep,
    Custom,
}

impl FromStr for ScanProfile {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "quick" => Ok(Self::Quick),
            "full" => Ok(Self::Full),
            "deep" => Ok(Self::Deep),
            "custom" => Ok(Self::Custom),
            other => Err(format!("unknown scan profile: {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanVerdict {
    Clean,
    Suspicious,
    Malicious,
    Skipped,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileMetadata {
    pub size_bytes: u64,
    pub extension: Option<String>,
    pub mime_guess: Option<String>,
    pub modified_unix_seconds: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeuristicFinding {
    pub rule_id: String,
    pub description: String,
    pub score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Detection {
    pub id: String,
    pub name: String,
    pub severity: String,
    pub description: String,
    pub recommended_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanResult {
    pub path: PathBuf,
    pub verdict: ScanVerdict,
    pub detection_name: Option<String>,
    pub confidence_score: u8,
    pub matched_rule: Option<String>,
    pub sha256: Option<String>,
    pub file_metadata: Option<FileMetadata>,
    pub scan_duration_ms: u128,
    pub heuristics: Vec<HeuristicFinding>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScanSummary {
    pub target: PathBuf,
    pub profile: ScanProfile,
    pub files_scanned: usize,
    pub files_skipped: usize,
    pub errors: usize,
    pub threats_found: usize,
    pub suspicious_found: usize,
    pub duration_ms: u128,
    pub results: Vec<ScanResult>,
}
