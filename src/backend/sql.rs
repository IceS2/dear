use async_trait::async_trait;

use sqlx::sqlite::{SqliteArguments, SqlitePool, SqliteQueryResult, SqliteRow};
use sqlx::any::AnyEncode;
use sqlx::Arguments;

// use crate::entity::note::Note;
use crate::entity::SQLEntity;

use super::{Backend, BackendListStrategy, BackendSaveStrategy};

// Auxiliary struct and traits to enable encoding structs.
pub(crate) trait DBEncoder<'q> {
    fn encode<T: AnyEncode<'q> + Send + 'q>(&mut self, col: &str, value: T) -> &mut Self;
}

#[derive(Default)]
struct SqliteEncoder<'q> {
    columns: Vec<String>,
    args: SqliteArguments<'q>,
}

impl<'q> DBEncoder<'q> for SqliteEncoder<'q> {
    fn encode<T: AnyEncode<'q> + Send + 'q>(&mut self, col: &str, value: T) -> &mut Self {
        self.columns.push(col.to_owned());
        self.args.add(value);
        self
    }
}

#[derive(Debug)]
pub(crate) struct SQL {
    pool: SqlitePool,
}

impl SQL {
    pub(crate) async fn new(uri: &str) -> Self {
        let pool = SqlitePool::connect(uri).await.unwrap_or_else(|e| {
            panic!("Couldn't connect to database at {uri}\nError:\n{e}");
        });

        Self { pool }
    }

    pub(crate) async fn create_table(&self) -> SqliteQueryResult {
        sqlx::query(
            "
            CREATE TABLE notes (
                id INTEGER PRIMARY KEY,
                title VARCHAR(255),
                description TEXT,
                created_at DATETIME,
                updated_at DATETIME,
                deleted_at DATETIME,
                tags TEXT)",
        )
        .execute(&self.pool)
        .await
        .unwrap()
    }

    pub(crate) async fn add<T: SQLEntity>(&self, entity: T) -> SqliteQueryResult {
        let mut encoder = SqliteEncoder::default();
        entity.with_encoder(&mut encoder);

        let insert = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            entity.name(),
            encoder.columns.join(", "),
            vec!["?"; encoder.columns.len()].join(", ")
        );

        sqlx::query_with(&insert, encoder.args)
            .execute(&self.pool)
            .await
            .unwrap()
    }

    // pub(crate) async fn list(&self) -> Vec<Note> {
    //     sqlx::query_as::<_, Note>("SELECT * FROM notes")
    //         .fetch_all(&self.pool)
    //         .await
    //         .unwrap()
    // }
}

#[async_trait]
impl Backend for SQL {
    async fn init(&self) {
        let r = self.create_table().await;
        println!("{r:?}");
    }

//     async fn add(&self, note: &EntityType) {
//         let r = self.add(note).await;
//         println!("{r:?}");
//     }

//     async fn list(&self) {
//         let r = self.list().await;
//         println!("{r:?}");
//     }
}
#[async_trait]
impl<Entity> BackendSaveStrategy<Entity> for SQL
where
    Entity: SQLEntity + Send + 'static,
{
    async fn save(&self, entity: Entity) {
        let r = self.add(entity).await;
        println!("{r:?}");
    }
}


#[async_trait]
impl<Entity> BackendListStrategy<Entity> for SQL
where
    Entity: for<'any> ::sqlx::FromRow<'any, SqliteRow> + Unpin + Send + std::fmt::Debug,
{
    async fn list(&self) {
        let r = sqlx::query_as::<_, Entity>("SELECT * FROM notes")
            .fetch_all(&self.pool)
            .await
            .unwrap();
        println!("{r:?}");
    }
}
