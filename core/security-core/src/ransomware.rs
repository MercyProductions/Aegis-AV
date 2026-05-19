use crate::behavior::{assessment_from_matches, BehaviorAssessment, BehaviorRuleMatch};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "event_type")]
pub enum ProtectedFolderEvent {
    FileRenamed {
        old_path: PathBuf,
        new_path: PathBuf,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
    FileWritten {
        path: PathBuf,
        process_id: Option<u32>,
        entropy_hint: Option<u8>,
        timestamp_ms: u64,
    },
    FileDeleted {
        path: PathBuf,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
}

#[derive(Debug, Clone)]
pub struct RansomwareMonitor {
    window_ms: u64,
    rename_threshold: usize,
    delete_threshold: usize,
    write_threshold: usize,
    renames: HashMap<Option<u32>, VecDeque<u64>>,
    deletes: HashMap<Option<u32>, VecDeque<u64>>,
    writes: HashMap<Option<u32>, VecDeque<(u64, Option<u8>)>>,
}

impl Default for RansomwareMonitor {
    fn default() -> Self {
        Self {
            window_ms: 10_000,
            rename_threshold: 12,
            delete_threshold: 15,
            write_threshold: 20,
            renames: HashMap::new(),
            deletes: HashMap::new(),
            writes: HashMap::new(),
        }
    }
}

impl RansomwareMonitor {
    pub fn assess_event(&mut self, event: ProtectedFolderEvent) -> BehaviorAssessment {
        let mut matches = Vec::new();

        match event {
            ProtectedFolderEvent::FileRenamed {
                old_path,
                new_path,
                process_id,
                timestamp_ms,
            } => {
                let count =
                    record_timestamp(&mut self.renames, process_id, timestamp_ms, self.window_ms);
                if extension_changed(&old_path, &new_path) {
                    matches.push(BehaviorRuleMatch {
                        rule_id: "RANSOM-EXTENSION-CHANGE".to_string(),
                        description: "A protected file was renamed with a different extension."
                            .to_string(),
                        score: 20,
                        evidence: vec![
                            old_path.display().to_string(),
                            new_path.display().to_string(),
                        ],
                    });
                }
                if count >= self.rename_threshold {
                    matches.push(BehaviorRuleMatch {
                        rule_id: "RANSOM-RAPID-RENAMES".to_string(),
                        description: "Rapid protected-folder rename activity was observed."
                            .to_string(),
                        score: 45,
                        evidence: vec![format!("{count} renames in {} ms", self.window_ms)],
                    });
                }
            }
            ProtectedFolderEvent::FileWritten {
                path,
                process_id,
                entropy_hint,
                timestamp_ms,
            } => {
                let count = record_write(
                    &mut self.writes,
                    process_id,
                    timestamp_ms,
                    entropy_hint,
                    self.window_ms,
                );
                if entropy_hint.unwrap_or_default() >= 85 {
                    matches.push(BehaviorRuleMatch {
                        rule_id: "RANSOM-HIGH-ENTROPY-WRITE".to_string(),
                        description:
                            "A protected file write looked encryption-like by entropy hint."
                                .to_string(),
                        score: 35,
                        evidence: vec![path.display().to_string()],
                    });
                }
                if count >= self.write_threshold {
                    matches.push(BehaviorRuleMatch {
                        rule_id: "RANSOM-MASS-WRITES".to_string(),
                        description: "Many protected files were written in a short time."
                            .to_string(),
                        score: 45,
                        evidence: vec![format!("{count} writes in {} ms", self.window_ms)],
                    });
                }
            }
            ProtectedFolderEvent::FileDeleted {
                path,
                process_id,
                timestamp_ms,
            } => {
                let count =
                    record_timestamp(&mut self.deletes, process_id, timestamp_ms, self.window_ms);
                if count >= self.delete_threshold {
                    matches.push(BehaviorRuleMatch {
                        rule_id: "RANSOM-DELETION-BURST".to_string(),
                        description: "Many protected files were deleted in a short time."
                            .to_string(),
                        score: 45,
                        evidence: vec![
                            format!("{count} deletes in {} ms", self.window_ms),
                            path.display().to_string(),
                        ],
                    });
                }
            }
        }

        assessment_from_matches(matches)
    }
}

fn record_timestamp(
    buckets: &mut HashMap<Option<u32>, VecDeque<u64>>,
    process_id: Option<u32>,
    timestamp_ms: u64,
    window_ms: u64,
) -> usize {
    let bucket = buckets.entry(process_id).or_default();
    bucket.push_back(timestamp_ms);
    while let Some(seen_at) = bucket.front() {
        if timestamp_ms.saturating_sub(*seen_at) <= window_ms {
            break;
        }
        bucket.pop_front();
    }
    bucket.len()
}

fn record_write(
    buckets: &mut HashMap<Option<u32>, VecDeque<(u64, Option<u8>)>>,
    process_id: Option<u32>,
    timestamp_ms: u64,
    entropy_hint: Option<u8>,
    window_ms: u64,
) -> usize {
    let bucket = buckets.entry(process_id).or_default();
    bucket.push_back((timestamp_ms, entropy_hint));
    while let Some((seen_at, _)) = bucket.front() {
        if timestamp_ms.saturating_sub(*seen_at) <= window_ms {
            break;
        }
        bucket.pop_front();
    }
    bucket.len()
}

fn extension_changed(old_path: &Path, new_path: &Path) -> bool {
    old_path.extension().and_then(|ext| ext.to_str())
        != new_path.extension().and_then(|ext| ext.to_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::behavior::RiskLevel;

    #[test]
    fn mass_renames_raise_risk() {
        let mut monitor = RansomwareMonitor::default();
        let mut assessment = monitor.assess_event(ProtectedFolderEvent::FileRenamed {
            old_path: PathBuf::from("a.docx"),
            new_path: PathBuf::from("a.locked"),
            process_id: Some(9),
            timestamp_ms: 0,
        });

        for i in 1..12 {
            assessment = monitor.assess_event(ProtectedFolderEvent::FileRenamed {
                old_path: PathBuf::from(format!("{i}.docx")),
                new_path: PathBuf::from(format!("{i}.locked")),
                process_id: Some(9),
                timestamp_ms: i as u64,
            });
        }

        assert!(assessment.risk_level >= RiskLevel::High);
    }
}
