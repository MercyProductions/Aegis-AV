use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PremiumExperienceArea {
    Installer,
    Onboarding,
    Animations,
    Sounds,
    Transitions,
    Dashboards,
    Website,
    Documentation,
    EcosystemConsistency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PremiumExperiencePillar {
    pub area: PremiumExperienceArea,
    pub status: String,
    pub principle: String,
}

pub struct PremiumExperiencePlan;

impl PremiumExperiencePlan {
    pub fn defaults() -> Vec<PremiumExperiencePillar> {
        vec![
            pillar(
                PremiumExperienceArea::Installer,
                "planned",
                "clear, signed, easy to uninstall",
            ),
            pillar(
                PremiumExperienceArea::Onboarding,
                "planned",
                "fast setup with transparent permissions",
            ),
            pillar(
                PremiumExperienceArea::Animations,
                "active",
                "subtle, responsive, and purposeful",
            ),
            pillar(
                PremiumExperienceArea::Dashboards,
                "active",
                "dense command surfaces without clutter",
            ),
            pillar(
                PremiumExperienceArea::Documentation,
                "active",
                "architecture-first and privacy-clear",
            ),
            pillar(
                PremiumExperienceArea::EcosystemConsistency,
                "active",
                "shared language across Aegis products",
            ),
        ]
    }
}

fn pillar(area: PremiumExperienceArea, status: &str, principle: &str) -> PremiumExperiencePillar {
    PremiumExperiencePillar {
        area,
        status: status.to_string(),
        principle: principle.to_string(),
    }
}
