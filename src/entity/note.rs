#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub title: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

type Error = &'static str;

impl Note {
    pub fn new(
        title: &str,
        description: Option<&str>,
        tags: Option<&Vec<String>>,
    ) -> Result<Self, Error> {
        if title.is_empty() {
            return Err("Note's title can't be empty");
        }
        Ok(Self {
            title: title.into(),
            description: description.map(|desc| desc.into()),
            tags: tags.map(|tag_list| tag_list.iter().map(|tag| tag.to_string()).collect()),
        })
    }
}
