use std::io::{BufRead, Write};
use std::{fs, io};

use crate::entity::note::Note;
use crate::repository::Repository;

type Error = &'static str;

pub struct FileRepository {
    root_dir: String,
    notes: Vec<Note>,
}

impl FileRepository {
    pub fn new(root_dir: &str) -> Self {
        fs::create_dir_all(root_dir).unwrap();
        Self {
            root_dir: String::from(root_dir),
            notes: vec![],
        }
    }
}

impl Repository for FileRepository {
    fn insert(&mut self, note: Note) -> Result<Note, Error> {
        let mut file = fs::File::create(format!(
            "{path}/{title}",
            path = self.root_dir,
            title = note.title
        ))
        .unwrap();
        let desc = match &note.description {
            Some(d) => d,
            None => "",
        };

        let t = match &note.tags {
            Some(tags) => tags.join(","),
            None => String::from("\n"),
        };
        write!(
            file,
            "{title}\n{description}\n{tags}",
            title = note.title,
            description = desc,
            tags = t
        )
        .unwrap();
        Ok(note)
    }

    fn list(&mut self) -> &Vec<Note> {
        let paths = fs::read_dir(&self.root_dir).unwrap();
        for path in paths {
            let file_path = path.unwrap().path();
            let file = fs::File::open(file_path).unwrap();
            let mut lines = io::BufReader::new(file).lines();
            let title = lines.next().unwrap().unwrap();
            let mut body: Vec<String> = lines.map(|line| line.unwrap()).collect();
            let last = body.pop().unwrap();
            let tags: Vec<String> = last
                .split(',')
                .map(|tag| tag.to_string())
                .filter(|tag| !tag.is_empty())
                .collect();
            let description = body.join("\n");

            let desc: Option<&str> = match &*description {
                "" => None,
                _ => Some(&description),
            };

            let t = if tags.is_empty() { None } else { Some(&tags) };

            self.notes.push(Note::new(&title, desc, t).unwrap())
        }
        &self.notes
    }
}

impl Default for FileRepository {
    fn default() -> Self {
        FileRepository::new("./notes/")
    }
}
