use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SecurityLogEvent {
    pub time: String,
    pub level: LogLevel,
    pub event: String,
    pub details: Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl SecurityLogEvent {
    pub fn new(
        time: impl Into<String>,
        level: LogLevel,
        event: impl Into<String>,
        details: Value,
    ) -> Self {
        Self {
            time: time.into(),
            level,
            event: event.into(),
            details,
        }
    }
}
