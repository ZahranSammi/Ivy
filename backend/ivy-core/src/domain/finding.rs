use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub r#type: String, // Type of entity (Subdomain, Vulnerability, etc.)
    pub data: serde_json::Value,
    pub confidence: f32,
    pub relationships: Vec<FindingRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindingRelationship {
    pub r#type: String,
    pub from_type: String,
    pub from_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Informational,
}
