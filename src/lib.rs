use crate::backend::sql::SQL;
use chrono::prelude::*;
use entity::note::Note;

mod backend;
mod entity;

pub async fn test() {
    let sql_client = SQL::new("sqlite::memory:").await;
    println!("{sql_client:?}");

    let res = sql_client.create_table().await;
    println!("{res:?}");

    let note = Note {
        id: 10,
        title: "My Title".to_string(),
        description: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
        tags: serde_json::from_str(r#"["tag1", "tag2"]"#).unwrap()
    };

    let res = sql_client.add_note(&note).await;
    println!("{res:?}");

    let res = sql_client.list_notes().await;
    println!("{res:?}");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
