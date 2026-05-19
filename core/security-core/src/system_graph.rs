use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeKind {
    Process,
    Service,
    Driver,
    Connection,
    Module,
    StartupEntry,
    ScheduledTask,
    Threat,
    AutomationEvent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GraphEdgeKind {
    ParentChild,
    DependsOn,
    OwnsConnection,
    LoadsDriver,
    Triggers,
    Mitigates,
    ScheduledBy,
    StartedBy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemGraphNode {
    pub id: String,
    pub label: String,
    pub kind: GraphNodeKind,
    pub risk_score: u8,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemGraphEdge {
    pub from: String,
    pub to: String,
    pub relation: GraphEdgeKind,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LiveSystemGraph {
    pub nodes: Vec<SystemGraphNode>,
    pub edges: Vec<SystemGraphEdge>,
}

impl LiveSystemGraph {
    pub fn add_node(&mut self, node: SystemGraphNode) {
        if !self.nodes.iter().any(|existing| existing.id == node.id) {
            self.nodes.push(node);
        }
    }

    pub fn add_edge(&mut self, edge: SystemGraphEdge) {
        if self.nodes.iter().any(|node| node.id == edge.from)
            && self.nodes.iter().any(|node| node.id == edge.to)
        {
            self.edges.push(edge);
        }
    }

    pub fn neighbors(&self, node_id: &str) -> Vec<&SystemGraphNode> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node_id)
            .filter_map(|edge| self.nodes.iter().find(|node| node.id == edge.to))
            .collect()
    }

    pub fn highest_risk_node(&self) -> Option<&SystemGraphNode> {
        self.nodes.iter().max_by_key(|node| node.risk_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_traces_process_relationships() {
        let mut graph = LiveSystemGraph::default();
        graph.add_node(SystemGraphNode {
            id: "proc_browser".to_string(),
            label: "browser.exe".to_string(),
            kind: GraphNodeKind::Process,
            risk_score: 8,
            status: "running".to_string(),
        });
        graph.add_node(SystemGraphNode {
            id: "proc_shell".to_string(),
            label: "powershell.exe".to_string(),
            kind: GraphNodeKind::Process,
            risk_score: 72,
            status: "review".to_string(),
        });
        graph.add_edge(SystemGraphEdge {
            from: "proc_browser".to_string(),
            to: "proc_shell".to_string(),
            relation: GraphEdgeKind::ParentChild,
        });

        assert_eq!(graph.neighbors("proc_browser").len(), 1);
        assert_eq!(graph.highest_risk_node().unwrap().label, "powershell.exe");
    }
}
