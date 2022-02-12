use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
#[cfg(feature = "sql")]
use sqlx;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "sql", derive(sqlx::FromRow))]
pub(crate) struct Note {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) description: Option<String>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) deleted_at: Option<DateTime<Utc>>,
    pub(crate) tags: Option<serde_json::Value>
}

