use std::fmt;

use owo_colors::OwoColorize;

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

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = self.description.as_deref().unwrap_or("");
        let tags = match self.tags.as_deref() {
            Some(tags) => format!("[{}]", tags.join("][")),
            None => "".to_owned(),
        };

        writeln!(
            f,
            "{}{:\u{2500}<78}{}",
            "\u{250C}".purple().dimmed(),
            "\u{2500}".purple().dimmed(),
            "\u{2510}".purple().dimmed()
        )?;
        for title_line in textwrap::wrap(&self.title, 78) {
            writeln!(
                f,
                "{}{:^78}{}",
                "\u{2502}".purple().dimmed(),
                title_line.bright_purple().bold(),
                "\u{2502}".purple().dimmed()
            )?;
        }
        writeln!(
            f,
            "{:<79}{}",
            "\u{2502}".purple().dimmed(),
            "\u{2502}".purple().dimmed()
        )?;
        for desc_line in textwrap::wrap(description, 77) {
            writeln!(
                f,
                "{} {:<77}{}",
                "\u{2502}".purple().dimmed(),
                desc_line.yellow(),
                "\u{2502}".purple().dimmed()
            )?;
        }
        writeln!(
            f,
            "{:<79}{}",
            "\u{2502}".purple().dimmed(),
            "\u{2502}".purple().dimmed()
        )?;
        writeln!(
            f,
            "{}{:>78}{}",
            "\u{2502}".purple().dimmed(),
            tags.bright_cyan().bold(),
            "\u{2502}".purple().dimmed()
        )?;
        writeln!(
            f,
            "{}{:\u{2500}<78}{}",
            "\u{2514}".purple().dimmed(),
            "\u{2500}".purple().dimmed(),
            "\u{2518}".purple().dimmed()
        )
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
