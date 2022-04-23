use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use super::{NoteIterator, Repository};
use crate::entity::note::Note;

pub struct FileRepository {
    root_dir: PathBuf,
}

impl FileRepository {
    pub fn new(root_dir: impl AsRef<Path>) -> Self {
        let path: &Path = root_dir.as_ref();

        fs::create_dir_all(path).unwrap();

        Self {
            root_dir: path.to_owned(),
        }
    }
}

impl Repository for FileRepository {
    type Error = std::io::Error;

    fn insert(&mut self, note: Note) -> Result<Note, Self::Error> {
        let file_path = {
            let mut path = self.root_dir.clone();
            path.push(&note.title);
            path
        };

        let mut file = fs::File::create(file_path)?;

        let description = note.description.as_deref().unwrap_or("");
        let tags = match &note.tags {
            Some(tags) => tags.join(","),
            None => "".to_owned(),
        };

        write!(
            file,
            "{title}\n\n{description}\n\n{tags}",
            title = note.title
        )?;

        Ok(note)
    }

    fn list(&self) -> Result<NoteIterator<'_>, Self::Error> {
        let mut notes = vec![];

        for entry in fs::read_dir(&self.root_dir)? {
            let file = fs::File::open(entry?.path())?;
            let mut lines = io::BufReader::new(file).lines();

            let title = lines.next().unwrap()?;
            let (tags, description) = {
                let mut body: Vec<String> = lines.filter_map(|line| line.ok()).collect();
                let last_line = body.pop().unwrap();
                let tags: Vec<String> = last_line
                    .split(',')
                    .filter(|tag| !tag.is_empty())
                    .map(|tag| tag.to_owned())
                    .collect();
                let description = Some(body.join("\n").trim_matches('\n').to_owned())
                    .filter(|desc| !desc.is_empty());
                (tags, description)
            };

            let tags = (!tags.is_empty()).then(|| &tags);

            notes.push(Note::new(&title, description.as_deref(), tags).unwrap());
        }

        Ok(Box::new(notes.into_iter()))
    }
}

impl Default for FileRepository {
    fn default() -> Self {
        let home = env::var("HOME").unwrap();
        FileRepository::new(&format!("{home}/.dear/notes/"))
    }
}
