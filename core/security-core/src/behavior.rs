use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseAction {
    LogOnly,
    NotifyUser,
    PauseOrQuarantinePendingConfirmation,
    AutoQuarantineIfEnabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "event_type")]
pub enum BehaviorEvent {
    FileModified {
        path: PathBuf,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
    StartupFolderWrite {
        path: PathBuf,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
    TempExecutableDrop {
        path: PathBuf,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
    ScriptSpawnedShell {
        script_path: PathBuf,
        shell: String,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
    DownloadsExecutableLaunch {
        path: PathBuf,
        process_id: u32,
        timestamp_ms: u64,
    },
    FailedAccess {
        path: PathBuf,
        process_id: Option<u32>,
        timestamp_ms: u64,
    },
    ChildProcess {
        parent_path: PathBuf,
        child_path: PathBuf,
        parent_id: Option<u32>,
        child_id: Option<u32>,
        timestamp_ms: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehaviorRuleMatch {
    pub rule_id: String,
    pub description: String,
    pub score: u8,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehaviorAssessment {
    pub risk_score: u8,
    pub risk_level: RiskLevel,
    pub response_action: ResponseAction,
    pub matches: Vec<BehaviorRuleMatch>,
}

#[derive(Debug, Clone)]
pub struct BehaviorMonitor {
    file_window_ms: u64,
    high_volume_write_threshold: usize,
    failed_access_threshold: usize,
    recent_file_modifications: VecDeque<(u64, Option<u32>, PathBuf)>,
    failed_accesses: HashMap<Option<u32>, VecDeque<u64>>,
}

impl Default for BehaviorMonitor {
    fn default() -> Self {
        Self {
            file_window_ms: 10_000,
            high_volume_write_threshold: 25,
            failed_access_threshold: 8,
            recent_file_modifications: VecDeque::new(),
            failed_accesses: HashMap::new(),
        }
    }
}

impl BehaviorMonitor {
    pub fn assess_event(&mut self, event: BehaviorEvent) -> BehaviorAssessment {
        let mut matches = Vec::new();

        match event {
            BehaviorEvent::FileModified {
                path,
                process_id,
                timestamp_ms,
            } => {
                self.record_file_modification(timestamp_ms, process_id, path);
                if let Some(rule_match) = self.score_many_files_modified(timestamp_ms, process_id) {
                    matches.push(rule_match);
                }
            }
            BehaviorEvent::StartupFolderWrite { path, .. } => {
                matches.push(BehaviorRuleMatch {
                    rule_id: "BEH-STARTUP-WRITE".to_string(),
                    description: "A file was written to a startup folder.".to_string(),
                    score: 35,
                    evidence: vec![path.display().to_string()],
                });
            }
            BehaviorEvent::TempExecutableDrop { path, .. } => {
                matches.push(BehaviorRuleMatch {
                    rule_id: "BEH-TEMP-EXECUTABLE-DROP".to_string(),
                    description: "An executable or script was created in a temporary folder."
                        .to_string(),
                    score: 30,
                    evidence: vec![path.display().to_string()],
                });
            }
            BehaviorEvent::ScriptSpawnedShell {
                script_path, shell, ..
            } => {
                matches.push(BehaviorRuleMatch {
                    rule_id: "BEH-SCRIPT-SPAWNED-SHELL".to_string(),
                    description: "A script spawned PowerShell, cmd, or another command shell."
                        .to_string(),
                    score: 45,
                    evidence: vec![script_path.display().to_string(), shell],
                });
            }
            BehaviorEvent::DownloadsExecutableLaunch { path, .. } => {
                matches.push(BehaviorRuleMatch {
                    rule_id: "BEH-DOWNLOADS-EXEC-LAUNCH".to_string(),
                    description: "An executable launched from Downloads without established trust."
                        .to_string(),
                    score: 40,
                    evidence: vec![path.display().to_string()],
                });
            }
            BehaviorEvent::FailedAccess {
                path,
                process_id,
                timestamp_ms,
            } => {
                self.record_failed_access(timestamp_ms, process_id);
                if let Some(rule_match) =
                    self.score_failed_accesses(timestamp_ms, process_id, &path)
                {
                    matches.push(rule_match);
                }
            }
            BehaviorEvent::ChildProcess {
                parent_path,
                child_path,
                ..
            } => {
                if is_suspicious_child_chain(&parent_path, &child_path) {
                    matches.push(BehaviorRuleMatch {
                        rule_id: "BEH-SUSPICIOUS-CHILD-CHAIN".to_string(),
                        description:
                            "A user-facing or downloaded process launched a command interpreter."
                                .to_string(),
                        score: 40,
                        evidence: vec![
                            parent_path.display().to_string(),
                            child_path.display().to_string(),
                        ],
                    });
                }
            }
        }

        assessment_from_matches(matches)
    }

    fn record_file_modification(
        &mut self,
        timestamp_ms: u64,
        process_id: Option<u32>,
        path: PathBuf,
    ) {
        self.recent_file_modifications
            .push_back((timestamp_ms, process_id, path));
        while let Some((seen_at, _, _)) = self.recent_file_modifications.front() {
            if timestamp_ms.saturating_sub(*seen_at) <= self.file_window_ms {
                break;
            }
            self.recent_file_modifications.pop_front();
        }
    }

    fn score_many_files_modified(
        &self,
        timestamp_ms: u64,
        process_id: Option<u32>,
    ) -> Option<BehaviorRuleMatch> {
        let count = self
            .recent_file_modifications
            .iter()
            .filter(|(seen_at, pid, _)| {
                timestamp_ms.saturating_sub(*seen_at) <= self.file_window_ms && *pid == process_id
            })
            .count();

        (count >= self.high_volume_write_threshold).then(|| BehaviorRuleMatch {
            rule_id: "BEH-MANY-FILES-MODIFIED".to_string(),
            description: "Many files were modified in a short time window.".to_string(),
            score: 55,
            evidence: vec![format!("{count} files in {} ms", self.file_window_ms)],
        })
    }

    fn record_failed_access(&mut self, timestamp_ms: u64, process_id: Option<u32>) {
        let window = self.failed_accesses.entry(process_id).or_default();
        window.push_back(timestamp_ms);
        while let Some(seen_at) = window.front() {
            if timestamp_ms.saturating_sub(*seen_at) <= self.file_window_ms {
                break;
            }
            window.pop_front();
        }
    }

    fn score_failed_accesses(
        &self,
        timestamp_ms: u64,
        process_id: Option<u32>,
        path: &Path,
    ) -> Option<BehaviorRuleMatch> {
        let count = self
            .failed_accesses
            .get(&process_id)
            .map(|window| {
                window
                    .iter()
                    .filter(|seen_at| timestamp_ms.saturating_sub(**seen_at) <= self.file_window_ms)
                    .count()
            })
            .unwrap_or_default();

        (count >= self.failed_access_threshold).then(|| BehaviorRuleMatch {
            rule_id: "BEH-REPEATED-FAILED-ACCESS".to_string(),
            description: "Repeated failed access attempts were observed.".to_string(),
            score: 25,
            evidence: vec![format!("{count} failures"), path.display().to_string()],
        })
    }
}

pub fn assessment_from_matches(matches: Vec<BehaviorRuleMatch>) -> BehaviorAssessment {
    let risk_score = matches
        .iter()
        .map(|rule_match| rule_match.score as u16)
        .sum::<u16>()
        .min(100) as u8;
    let risk_level = match risk_score {
        0..=19 => RiskLevel::Low,
        20..=49 => RiskLevel::Medium,
        50..=79 => RiskLevel::High,
        _ => RiskLevel::Critical,
    };
    let response_action = match risk_level {
        RiskLevel::Low => ResponseAction::LogOnly,
        RiskLevel::Medium => ResponseAction::NotifyUser,
        RiskLevel::High => ResponseAction::PauseOrQuarantinePendingConfirmation,
        RiskLevel::Critical => ResponseAction::AutoQuarantineIfEnabled,
    };

    BehaviorAssessment {
        risk_score,
        risk_level,
        response_action,
        matches,
    }
}

fn is_suspicious_child_chain(parent_path: &Path, child_path: &Path) -> bool {
    let parent = parent_path.to_string_lossy().to_ascii_lowercase();
    let child = child_path.to_string_lossy().to_ascii_lowercase();
    let parent_is_user_app = parent.contains("\\downloads\\")
        || parent.contains("/downloads/")
        || parent.contains("browser")
        || parent.contains("chrome")
        || parent.contains("edge")
        || parent.contains("firefox");
    let child_is_shell = child.ends_with("powershell.exe")
        || child.ends_with("cmd.exe")
        || child.ends_with("pwsh.exe")
        || child.ends_with("wscript.exe")
        || child.ends_with("cscript.exe");

    parent_is_user_app && child_is_shell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn many_file_modifications_become_high_risk() {
        let mut monitor = BehaviorMonitor::default();
        let mut assessment = monitor.assess_event(BehaviorEvent::FileModified {
            path: PathBuf::from("C:/Users/User/Documents/a.txt"),
            process_id: Some(42),
            timestamp_ms: 0,
        });

        for i in 1..25 {
            assessment = monitor.assess_event(BehaviorEvent::FileModified {
                path: PathBuf::from(format!("C:/Users/User/Documents/{i}.txt")),
                process_id: Some(42),
                timestamp_ms: i as u64,
            });
        }

        assert_eq!(assessment.risk_level, RiskLevel::High);
        assert_eq!(
            assessment.response_action,
            ResponseAction::PauseOrQuarantinePendingConfirmation
        );
    }

    #[test]
    fn startup_write_notifies_user() {
        let mut monitor = BehaviorMonitor::default();
        let assessment = monitor.assess_event(BehaviorEvent::StartupFolderWrite {
            path: PathBuf::from("Startup/a.exe"),
            process_id: Some(7),
            timestamp_ms: 100,
        });

        assert_eq!(assessment.risk_level, RiskLevel::Medium);
        assert_eq!(assessment.response_action, ResponseAction::NotifyUser);
    }
}
