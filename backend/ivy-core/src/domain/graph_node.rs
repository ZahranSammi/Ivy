use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub labels: Vec<String>,
    pub properties: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainNode {
    pub name: String,
    pub registrar: Option<String>,
    pub creation_date: Option<String>,
    pub expiry_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubdomainNode {
    pub name: String,
    pub source: Option<String>,
    pub first_seen: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPAddressNode {
    pub address: String,
    pub asn: Option<String>,
    pub geo_location: Option<String>,
    pub isp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortNode {
    pub number: u16,
    pub protocol: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceNode {
    pub name: String,
    pub version: Option<String>,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityNode {
    pub id: String,
    pub severity: String,
    pub cvss: Option<f32>,
    pub description: Option<String>,
}
