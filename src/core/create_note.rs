use crate::entity::note::{Note, NoteError};
use crate::repository::Repository;

#[derive(Debug)]
pub struct Request<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub tags: Option<&'a Vec<String>>,
}

#[derive(Debug)]
pub enum CreateNoteError<E> {
    BadRequest(NoteError),
    Repository(E),
}

pub fn execute<R>(repo: &mut R, req: &Request<'_>) -> Result<Note, CreateNoteError<R::Error>>
where
    R: Repository,
{
    let note = req.try_into().map_err(CreateNoteError::BadRequest)?;
    repo.insert(note).map_err(CreateNoteError::Repository)
}

impl TryFrom<&Request<'_>> for Note {
    type Error = NoteError;

    fn try_from(req: &Request<'_>) -> Result<Self, Self::Error> {
        Note::new(req.title, req.description, req.tags)
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
            tags: None,
        };

        let res = execute(&mut repo, &req).unwrap();

        assert_eq!(res.title, title);
        assert_eq!(res.description, Some(String::from(description)));
    }

    #[test]
    fn it_should_raise_a_bad_request_error_when_title_is_empty() {
        let mut repo = InMemoryRepository::new();

        let title = "";
        let description = "Lorem Ipsum.";

        let req = Request {
            title,
            description: Some(description),
            tags: None,
        };

        let res = execute(&mut repo, &req);

        match res {
            Err(CreateNoteError::BadRequest(_)) => {}
            _ => unreachable!(),
        }
    }
}
