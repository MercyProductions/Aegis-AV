use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransparencyControl {
    pub name: String,
    pub enabled: bool,
    pub user_visible: bool,
    pub description: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransparencyLedger {
    pub controls: Vec<TransparencyControl>,
}

impl TransparencyLedger {
    pub fn score(&self) -> u8 {
        if self.controls.is_empty() {
            return 0;
        }
        let visible_enabled = self
            .controls
            .iter()
            .filter(|control| control.enabled && control.user_visible)
            .count() as u16;
        ((visible_enabled * 100) / self.controls.len() as u16) as u8
    }
}
