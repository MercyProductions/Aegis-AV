use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetworkConnection {
    pub process_name: String,
    pub process_id: Option<u32>,
    pub protocol: String,
    pub local_endpoint: String,
    pub remote_endpoint: String,
    pub direction: Direction,
    pub state: String,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub dns_name: Option<String>,
    pub first_seen: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Outbound,
    Inbound,
    Listening,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListeningPort {
    pub protocol: String,
    pub local_endpoint: String,
    pub process_name: Option<String>,
    pub process_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DnsRequest {
    pub process_name: Option<String>,
    pub query: String,
    pub resolved_ips: Vec<String>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FirewallVisibilitySnapshot {
    pub captured_at: String,
    pub active_connections: Vec<NetworkConnection>,
    pub listening_ports: Vec<ListeningPort>,
    pub dns_requests: Vec<DnsRequest>,
    pub connection_history: Vec<NetworkConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectionAction {
    Observe,
    AlertMode,
    AskBeforeOutboundForUnknownApps,
    TemporaryNetworkRestriction { minutes: u16 },
    BlockConnection { reason: String },
}

impl FirewallVisibilitySnapshot {
    pub fn outbound_unknown_count(&self) -> usize {
        self.active_connections
            .iter()
            .filter(|connection| connection.direction == Direction::Outbound)
            .filter(|connection| connection.dns_name.is_none())
            .count()
    }

    pub fn bandwidth_total(&self) -> u64 {
        self.active_connections
            .iter()
            .map(|connection| connection.bytes_sent + connection.bytes_received)
            .sum()
    }
}
