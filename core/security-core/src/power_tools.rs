use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PowerTool {
    ProcessExplorer,
    FileInspector,
    HashGenerator,
    SignatureTester,
    YaraTestingTool,
    LogExplorer,
    EventInspector,
    ScanDebugger,
    HarmlessThreatReplaySimulator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolRunRequest {
    pub tool: PowerTool,
    pub target: Option<String>,
    pub harmless_data_only: bool,
}

impl ToolRunRequest {
    pub fn validate(&self) -> Vec<String> {
        if matches!(self.tool, PowerTool::HarmlessThreatReplaySimulator) && !self.harmless_data_only
        {
            vec!["threat replay simulator only supports harmless data".to_string()]
        } else {
            Vec::new()
        }
    }
}
