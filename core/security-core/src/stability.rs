use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StabilitySignal {
    CrashDetected,
    WatchdogMissedHeartbeat,
    CorruptionDetected,
    UpdateFailed,
    MemoryGrowth,
    DeadlockSuspected,
    LongRuntimeStressFailure,
    ServiceStopped,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAction {
    RestartService,
    EnterSafeMode,
    RollbackUpdate,
    RepairCorruption,
    CaptureCrashDump,
    ThrottleModule,
    RunStressTest,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum StabilitySeverity {
    Watch,
    Recoverable,
    Degraded,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StabilityAssessment {
    pub signal: StabilitySignal,
    pub severity: StabilitySeverity,
    pub recommended_actions: Vec<RecoveryAction>,
    pub user_visible: bool,
    pub explanation: String,
}

pub struct StabilityEngine;

impl StabilityEngine {
    pub fn assess(signal: StabilitySignal) -> StabilityAssessment {
        let (severity, recommended_actions, user_visible, explanation) = match signal {
            StabilitySignal::CrashDetected => (
                StabilitySeverity::Degraded,
                vec![
                    RecoveryAction::CaptureCrashDump,
                    RecoveryAction::RestartService,
                    RecoveryAction::RunStressTest,
                ],
                true,
                "A protected component crashed and should restart with diagnostics captured.",
            ),
            StabilitySignal::WatchdogMissedHeartbeat => (
                StabilitySeverity::Recoverable,
                vec![RecoveryAction::RestartService],
                true,
                "The watchdog missed a heartbeat and should recover the affected service.",
            ),
            StabilitySignal::CorruptionDetected => (
                StabilitySeverity::Critical,
                vec![
                    RecoveryAction::EnterSafeMode,
                    RecoveryAction::RepairCorruption,
                    RecoveryAction::RollbackUpdate,
                ],
                true,
                "Local state or signatures failed integrity checks and require recovery.",
            ),
            StabilitySignal::UpdateFailed => (
                StabilitySeverity::Degraded,
                vec![
                    RecoveryAction::RollbackUpdate,
                    RecoveryAction::EnterSafeMode,
                ],
                true,
                "An update failed verification or application and should roll back safely.",
            ),
            StabilitySignal::MemoryGrowth => (
                StabilitySeverity::Watch,
                vec![
                    RecoveryAction::ThrottleModule,
                    RecoveryAction::RunStressTest,
                ],
                false,
                "Memory growth crossed a profiling threshold and should be investigated.",
            ),
            StabilitySignal::DeadlockSuspected => (
                StabilitySeverity::Degraded,
                vec![
                    RecoveryAction::CaptureCrashDump,
                    RecoveryAction::RestartService,
                ],
                true,
                "A module stopped making progress and should be recovered with a dump.",
            ),
            StabilitySignal::LongRuntimeStressFailure => (
                StabilitySeverity::Recoverable,
                vec![
                    RecoveryAction::RunStressTest,
                    RecoveryAction::ThrottleModule,
                ],
                false,
                "Stress testing exposed a reliability regression for engineering review.",
            ),
            StabilitySignal::ServiceStopped => (
                StabilitySeverity::Degraded,
                vec![RecoveryAction::RestartService],
                true,
                "The endpoint agent service stopped unexpectedly.",
            ),
        };

        StabilityAssessment {
            signal,
            severity,
            recommended_actions,
            user_visible,
            explanation: explanation.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_failure_recommends_safe_rollback() {
        let assessment = StabilityEngine::assess(StabilitySignal::UpdateFailed);

        assert!(assessment
            .recommended_actions
            .contains(&RecoveryAction::RollbackUpdate));
        assert!(assessment
            .recommended_actions
            .contains(&RecoveryAction::EnterSafeMode));
        assert!(assessment.user_visible);
    }
}
