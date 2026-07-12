use async_trait::async_trait;

#[async_trait]
pub trait Cache: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>, anyhow::Error>;
    async fn set(&self, key: &str, value: &str, ttl_secs: u64) -> Result<(), anyhow::Error>;
    async fn delete(&self, key: &str) -> Result<(), anyhow::Error>;
}
