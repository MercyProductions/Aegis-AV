use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatKnowledgeEntry {
    pub detection_name: String,
    pub explanation: String,
    pub severity: String,
    pub risk: String,
    pub behavior: Vec<String>,
    pub remediation: Vec<String>,
    pub prevention_tips: Vec<String>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatKnowledgeBase {
    entries: BTreeMap<String, ThreatKnowledgeEntry>,
}

impl ThreatKnowledgeBase {
    pub fn insert(&mut self, entry: ThreatKnowledgeEntry) {
        self.entries.insert(entry.detection_name.clone(), entry);
    }

    pub fn get(&self, detection_name: &str) -> Option<&ThreatKnowledgeEntry> {
        self.entries.get(detection_name)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
