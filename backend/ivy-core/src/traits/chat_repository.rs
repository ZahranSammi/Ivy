use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::chat::ChatMessage;

#[async_trait]
pub trait ChatRepository: Send + Sync {
    async fn save(&self, message: &ChatMessage) -> Result<(), anyhow::Error>;
    async fn list_by_project(&self, project_id: Uuid) -> Result<Vec<ChatMessage>, anyhow::Error>;
}
