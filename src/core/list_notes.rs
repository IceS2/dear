use crate::entity::note::Note;
use crate::repository::Repository;

pub fn execute(repo: &mut dyn Repository) -> &Vec<Note> {
    repo.list()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::InMemoryRepository;

    #[test]
    fn it_should_list_all_created_notes() {
        let mut repo = InMemoryRepository::new();

        let first_note = Note::new("My Title", Some("Lorem Ipsum.")).unwrap();
        let second_note = Note::new("Another Title", None).unwrap();

        let expected: Vec<Note> = [first_note.clone(), second_note.clone()].into();

        repo.insert(first_note).unwrap();
        repo.insert(second_note).unwrap();

        let res = repo.list();

        assert_eq!(expected.len(), res.len());
        assert!(expected.iter().all(|note| res.contains(note)));
    }
}
