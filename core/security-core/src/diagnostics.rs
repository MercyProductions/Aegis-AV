use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiagnosticsSnapshot {
    pub captured_at: String,
    pub startup_entries: Vec<StartupEntry>,
    pub installed_applications: Vec<InstalledApplication>,
    pub running_services: Vec<ServiceInfo>,
    pub scheduled_tasks: Vec<ScheduledTaskInfo>,
    pub drivers: Vec<DriverInfo>,
    pub disks: Vec<DiskHealth>,
    pub resource_usage: ResourceUsage,
    pub network_usage: NetworkUsage,
    pub open_connections: Vec<OpenConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartupEntry {
    pub name: String,
    pub command: String,
    pub location: String,
    pub enabled: bool,
    pub risk_note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstalledApplication {
    pub name: String,
    pub publisher: Option<String>,
    pub version: Option<String>,
    pub install_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceInfo {
    pub name: String,
    pub display_name: String,
    pub status: String,
    pub start_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledTaskInfo {
    pub name: String,
    pub path: String,
    pub enabled: bool,
    pub last_run: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DriverInfo {
    pub name: String,
    pub provider: Option<String>,
    pub signed: Option<bool>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiskHealth {
    pub volume: String,
    pub free_percent: f32,
    pub health: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUsage {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub gpu_percent: Option<f32>,
    pub temperature_celsius: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkUsage {
    pub download_kbps: f32,
    pub upload_kbps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenConnection {
    pub protocol: String,
    pub local_address: String,
    pub remote_address: String,
    pub process_name: Option<String>,
    pub state: String,
}

impl DiagnosticsSnapshot {
    pub fn risk_notes(&self) -> Vec<String> {
        self.startup_entries
            .iter()
            .filter_map(|entry| entry.risk_note.clone())
            .chain(
                self.drivers
                    .iter()
                    .filter(|driver| driver.signed == Some(false))
                    .map(|driver| format!("Unsigned driver: {}", driver.name)),
            )
            .collect()
    }
}
