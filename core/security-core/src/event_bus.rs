use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum AegisEventKind {
    ThreatDetected,
    ScanStarted,
    ScanFinished,
    BehaviorTriggered,
    ConnectionBlocked,
    FileQuarantined,
    PolicyChanged,
    UpdateInstalled,
    NotificationRaised,
    AutomationExecuted,
    ServiceRecovered,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum EventSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AegisEvent {
    pub id: String,
    pub time: String,
    pub kind: AegisEventKind,
    pub source_module: String,
    pub severity: EventSeverity,
    pub summary: String,
    pub details: BTreeMap<String, String>,
}

#[derive(Debug, Default, Clone)]
pub struct EventBus {
    events: Vec<AegisEvent>,
}

impl EventBus {
    pub fn emit(&mut self, event: AegisEvent) {
        self.events.push(event);
    }

    pub fn events(&self) -> &[AegisEvent] {
        &self.events
    }

    pub fn by_kind(&self, kind: AegisEventKind) -> Vec<&AegisEvent> {
        self.events
            .iter()
            .filter(|event| event.kind == kind)
            .collect()
    }

    pub fn drain(&mut self) -> Vec<AegisEvent> {
        std::mem::take(&mut self.events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_bus_indexes_by_kind() {
        let mut bus = EventBus::default();
        bus.emit(AegisEvent {
            id: "evt_1".to_string(),
            time: "2026-05-18T12:00:00Z".to_string(),
            kind: AegisEventKind::ThreatDetected,
            source_module: "Aegis.Scanner".to_string(),
            severity: EventSeverity::High,
            summary: "Safe EICAR test signature matched.".to_string(),
            details: BTreeMap::new(),
        });

        assert_eq!(bus.by_kind(AegisEventKind::ThreatDetected).len(), 1);
        assert!(bus.by_kind(AegisEventKind::ScanFinished).is_empty());
    }
}
