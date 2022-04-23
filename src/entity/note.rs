#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum NoteError {
    EmptyTitle
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
