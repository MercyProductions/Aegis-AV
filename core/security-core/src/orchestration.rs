use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrchestrationTrigger {
    SuspiciousProcessLaunch,
    RansomwareBehaviorDetected,
    NetworkAnomaly,
    ServiceFailure,
    PolicyChange,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrchestrationStepKind {
    IsolateProcessPendingReview,
    CaptureMetadata,
    ScanRelatedFiles,
    GenerateIncident,
    NotifyUser,
    PauseProcessPendingPolicy,
    ProtectFolders,
    SnapshotAffectedFiles,
    LaunchRecoveryWorkflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrchestrationStep {
    pub kind: OrchestrationStepKind,
    pub requires_confirmation: bool,
    pub emits_event: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrchestrationPlan {
    pub trigger: OrchestrationTrigger,
    pub reviewable: bool,
    pub steps: Vec<OrchestrationStep>,
}

pub struct IntelligentAutomationEngine;

impl IntelligentAutomationEngine {
    pub fn plan_for(trigger: OrchestrationTrigger) -> OrchestrationPlan {
        let steps = match trigger {
            OrchestrationTrigger::SuspiciousProcessLaunch => vec![
                step(OrchestrationStepKind::IsolateProcessPendingReview, true),
                step(OrchestrationStepKind::CaptureMetadata, false),
                step(OrchestrationStepKind::ScanRelatedFiles, false),
                step(OrchestrationStepKind::GenerateIncident, false),
                step(OrchestrationStepKind::NotifyUser, false),
            ],
            OrchestrationTrigger::RansomwareBehaviorDetected => vec![
                step(OrchestrationStepKind::PauseProcessPendingPolicy, true),
                step(OrchestrationStepKind::ProtectFolders, false),
                step(OrchestrationStepKind::SnapshotAffectedFiles, true),
                step(OrchestrationStepKind::LaunchRecoveryWorkflow, true),
                step(OrchestrationStepKind::GenerateIncident, false),
            ],
            OrchestrationTrigger::NetworkAnomaly => vec![
                step(OrchestrationStepKind::CaptureMetadata, false),
                step(OrchestrationStepKind::GenerateIncident, false),
                step(OrchestrationStepKind::NotifyUser, false),
            ],
            OrchestrationTrigger::ServiceFailure => vec![
                step(OrchestrationStepKind::CaptureMetadata, false),
                step(OrchestrationStepKind::LaunchRecoveryWorkflow, false),
                step(OrchestrationStepKind::NotifyUser, false),
            ],
            OrchestrationTrigger::PolicyChange => vec![
                step(OrchestrationStepKind::CaptureMetadata, false),
                step(OrchestrationStepKind::NotifyUser, false),
            ],
        };

        OrchestrationPlan {
            trigger,
            reviewable: true,
            steps,
        }
    }
}

fn step(kind: OrchestrationStepKind, requires_confirmation: bool) -> OrchestrationStep {
    OrchestrationStep {
        kind,
        requires_confirmation,
        emits_event: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suspicious_process_plan_is_reviewable_and_incident_driven() {
        let plan =
            IntelligentAutomationEngine::plan_for(OrchestrationTrigger::SuspiciousProcessLaunch);

        assert!(plan.reviewable);
        assert!(plan
            .steps
            .iter()
            .any(|step| step.kind == OrchestrationStepKind::GenerateIncident));
        assert!(plan.steps[0].requires_confirmation);
    }
}
