use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::io::{Read, Write};

use crate::entity::note::Note;
use crate::repository::Repository;

type Error = &'static str;

pub struct FileRepository {
    root_dir: String,
    notes: Vec<Note>,
}

impl FileRepository {
    pub fn new(root_dir: &str) -> Self {
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
        write!(
            file,
            "Title:\n{title}\n\nDescription:\n{description}",
            title = note.title,
            description = desc
        )
        .unwrap();
        Ok(note)
    }

    fn list(&mut self) -> &Vec<Note> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Title:\n(.*)\n\nDescription:\n(.*)").unwrap();
        }
        let paths = fs::read_dir(&self.root_dir).unwrap();
        for path in paths {
            let file_path = path.unwrap().path();
            let mut file = fs::File::open(file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            for cap in RE.captures_iter(&contents) {
                println!("{}", &cap[2]);
                let description = match &cap[2] {
                    "" => None,
                    _ => Some(&cap[2]),
                };
                self.notes.push(Note::new(&cap[1], description).unwrap());
            }
        }
        &self.notes
    }
}

impl Default for FileRepository {
    fn default() -> Self {
        FileRepository::new("./notes/")
    }
}
