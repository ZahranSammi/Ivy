use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub id: String,
    pub source_id: String,
    pub target_id: String,
    pub r#type: String,
    pub properties: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    HasSubdomain,
    ResolvesTo,
    HasPort,
    RunsService,
    UsesTechnology,
    HasVulnerability,
    AssociatedEmail,
    HasCertificate,
    DiscoveredUrl,
}

impl EdgeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EdgeType::HasSubdomain => "HAS_SUBDOMAIN",
            EdgeType::ResolvesTo => "RESOLVES_TO",
            EdgeType::HasPort => "HAS_PORT",
            EdgeType::RunsService => "RUNS_SERVICE",
            EdgeType::UsesTechnology => "USES_TECHNOLOGY",
            EdgeType::HasVulnerability => "HAS_VULNERABILITY",
            EdgeType::AssociatedEmail => "ASSOCIATED_EMAIL",
            EdgeType::HasCertificate => "HAS_CERTIFICATE",
            EdgeType::DiscoveredUrl => "DISCOVERED_URL",
        }
    }
}
