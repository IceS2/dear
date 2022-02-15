use backend::sql::SQL;
use backend::Backend;
use chrono::prelude::*;
use entity::note::Note;
use entity::EntityType;

mod backend;
mod entity;

pub async fn test() {
    let sql_client = SQL::new("sqlite::memory:").await;
    println!("{sql_client:?}");

    sql_client.init().await;

    let note = EntityType::Note(Note {
        id: 1,
        title: "My Title".to_string(),
        description: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
        tags: Some(serde_json::from_str(r#"["tag1", "tag2"]"#).unwrap()),
    });

    sql_client.save(note).await;

    sql_client.list::<Note>().await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
