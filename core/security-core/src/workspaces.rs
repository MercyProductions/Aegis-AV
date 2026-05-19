use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceKind {
    Security,
    Diagnostics,
    Developer,
    Automation,
    Network,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceDefinition {
    pub kind: WorkspaceKind,
    pub layout: String,
    pub widgets: Vec<String>,
    pub tools: Vec<String>,
    pub permissions: Vec<String>,
    pub automation_visible: bool,
}

pub struct WorkspaceCatalog;

impl WorkspaceCatalog {
    pub fn defaults() -> Vec<WorkspaceDefinition> {
        vec![
            workspace(
                WorkspaceKind::Security,
                "command_center",
                ["risk", "incidents", "event_stream"],
                ["quick_scan", "incident_export"],
                ["read_events", "request_scan"],
                true,
            ),
            workspace(
                WorkspaceKind::Diagnostics,
                "observability_grid",
                ["cpu", "memory", "disk", "services"],
                ["trace_viewer", "crash_analysis"],
                ["read_diagnostics"],
                false,
            ),
            workspace(
                WorkspaceKind::Developer,
                "toolbench",
                ["logs", "hashing", "signature_tests"],
                ["yara_tester", "event_inspector"],
                ["read_events", "write_reports"],
                true,
            ),
            workspace(
                WorkspaceKind::Automation,
                "workflow_board",
                ["workflow_runs", "pending_confirmations"],
                ["workflow_editor", "policy_preview"],
                ["read_events", "manage_workflows"],
                true,
            ),
            workspace(
                WorkspaceKind::Network,
                "topology",
                ["connections", "dns", "bandwidth"],
                ["connection_review"],
                ["read_network_metadata"],
                true,
            ),
            workspace(
                WorkspaceKind::Enterprise,
                "fleet",
                ["devices", "policies", "licenses"],
                ["remote_scan_request"],
                ["enterprise_admin"],
                true,
            ),
        ]
    }
}

fn workspace<const W: usize, const T: usize, const P: usize>(
    kind: WorkspaceKind,
    layout: &str,
    widgets: [&str; W],
    tools: [&str; T],
    permissions: [&str; P],
    automation_visible: bool,
) -> WorkspaceDefinition {
    WorkspaceDefinition {
        kind,
        layout: layout.to_string(),
        widgets: widgets.into_iter().map(str::to_string).collect(),
        tools: tools.into_iter().map(str::to_string).collect(),
        permissions: permissions.into_iter().map(str::to_string).collect(),
        automation_visible,
    }
}
