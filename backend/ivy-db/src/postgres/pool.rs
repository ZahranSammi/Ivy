use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct PgPool {
    pub inner: Pool<Postgres>,
}

impl PgPool {
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        Ok(Self { inner: pool })
    }
}
