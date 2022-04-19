mod file;
mod in_memory;

pub use file::FileRepository;
pub use in_memory::InMemoryRepository;

use crate::entity::note::Note;

type Error = &'static str;

pub trait Repository {
    fn insert(&mut self, note: Note) -> Result<Note, Error>;
    fn list(&mut self) -> &Vec<Note>;
}
