mod entity;
use entity::{SQLEntitySave, note::Note, task::Task, SQLEntityLoad};

use sqlx::SqlitePool;
use std::collections::HashMap;

pub(crate) struct BuilderArgs {
    args: HashMap<String, String>
}

impl BuilderArgs {
    pub(crate) fn new() -> Self {
        Self { args: HashMap::new() }
    }
    pub(crate) fn insert(mut self, key: &str, value: &str) -> Self {
        self.args.insert(key.to_string(), value.to_string());
        self
    }
    pub(crate) fn values(self) -> HashMap<String, String> {
        self.args
    }
}

pub(crate) enum Entity {
    Note,
    Task
}

pub(crate) fn build_entity(entity: Entity, args: BuilderArgs) -> Box<dyn SQLEntitySave> {
    match entity {
        Entity::Note => {
            let note: Note = args.values().into();
            Box::new(note)
        },
        Entity::Task => {
            let task: Task = args.values().into();
            Box::new(task)
        }
    }
}

pub(crate) async fn list_entity(entity: Entity, pool: &sqlx::SqlitePool) {
    match entity {
        Entity::Note => Note::list::<Note>(pool).await,
        Entity::Task => Task::list::<Task>(pool).await
    };
}

pub async fn test() {
    println!("Testing...");
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap_or_else(|e| {
        panic!("Couldn't connect to database at 'sqlite::memory:'\nError:\n{e}");
    });

    sqlx::query("
        CREATE TABLE notes (
            id INTEGER PRIMARY KEY,
            title VARCHAR(255),
            description TEXT,
            created_at DATETIME,
            updated_at DATETIME,
            deleted_at DATETIME,
            tags TEXT)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("
        CREATE TABLE tasks (
            id INTEGER PRIMARY KEY,
            title VARCHAR(255),
            description TEXT,
            due_date DATETIME,
            created_at DATETIME,
            updated_at DATETIME,
            deleted_at DATETIME,
            tags TEXT)")
        .execute(&pool)
        .await
        .unwrap();


    let args = BuilderArgs::new()
        .insert("title", "My First note");
    let note = build_entity(Entity::Note, args);

    // let note = Note::builder()
    //     .title("My First Note")
    //     .build();

    println!("{note:?}");
    note.save(&pool).await;

    let args = BuilderArgs::new()
        .insert("title", "My Second note")
        .insert("description", "Lorem Ipsum");
    let note = build_entity(Entity::Note, args);

    // let note = Note::builder()
    //     .title("My Second Note")
    //     .description("Lorem Ipsum")
    //     .build();

    println!("{note:?}");
    note.save(&pool).await;

    list_entity(Entity::Note, &pool).await;
    // note.list(&pool).await;

    let args = BuilderArgs::new()
        .insert("title", "My First Task")
        .insert("description", "Gotta learn Rust!!!")
        .insert("due_date", "2022-07-01T00:00:00");
    let task = build_entity(Entity::Task, args);

    // let task = Task::builder()
    //     .title("My First Task")
    //     .description("Gotta learn Rust!!!")
    //     .due_date(Utc.ymd(2022, 7, 1).and_hms(0, 0, 0))
    //     .build();

    task.save(&pool).await;
    list_entity(Entity::Task, &pool).await;
    // task.list(&pool).await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
