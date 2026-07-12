use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::project::Project;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Project>, anyhow::Error>;
    async fn create(&self, project: &Project) -> Result<(), anyhow::Error>;
    async fn update(&self, project: &Project) -> Result<(), anyhow::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), anyhow::Error>;
    async fn list_by_user(&self, user_id: Uuid) -> Result<Vec<Project>, anyhow::Error>;
}
