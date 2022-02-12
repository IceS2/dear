use sqlx::sqlite::{SqlitePool, SqliteQueryResult};
use crate::entity::note::Note;


#[derive(Debug)]
pub struct SQL {
    pool:SqlitePool
}

impl SQL {
    pub(crate) async fn new(uri: &str) -> Self {
        let pool = SqlitePool::connect(uri).await.unwrap_or_else( |e| {
            panic!("Couldn't connect to database at {uri}\nError:\n{e}");
        });

        Self { pool }
    }

    pub(crate) async fn create_table(&self) -> SqliteQueryResult {
        sqlx::query("
            CREATE TABLE notes (
                id INTEGER PRIMARY KEY,
                title VARCHAR(255),
                description TEXT,
                created_at DATETIME,
                updated_at DATETIME,
                deleted_at DATETIME,
                tags TEXT)")
            .execute(&self.pool)
            .await.unwrap()
    }

    pub(crate) async fn add_note(&self, note: &Note) -> SqliteQueryResult {
        sqlx::query("
            INSERT INTO notes
            (title, description, created_at, updated_at, deleted_at, tags)
            VALUES
            (?, ?, ?, ?, ?, ?)")
            .bind(&note.title)
            .bind(&note.description)
            .bind(&note.created_at)
            .bind(&note.updated_at)
            .bind(&note.deleted_at)
            .bind(&note.tags)
            .execute(&self.pool)
            .await.unwrap()
    }

    pub(crate) async fn list_notes(&self) -> Vec<Note> {
        sqlx::query_as::<_, Note>("SELECT * FROM notes").fetch_all(&self.pool).await.unwrap()
    }
}
