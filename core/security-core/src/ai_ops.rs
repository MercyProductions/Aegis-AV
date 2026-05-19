use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiOpsCapability {
    ExplainIncident,
    SummarizeLogs,
    RecommendActions,
    GuideTroubleshooting,
    ExplainProcess,
    SummarizeDeviceHealth,
    ExplainSecurityPosture,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiOpsRequest {
    pub capability: AiOpsCapability,
    pub subject: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiOpsResponse {
    pub narrative: String,
    pub next_steps: Vec<String>,
    pub confidence: u8,
    pub requires_user_decision: bool,
}

pub struct AiOperationsLayer;

impl AiOperationsLayer {
    pub fn summarize(request: &AiOpsRequest) -> AiOpsResponse {
        let evidence = if request.evidence.is_empty() {
            "no attached evidence".to_string()
        } else {
            request.evidence.join("; ")
        };
        let narrative = match request.capability {
            AiOpsCapability::ExplainIncident => {
                format!("Incident {} is explained by: {evidence}", request.subject)
            }
            AiOpsCapability::SummarizeLogs => {
                format!("Logs for {} show: {evidence}", request.subject)
            }
            AiOpsCapability::RecommendActions => {
                format!(
                    "Recommended actions for {} should be based on: {evidence}",
                    request.subject
                )
            }
            AiOpsCapability::GuideTroubleshooting => {
                format!("Troubleshooting {} starts with service health, update state, and graph context.", request.subject)
            }
            AiOpsCapability::ExplainProcess => {
                format!("Process {} should be reviewed with its parent, children, path, and network activity.", request.subject)
            }
            AiOpsCapability::SummarizeDeviceHealth => {
                format!(
                    "Device health for {} is summarized from: {evidence}",
                    request.subject
                )
            }
            AiOpsCapability::ExplainSecurityPosture => {
                format!("Security posture for {} reflects protection layers, hygiene, incidents, and updates.", request.subject)
            }
        };

        AiOpsResponse {
            narrative,
            next_steps: vec![
                "Review the visible evidence.".to_string(),
                "Keep disruptive actions under policy or user confirmation.".to_string(),
                "Export an incident report when action is taken.".to_string(),
            ],
            confidence: 82,
            requires_user_decision: true,
        }
    }
}
