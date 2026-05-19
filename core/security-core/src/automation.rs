use crate::event_bus::{AegisEvent, AegisEventKind, EventSeverity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowCondition {
    RansomwareScoreGreaterThan(u8),
    UnknownExecutableDetected,
    EventKind(AegisEventKind),
    SeverityAtLeast(EventSeverity),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WorkflowAction {
    IsolateProcessPendingConfirmation,
    QuarantineFile,
    CreateIncidentReport,
    NotifyUser,
    UploadMetadataOnly,
    RequestCloudReputation,
    PromptUser,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AutomationWorkflow {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub conditions: Vec<WorkflowCondition>,
    pub actions: Vec<WorkflowAction>,
}

#[derive(Debug, Default, Clone)]
pub struct AutomationEngine {
    workflows: Vec<AutomationWorkflow>,
}

impl AutomationEngine {
    pub fn new(workflows: Vec<AutomationWorkflow>) -> Self {
        Self { workflows }
    }

    pub fn workflows(&self) -> &[AutomationWorkflow] {
        &self.workflows
    }

    pub fn evaluate(&self, event: &AegisEvent) -> Vec<WorkflowAction> {
        self.workflows
            .iter()
            .filter(|workflow| workflow.enabled)
            .filter(|workflow| {
                workflow
                    .conditions
                    .iter()
                    .all(|condition| matches_event(condition, event))
            })
            .flat_map(|workflow| workflow.actions.iter().copied())
            .filter(is_safe_action)
            .collect()
    }
}

fn matches_event(condition: &WorkflowCondition, event: &AegisEvent) -> bool {
    match condition {
        WorkflowCondition::RansomwareScoreGreaterThan(threshold) => event
            .details
            .get("ransomware_score")
            .and_then(|score| score.parse::<u8>().ok())
            .is_some_and(|score| score > *threshold),
        WorkflowCondition::UnknownExecutableDetected => event
            .details
            .get("file_reputation")
            .is_some_and(|reputation| reputation == "unknown_executable"),
        WorkflowCondition::EventKind(kind) => event.kind == *kind,
        WorkflowCondition::SeverityAtLeast(severity) => event.severity >= *severity,
    }
}

fn is_safe_action(action: &WorkflowAction) -> bool {
    matches!(
        action,
        WorkflowAction::IsolateProcessPendingConfirmation
            | WorkflowAction::QuarantineFile
            | WorkflowAction::CreateIncidentReport
            | WorkflowAction::NotifyUser
            | WorkflowAction::UploadMetadataOnly
            | WorkflowAction::RequestCloudReputation
            | WorkflowAction::PromptUser
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn high_ransomware_event_triggers_safe_response_plan() {
        let workflow = AutomationWorkflow {
            id: "wf_ransomware_high".to_string(),
            name: "High ransomware score".to_string(),
            enabled: true,
            conditions: vec![WorkflowCondition::RansomwareScoreGreaterThan(80)],
            actions: vec![
                WorkflowAction::IsolateProcessPendingConfirmation,
                WorkflowAction::QuarantineFile,
                WorkflowAction::CreateIncidentReport,
                WorkflowAction::NotifyUser,
            ],
        };
        let engine = AutomationEngine::new(vec![workflow]);
        let mut details = BTreeMap::new();
        details.insert("ransomware_score".to_string(), "91".to_string());
        let event = AegisEvent {
            id: "evt_2".to_string(),
            time: "2026-05-18T12:00:00Z".to_string(),
            kind: AegisEventKind::BehaviorTriggered,
            source_module: "Aegis.Ransomware".to_string(),
            severity: EventSeverity::Critical,
            summary: "Mass rename activity detected.".to_string(),
            details,
        };

        let actions = engine.evaluate(&event);

        assert_eq!(actions.len(), 4);
        assert!(actions.contains(&WorkflowAction::CreateIncidentReport));
    }
}
