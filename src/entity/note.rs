use crate::backend::sql::DBEncoder;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::SQLEntity;
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
    pub(crate) tags: Option<serde_json::Value>,
}

impl SQLEntity for Note {
    fn name(&self) -> &'static str {
        "notes"
    }

    fn with_encoder<'q>(&'q self, encoder: &mut impl DBEncoder<'q>) {
        encoder
            .encode("id", &self.id)
            .encode("title", &self.title)
            .encode("description", &self.description)
            .encode("created_at", &self.created_at)
            .encode("updated_at", &self.updated_at)
            .encode("deleted_at", &self.deleted_at)
            .encode("tags", &self.tags);
    }
}

