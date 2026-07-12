use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub target_domains: Vec<String>,
    pub scope_config: serde_json::Value,
    pub consent_given: bool,
    pub consent_at: Option<DateTime<Utc>>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Created,
    Active,
    Archived,
}

/// A single in-scope engagement target belonging to a [`Project`].
///
/// Where `Project::target_domains` holds a flat list of domain strings for
/// quick access, `Target` is the structured representation used for scope
/// enforcement: it distinguishes domains from raw IP addresses and CIDR
/// ranges, and records whether the entry is explicitly in or out of scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub id: Uuid,
    pub project_id: Uuid,
    pub value: String,
    pub target_type: TargetType,
    pub in_scope: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TargetType {
    Domain,
    IpAddress,
    CidrRange,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_type_serializes_snake_case() {
        let json = serde_json::to_string(&TargetType::IpAddress).unwrap();
        assert_eq!(json, "\"ip_address\"");
    }

    #[test]
    fn target_roundtrips_through_serde() {
        let target = Target {
            id: Uuid::nil(),
            project_id: Uuid::nil(),
            value: "example.com".to_string(),
            target_type: TargetType::Domain,
            in_scope: true,
            created_at: DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
        };

        let json = serde_json::to_string(&target).unwrap();
        let decoded: Target = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.value, target.value);
        assert_eq!(decoded.target_type, target.target_type);
        assert_eq!(decoded.in_scope, target.in_scope);
    }
}
