use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ScanPriority {
    Low,
    Balanced,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PerformancePolicy {
    pub priority: ScanPriority,
    pub max_parallel_scans: usize,
    pub cpu_limit_percent: Option<u8>,
    pub battery_saver: bool,
    pub skip_unchanged_files: bool,
    pub large_file_limit_bytes: u64,
}

impl Default for PerformancePolicy {
    fn default() -> Self {
        Self {
            priority: ScanPriority::Balanced,
            max_parallel_scans: 2,
            cpu_limit_percent: Some(40),
            battery_saver: true,
            skip_unchanged_files: true,
            large_file_limit_bytes: 256 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashCacheEntry {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified_unix_seconds: Option<u64>,
    pub sha256: String,
    pub scanned_at_unix_seconds: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct HashCache {
    entries: HashMap<PathBuf, HashCacheEntry>,
}

impl HashCache {
    pub fn get_if_unchanged(
        &self,
        path: &Path,
        size_bytes: u64,
        modified_unix_seconds: Option<u64>,
    ) -> Option<&HashCacheEntry> {
        self.entries.get(path).filter(|entry| {
            entry.size_bytes == size_bytes && entry.modified_unix_seconds == modified_unix_seconds
        })
    }

    pub fn upsert(&mut self, entry: HashCacheEntry) {
        self.entries.insert(entry.path.clone(), entry);
    }
}
