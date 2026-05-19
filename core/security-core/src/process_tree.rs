use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessInfo {
    pub pid: u32,
    pub parent_pid: Option<u32>,
    pub name: String,
    pub path: Option<PathBuf>,
    pub command_line: Option<String>,
    pub signature_status: SignatureStatus,
    pub sha256: Option<String>,
    pub start_time_unix_seconds: Option<u64>,
    pub risk_score: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignatureStatus {
    Trusted,
    SignedUnknown,
    Unsigned,
    Invalid,
    Unchecked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessNode {
    pub process: ProcessInfo,
    pub children: Vec<ProcessNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProcessSnapshot {
    pub captured_at_unix_seconds: u64,
    pub processes: Vec<ProcessInfo>,
}

impl ProcessSnapshot {
    pub fn roots(&self) -> Vec<ProcessNode> {
        let by_pid: HashMap<u32, ProcessInfo> = self
            .processes
            .iter()
            .cloned()
            .map(|process| (process.pid, process))
            .collect();

        let mut children_by_parent: HashMap<u32, Vec<u32>> = HashMap::new();
        for process in &self.processes {
            if let Some(parent_pid) = process.parent_pid {
                children_by_parent
                    .entry(parent_pid)
                    .or_default()
                    .push(process.pid);
            }
        }

        self.processes
            .iter()
            .filter(|process| {
                process
                    .parent_pid
                    .map(|parent_pid| !by_pid.contains_key(&parent_pid))
                    .unwrap_or(true)
            })
            .filter_map(|process| build_node(process.pid, &by_pid, &children_by_parent))
            .collect()
    }
}

fn build_node(
    pid: u32,
    by_pid: &HashMap<u32, ProcessInfo>,
    children_by_parent: &HashMap<u32, Vec<u32>>,
) -> Option<ProcessNode> {
    let process = by_pid.get(&pid)?.clone();
    let children = children_by_parent
        .get(&pid)
        .into_iter()
        .flat_map(|children| children.iter())
        .filter_map(|child_pid| build_node(*child_pid, by_pid, children_by_parent))
        .collect();

    Some(ProcessNode { process, children })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_process_relationships() {
        let snapshot = ProcessSnapshot {
            captured_at_unix_seconds: 1,
            processes: vec![
                process(1, None, "explorer.exe"),
                process(2, Some(1), "browser.exe"),
                process(3, Some(2), "downloaded_file.exe"),
                process(4, Some(3), "powershell.exe"),
            ],
        };

        let roots = snapshot.roots();

        assert_eq!(roots.len(), 1);
        assert_eq!(
            roots[0].children[0].children[0].children[0].process.name,
            "powershell.exe"
        );
    }

    fn process(pid: u32, parent_pid: Option<u32>, name: &str) -> ProcessInfo {
        ProcessInfo {
            pid,
            parent_pid,
            name: name.to_string(),
            path: None,
            command_line: None,
            signature_status: SignatureStatus::Unchecked,
            sha256: None,
            start_time_unix_seconds: None,
            risk_score: 0,
        }
    }
}
