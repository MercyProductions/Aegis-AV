use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum RiskSignalKind {
    RiskyApp,
    VulnerableSoftware,
    SuspiciousBehavior,
    RepeatedIncident,
    StartupChange,
    NetworkAnomaly,
    SecurityHygiene,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RiskSignal {
    pub kind: RiskSignalKind,
    pub weight: u8,
    pub description: String,
    pub observed_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RiskTrend {
    Falling,
    Stable,
    Rising,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PredictiveRiskAssessment {
    pub score: u8,
    pub trend: RiskTrend,
    pub patterns: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct PredictiveRiskEngine;

impl PredictiveRiskEngine {
    pub fn assess(signals: &[RiskSignal]) -> PredictiveRiskAssessment {
        let score = signals
            .iter()
            .map(|signal| signal.weight as u16)
            .sum::<u16>()
            .min(100) as u8;
        let trend = if signals
            .iter()
            .any(|signal| signal.kind == RiskSignalKind::RepeatedIncident)
            || score >= 70
        {
            RiskTrend::Rising
        } else if score <= 20 {
            RiskTrend::Falling
        } else {
            RiskTrend::Stable
        };
        let patterns = signals
            .iter()
            .map(|signal| signal.description.clone())
            .collect::<Vec<_>>();
        let mut recommendations = Vec::new();
        if score >= 60 {
            recommendations
                .push("Review recent startup, process, and network changes.".to_string());
        }
        if signals
            .iter()
            .any(|signal| signal.kind == RiskSignalKind::VulnerableSoftware)
        {
            recommendations
                .push("Update vulnerable applications before increasing automation.".to_string());
        }
        if recommendations.is_empty() {
            recommendations.push("Maintain current protection posture.".to_string());
        }

        PredictiveRiskAssessment {
            score,
            trend,
            patterns,
            recommendations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_incidents_raise_risk_trend() {
        let signals = vec![RiskSignal {
            kind: RiskSignalKind::RepeatedIncident,
            weight: 40,
            description: "Three suspicious launches from Downloads this week.".to_string(),
            observed_at: "2026-05-18T12:00:00Z".to_string(),
        }];

        let assessment = PredictiveRiskEngine::assess(&signals);

        assert_eq!(assessment.trend, RiskTrend::Rising);
        assert_eq!(assessment.score, 40);
    }
}
