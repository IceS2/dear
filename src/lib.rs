pub mod config;
mod core;
mod entity;
pub mod repository;

mod cli;

pub use crate::core::create_note;
pub use crate::core::list_notes;
pub use cli::{Action, Cli};

pub fn create_request<'a>(
    title: &'a str,
    description: Option<&'a str>,
    tags: Option<&'a Vec<String>>,
) -> core::create_note::Request<'a> {
    core::create_note::Request {
        title,
        description,
        tags,
    }
}
