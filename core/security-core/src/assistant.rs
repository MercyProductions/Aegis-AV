use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssistantPromptKind {
    ExplainDetection,
    SummarizeIncident,
    ExplainSetting,
    RecommendAction,
    ExplainLog,
    Troubleshoot,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssistantRequest {
    pub kind: AssistantPromptKind,
    pub subject: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssistantResponse {
    pub summary: String,
    pub confidence: u8,
    pub recommended_actions: Vec<String>,
    pub safety_note: String,
}

pub struct AegisAssistant;

impl AegisAssistant {
    pub fn explain(request: &AssistantRequest) -> AssistantResponse {
        let evidence_summary = if request.evidence.is_empty() {
            "No detailed evidence was attached.".to_string()
        } else {
            request.evidence.join("; ")
        };
        let summary = match request.kind {
            AssistantPromptKind::ExplainDetection => {
                format!(
                    "{} was flagged because: {evidence_summary}",
                    request.subject
                )
            }
            AssistantPromptKind::SummarizeIncident => {
                format!(
                    "Incident {} contains this observed sequence: {evidence_summary}",
                    request.subject
                )
            }
            AssistantPromptKind::ExplainSetting => {
                format!(
                    "{} changes how Aegis balances protection, noise, and performance.",
                    request.subject
                )
            }
            AssistantPromptKind::RecommendAction => {
                format!(
                    "Aegis recommends reviewing {} using the available evidence.",
                    request.subject
                )
            }
            AssistantPromptKind::ExplainLog => {
                format!(
                    "The log entry for {} records: {evidence_summary}",
                    request.subject
                )
            }
            AssistantPromptKind::Troubleshoot => {
                format!(
                    "Troubleshooting {} should start with integrity, service, and update status.",
                    request.subject
                )
            }
        };

        AssistantResponse {
            summary,
            confidence: 80,
            recommended_actions: vec![
                "Review detection evidence.".to_string(),
                "Keep quarantine until the file is trusted.".to_string(),
                "Use restore and exclude only for known-safe files.".to_string(),
            ],
            safety_note: "AI explanations are advisory and never block files by themselves."
                .to_string(),
        }
    }
}
