pub(crate) mod note;
pub(crate) mod task;
use async_trait::async_trait;

#[cfg(feature = "sql")]
use sqlx;

#[cfg(feature = "sql")]
#[async_trait]
pub(crate) trait SQLEntitySave: std::fmt::Debug {
    async fn save(&self, pool: &sqlx::SqlitePool);
}

#[cfg(feature = "sql")]
#[async_trait]
pub(crate) trait SQLEntityLoad {
    async fn list<Entity>(pool: &sqlx::SqlitePool, filter: Option<Box<dyn SQLFilter>>)
        where
            Entity: for<'a> sqlx::FromRow<'a, sqlx::sqlite::SqliteRow> + Send + Unpin + std::fmt::Debug + EntityImpl;
}

pub(crate) trait SQLFilter: Send {
    fn conditions(&self) -> Vec<String>;
}

pub(crate) trait EntityImpl {
    fn name() -> &'static str;
    fn attributes() -> Vec<&'static str>;
}

