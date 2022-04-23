use crate::entity::note::Note;
use crate::repository::Repository;

pub fn execute<R: Repository>(repo: &R) -> Result<impl Iterator<Item = Note> + '_, R::Error> {
    repo.list()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryRepository;

    #[test]
    fn it_should_list_all_created_notes() {
        let mut repo = InMemoryRepository::new();

        let first_note = Note::new("My Title", Some("Lorem Ipsum."), None).unwrap();
        let second_note = Note::new("Another Title", None, None).unwrap();
        let third_note = Note::new("Note with Tags", None, Some(&vec!["tag1".to_owned()])).unwrap();

        let expected = [first_note.clone(), second_note.clone(), third_note.clone()];

        repo.insert(first_note).unwrap();
        repo.insert(second_note).unwrap();
        repo.insert(third_note).unwrap();

        let res = repo.list().unwrap().collect::<Vec<_>>();

        assert_eq!(expected.len(), res.len());
        assert!(expected.iter().all(|note| res.contains(note)));
    }
}
