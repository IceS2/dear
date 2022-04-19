mod core;
mod entity;
mod repository;

mod cli;

pub use crate::core::create_note;
pub use crate::core::list_notes;
use crate::repository::Repository;
pub use cli::{Action, Cli};

pub fn test() {
    println!("Hello World!");

    let mut repo = repository::InMemoryRepository::new();

    let req = core::create_note::Request {
        title: "My Title",
        description: Some("Lorem Ipsum."),
    };

    let res = core::create_note::execute(&mut repo, &req);

    println!("{req:?}");
    println!("{res:?}");

    let _ = core::create_note::execute(&mut repo, &req);
    let res = repo.list();
    println!("{res:?}")
}

pub fn create_repo() -> repository::InMemoryRepository {
    repository::InMemoryRepository::new()
}

pub fn create_request<'a>(
    title: &'a str,
    description: Option<&'a str>,
) -> core::create_note::Request<'a> {
    core::create_note::Request { title, description }
}
