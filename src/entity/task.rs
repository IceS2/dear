use async_trait::async_trait;
use chrono::prelude::*;
use std::collections::HashMap;
use serde_json;

// -----------------------------------------------------------------------------
// Task Entity
// -----------------------------------------------------------------------------
#[derive(Debug)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
pub(crate) struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    tags: Option<serde_json::Value>,
}

impl Task {
    pub(crate) fn builder() -> TaskBuilder {
        TaskBuilder::default()
    }
}

impl From<HashMap<String, String>> for Task {
    fn from(item: HashMap<String, String>) -> Self {
        let mut builder = Task::builder();
        for (attribute, value) in item.iter() {
           builder =  match attribute.as_str() {
                "id" => builder.id(value.parse().unwrap()),
                "title" => builder.title(value),
                "description" => builder.description(value),
                "due_date" => builder.due_date(Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap())),
                "created_at" => builder.created_at(Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap())),
                "updated_at" => builder.updated_at(Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap())),
                "deleted_at" => builder.deleted_at(Utc.from_utc_datetime(&NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S").unwrap())),
                "tags" => builder.tags(serde_json::from_str(value).unwrap()),
                _ => panic!()
            };
        }
        builder.build()
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Task Builder
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct TaskBuilder {
    id: Option<i32>,
    title: Option<String>,
    description: Option<String>,
    due_date: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
    tags: Option<serde_json::Value>
}

impl Default for TaskBuilder {
    fn default() -> Self {
        Self {
            id: None,
            title: None,
            description: None,
            due_date: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            tags: None
        }
    }
}

impl TaskBuilder {
    pub(crate) fn id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }
    pub(crate) fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
    pub(crate) fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
    pub(crate) fn due_date(mut self, due_date: DateTime<Utc>) -> Self {
        self.due_date = Some(due_date);
        self
    }
    pub(crate) fn created_at(mut self, created_at: DateTime<Utc>) -> Self {
        self.created_at = Some(created_at);
        self
    }
    pub(crate) fn updated_at(mut self, updated_at: DateTime<Utc>) -> Self {
        self.updated_at = Some(updated_at);
        self
    }
    pub(crate) fn deleted_at(mut self, deleted_at: DateTime<Utc>) -> Self {
        self.deleted_at = Some(deleted_at);
        self
    }
    pub(crate) fn tags(mut self, tags: serde_json::Value) -> Self {
        self.tags = Some(tags);
        self
    }
    // TODO: Return Result<Note, Err> in order to provide meaningful messages to the user
    pub(crate) fn build(self) -> Task  {
        let id = self.id.unwrap_or(1);
        let title = self.title.unwrap();
        let created_at = self.created_at.unwrap_or_else(|| Utc::now());
        let updated_at = self.updated_at.unwrap_or_else(|| Utc::now());
        Task {
            id,
            title,
            description: self.description,
            due_date: self.due_date,
            created_at,
            updated_at,
            deleted_at: self.deleted_at,
            tags: self.tags
        }
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// SQLite Support
// -----------------------------------------------------------------------------
#[cfg(feature = "sql")]
use sqlx;
#[cfg(feature = "sql")]
use super::{SQLEntitySave, SQLEntityLoad};

#[cfg(feature = "sql")]
#[async_trait]
impl SQLEntitySave for Task {
    async fn save(&self, pool: &sqlx::SqlitePool) {
        let insert = format!(
            "INSERT INTO tasks (title, description, due_date, created_at, updated_at, deleted_at, tags) VALUES ({})",
            vec!["?"; 7].join(", ")
        );

        let res = sqlx::query(&insert)
            .bind(&self.title)
            .bind(&self.description)
            .bind(&self.due_date)
            .bind(&self.created_at)
            .bind(&self.updated_at)
            .bind(&self.deleted_at)
            .bind(&self.tags)
            .execute(pool)
            .await
            .unwrap();

        println!("{res:?}");
    }
}

#[cfg(feature = "sql")]
#[async_trait]
impl SQLEntityLoad for Task {
    async fn list<Entity>(pool: &sqlx::SqlitePool)
    where
        Entity: for<'a> sqlx::FromRow<'a, sqlx::sqlite::SqliteRow> + Send + Unpin + std::fmt::Debug
    {
        let res = sqlx::query_as::<_, Entity>("SELECT * FROM tasks")
            .fetch_all(pool)
            .await
            .unwrap();
        println!("{res:?}");
    }
}
