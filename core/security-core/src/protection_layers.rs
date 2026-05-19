use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProtectionLayerId {
    FileDetection = 1,
    HeuristicAnalysis = 2,
    BehavioralMonitoring = 3,
    ProcessMonitoring = 4,
    ScriptMonitoring = 5,
    RansomwareProtection = 6,
    NetworkVisibility = 7,
    SystemIntegrityChecks = 8,
    UserAwareness = 9,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtectionLayer {
    pub id: ProtectionLayerId,
    pub name: String,
    pub active: bool,
    pub health_score: u8,
    pub last_event: Option<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtectionLayerStack {
    pub layers: Vec<ProtectionLayer>,
}

impl Default for ProtectionLayerStack {
    fn default() -> Self {
        use ProtectionLayerId::*;
        Self {
            layers: vec![
                layer(
                    FileDetection,
                    "File Detection",
                    "Hash, YARA, and metadata detection.",
                ),
                layer(
                    HeuristicAnalysis,
                    "Heuristic Analysis",
                    "Explainable suspicious-file scoring.",
                ),
                layer(
                    BehavioralMonitoring,
                    "Behavioral Monitoring",
                    "Activity patterns are scored over time.",
                ),
                layer(
                    ProcessMonitoring,
                    "Process Monitoring",
                    "Process tree and command-line visibility.",
                ),
                layer(
                    ScriptMonitoring,
                    "Script Monitoring",
                    "Script launches and shell chains are inspected.",
                ),
                layer(
                    RansomwareProtection,
                    "Ransomware Protection",
                    "Protected folders detect rename/write bursts.",
                ),
                layer(
                    NetworkVisibility,
                    "Network Visibility",
                    "Connections, ports, DNS, and bandwidth are visible.",
                ),
                layer(
                    SystemIntegrityChecks,
                    "System Integrity",
                    "App, config, and signature hashes are verified.",
                ),
                layer(
                    UserAwareness,
                    "User Awareness",
                    "Decisions explain impact and ask when needed.",
                ),
            ],
        }
    }
}

impl ProtectionLayerStack {
    pub fn active_count(&self) -> usize {
        self.layers.iter().filter(|layer| layer.active).count()
    }

    pub fn overall_health(&self) -> u8 {
        if self.layers.is_empty() {
            return 0;
        }
        let total: u16 = self
            .layers
            .iter()
            .map(|layer| layer.health_score as u16)
            .sum();
        (total / self.layers.len() as u16).min(100) as u8
    }

    pub fn set_layer_active(&mut self, id: ProtectionLayerId, active: bool) -> bool {
        if let Some(layer) = self.layers.iter_mut().find(|layer| layer.id == id) {
            layer.active = active;
            if !active {
                layer.health_score = layer.health_score.min(60);
            }
            true
        } else {
            false
        }
    }
}

fn layer(id: ProtectionLayerId, name: &str, explanation: &str) -> ProtectionLayer {
    ProtectionLayer {
        id,
        name: name.to_string(),
        active: true,
        health_score: 96,
        last_event: None,
        explanation: explanation.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_stack_scores_active_layers() {
        let mut stack = ProtectionLayerStack::default();
        stack.set_layer_active(ProtectionLayerId::NetworkVisibility, false);

        assert_eq!(stack.active_count(), 8);
        assert!(stack.overall_health() < 96);
    }
}
