#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    pub title: String,
    pub description: Option<String>,
}

type Error = &'static str;

impl Note {
    pub fn new(title: &str, description: Option<&str>) -> Result<Self, Error> {
        if title.is_empty() {
            return Err("Note's title can't be empty");
        }
        Ok(Self {
            title: title.into(),
            description: description.map(|desc| desc.into()),
        })
    }
}
