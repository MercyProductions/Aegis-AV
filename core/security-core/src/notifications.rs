use crate::event_bus::EventSeverity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NotificationMode {
    Normal,
    Silent,
    Gaming,
    Enterprise,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    Info,
    Advisory,
    Important,
    Critical,
    Digest,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AegisNotification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub level: AlertLevel,
    pub mode: NotificationMode,
    pub source_event_id: Option<String>,
    pub scheduled_for: Option<String>,
}

pub struct NotificationCenter;

impl NotificationCenter {
    pub fn from_event(
        id: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        severity: EventSeverity,
        mode: NotificationMode,
        source_event_id: Option<String>,
    ) -> AegisNotification {
        AegisNotification {
            id: id.into(),
            title: title.into(),
            body: body.into(),
            level: alert_level_for(severity),
            mode,
            source_event_id,
            scheduled_for: None,
        }
    }

    pub fn should_show_now(notification: &AegisNotification) -> bool {
        match notification.mode {
            NotificationMode::Normal => notification.level != AlertLevel::Digest,
            NotificationMode::Silent | NotificationMode::Gaming => {
                notification.level == AlertLevel::Critical
            }
            NotificationMode::Enterprise => matches!(
                notification.level,
                AlertLevel::Important | AlertLevel::Critical
            ),
        }
    }
}

fn alert_level_for(severity: EventSeverity) -> AlertLevel {
    match severity {
        EventSeverity::Info | EventSeverity::Low => AlertLevel::Advisory,
        EventSeverity::Medium | EventSeverity::High => AlertLevel::Important,
        EventSeverity::Critical => AlertLevel::Critical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gaming_mode_only_shows_critical_alerts() {
        let medium = NotificationCenter::from_event(
            "n_1",
            "Behavior notice",
            "A script launched a shell.",
            EventSeverity::Medium,
            NotificationMode::Gaming,
            None,
        );
        let critical = NotificationCenter::from_event(
            "n_2",
            "Critical threat",
            "Ransomware-like behavior detected.",
            EventSeverity::Critical,
            NotificationMode::Gaming,
            None,
        );

        assert!(!NotificationCenter::should_show_now(&medium));
        assert!(NotificationCenter::should_show_now(&critical));
    }
}
