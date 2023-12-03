use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Tags<'p> {
    Tags(Vec<Tag<'p>>),
    SeparatorOnly(&'p str),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'p> {
    pub separator: &'p str,
    pub tag: &'p str,
}

impl<'p> fmt::Display for Tags<'p> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tags::Tags(tags) => {
                for tag in tags {
                    write!(f, "{}{}", tag.separator, tag.tag)?;
                }
            }
            Tags::SeparatorOnly(sep) => write!(f, "{}", sep)?,
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TagConfiguration {
    /// First separator is the default one.
    pub tag_main_separators: Vec<String>,
    pub tag_between_separators: Vec<char>,
}

impl TagConfiguration {
    pub(crate) fn parse<'p>(&self, filename: &mut &'p str) -> Option<Tags<'p>> {
        for separator in &self.tag_main_separators {
            // We find the separator in the filename in order to make a reference to the path
            // and use the same lifetime in any case.
            if let Some(separator_index) = filename.rfind(separator) {
                let separator_len = separator.len();

                let (left, separator_and_left) = filename.split_at(separator_index);
                let (mut separator, mut remainder) = separator_and_left.split_at(separator_len);

                *filename = left;

                if remainder.trim().is_empty() {
                    return Some(Tags::SeparatorOnly(separator));
                }

                let mut tags = Vec::new();

                for (index, next_separator) in
                    remainder.match_indices(&self.tag_between_separators[..])
                {
                    tags.push(Tag {
                        separator,
                        tag: &remainder[..index],
                    });

                    remainder = &remainder[(index + 1)..];
                    separator = next_separator;
                }

                tags.push(Tag {
                    separator,
                    tag: &remainder,
                });

                return Some(Tags::Tags(tags));
            }
        }

        None
    }
}
