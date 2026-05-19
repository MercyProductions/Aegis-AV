use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiAnalysisRequest {
    pub context_type: AiContextType,
    pub facts: Vec<String>,
    pub confidence_score: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AiContextType {
    DetectionExplanation,
    ThreatClassification,
    BehaviorSummary,
    LogSummary,
    IncidentExplanation,
    UserRecommendation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiAnalysisResponse {
    pub explanation: String,
    pub classification: Option<String>,
    pub recommendations: Vec<AiRecommendation>,
    pub may_block_automatically: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiRecommendation {
    pub title: String,
    pub rationale: String,
    pub user_visible: bool,
}

impl AiAnalysisResponse {
    pub fn explain_only(
        explanation: impl Into<String>,
        recommendations: Vec<AiRecommendation>,
    ) -> Self {
        Self {
            explanation: explanation.into(),
            classification: None,
            recommendations,
            may_block_automatically: false,
        }
    }
}
