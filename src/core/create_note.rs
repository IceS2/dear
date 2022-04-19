use crate::entity::note::Note;
use crate::repository::Repository;

#[derive(Debug)]
pub struct Request<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
}

type Error = &'static str;

pub fn execute(repo: &mut dyn Repository, req: &Request) -> Result<Note, Error> {
    match Note::try_from(req) {
        Ok(note) => repo.insert(note),
        Err(msg) => Err(msg),
    }
}

impl TryFrom<&Request<'_>> for Note {
    type Error = &'static str;
    fn try_from(req: &Request) -> Result<Self, Self::Error> {
        Note::new(req.title, req.description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryRepository;

    #[test]
    fn it_should_return_the_created_note() {
        let mut repo = InMemoryRepository::new();

        let title = "My Title";
        let description = "Lorem Ipsum.";

        let req = Request {
            title,
            description: Some(description),
        };

        let res = execute(&mut repo, &req).unwrap();

        assert_eq!(res.title, title);
        assert_eq!(res.description, Some(String::from(description)));
    }

    #[test]
    fn it_should_raise_an_error_when_title_is_empty() {
        let mut repo = InMemoryRepository::new();

        let title = "";
        let description = "Lorem Ipsum.";

        let req = Request {
            title,
            description: Some(description),
        };

        let res = execute(&mut repo, &req);

        match res {
            Err(_) => {}
            _ => unreachable!(),
        }
    }
}
