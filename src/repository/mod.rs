mod file;
mod in_memory;

pub use file::FileRepository;
pub use in_memory::InMemoryRepository;

use crate::entity::note::Note;

pub type NoteIterator<'repo> = Box<dyn Iterator<Item = Note> + 'repo>;

pub trait Repository: std::fmt::Debug {
    type Error: std::error::Error;

    fn insert(&mut self, note: Note) -> Result<Note, Self::Error>;
    fn list(&self) -> Result<NoteIterator<'_>, Self::Error>;
}
