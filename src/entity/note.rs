#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum NoteError {
    EmptyTitle,
}

impl Note {
    pub fn new(
        title: &str,
        description: Option<&str>,
        tags: Option<&Vec<String>>,
    ) -> Result<Self, NoteError> {
        if title.is_empty() {
            return Err(NoteError::EmptyTitle);
        }
        Ok(Self {
            title: title.to_owned(),
            description: description.map(|desc| desc.to_owned()),
            tags: tags.map(|tag_list| tag_list.iter().map(|tag| tag.to_owned()).collect()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_a_note_with_title() {
        let title = "My Title";
        let description = None;
        let tags = None;

        let res = Note::new(title, description, tags).unwrap();

        assert_eq!(res.title, title);
        assert_eq!(res.description, None);
        assert_eq!(res.tags, None);
    }

    #[test]
    fn it_should_return_a_full_note() {
        let title = "My Title";
        let description = Some("Description");
        let tags = vec!["tag1".to_owned(), "tag2".to_owned()];

        let res = Note::new(title, description, Some(&tags)).unwrap();

        let expected_tags = vec!["tag1".to_owned(), "tag2".to_owned()];

        assert_eq!(res.title, title);
        assert_eq!(res.description.as_deref(), description);

        let matching = res
            .tags
            .clone()
            .unwrap()
            .iter()
            .zip(expected_tags.iter())
            .filter(|&(expected_tag, res_tag)| expected_tag == res_tag)
            .count();
        assert!(matching == expected_tags.len() && matching == res.tags.unwrap().len())
    }

    #[test]
    fn it_should_raise_an_empty_title_error_when_title_is_empty() {
        let title = "";

        let res = Note::new(title, None, None);

        match res {
            Err(NoteError::EmptyTitle) => {}
            _ => unreachable!(),
        }
    }
}
