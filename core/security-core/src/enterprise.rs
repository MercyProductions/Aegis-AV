use crate::policies::PolicyProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceRecord {
    pub device_id: String,
    pub device_name: String,
    pub user_label: Option<String>,
    pub health_score: u8,
    pub protection_status: String,
    pub assigned_policy: PolicyProfile,
    pub signature_version: String,
    pub last_seen: String,
    pub threats_blocked: u32,
    pub quarantine_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemoteScanRequest {
    pub request_id: String,
    pub device_id: String,
    pub profile: String,
    pub requested_by: String,
    pub created_at: String,
    pub requires_user_visibility: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnterpriseAdminState {
    pub devices: Vec<DeviceRecord>,
    pub remote_scan_requests: Vec<RemoteScanRequest>,
}

impl EnterpriseAdminState {
    pub fn assign_policy(&mut self, device_id: &str, policy: PolicyProfile) -> bool {
        if let Some(device) = self
            .devices
            .iter_mut()
            .find(|device| device.device_id == device_id)
        {
            device.assigned_policy = policy;
            true
        } else {
            false
        }
    }

    pub fn unhealthy_devices(&self) -> Vec<&DeviceRecord> {
        self.devices
            .iter()
            .filter(|device| device.health_score < 80)
            .collect()
    }
}
