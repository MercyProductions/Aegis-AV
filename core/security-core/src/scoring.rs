use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityScoreBreakdown {
    pub protection: ScoreCategory,
    pub updates: ScoreCategory,
    pub system_health: ScoreCategory,
    pub startup_risk: ScoreCategory,
    pub network_risk: ScoreCategory,
    pub user_configuration: ScoreCategory,
    pub threat_history: ScoreCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScoreCategory {
    pub name: String,
    pub score: u8,
    pub deductions: Vec<String>,
}

pub struct SecurityScoreEngine;

impl SecurityScoreEngine {
    pub fn overall_score(breakdown: &SecurityScoreBreakdown) -> u8 {
        let categories = [
            &breakdown.protection,
            &breakdown.updates,
            &breakdown.system_health,
            &breakdown.startup_risk,
            &breakdown.network_risk,
            &breakdown.user_configuration,
            &breakdown.threat_history,
        ];
        let total: u16 = categories
            .iter()
            .map(|category| category.score as u16)
            .sum();
        (total / categories.len() as u16).min(100) as u8
    }

    pub fn deductions(breakdown: &SecurityScoreBreakdown) -> Vec<String> {
        [
            &breakdown.protection,
            &breakdown.updates,
            &breakdown.system_health,
            &breakdown.startup_risk,
            &breakdown.network_risk,
            &breakdown.user_configuration,
            &breakdown.threat_history,
        ]
        .into_iter()
        .flat_map(|category| {
            category
                .deductions
                .iter()
                .map(|deduction| format!("{}: {}", category.name, deduction))
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_explains_deductions() {
        let breakdown = SecurityScoreBreakdown {
            protection: category("Protection", 96, vec![]),
            updates: category("Updates", 90, vec!["Beta channel pending restart"]),
            system_health: category("System Health", 92, vec![]),
            startup_risk: category("Startup Risk", 86, vec!["One unknown startup entry"]),
            network_risk: category("Network Risk", 94, vec![]),
            user_configuration: category("User Configuration", 90, vec![]),
            threat_history: category("Threat History", 96, vec![]),
        };

        assert_eq!(SecurityScoreEngine::overall_score(&breakdown), 92);
        assert_eq!(SecurityScoreEngine::deductions(&breakdown).len(), 2);
    }

    fn category(name: &str, score: u8, deductions: Vec<&str>) -> ScoreCategory {
        ScoreCategory {
            name: name.to_string(),
            score,
            deductions: deductions.into_iter().map(str::to_string).collect(),
        }
    }
}
