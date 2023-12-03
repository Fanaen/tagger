use crate::{configuration::TimestampConfiguration, TagConfiguration, Tags, Timestamp};
use camino::Utf8Path;
use std::fmt;

/// Serialisation/deserialisation configuration for file/directory names.
#[derive(Debug, PartialEq, Eq)]
pub struct StructuredNameConfiguration {
    pub timestamp_configuration: TimestampConfiguration,
    pub tag_configuration: TagConfiguration,
}

/// The result of a parsed name of a file/directory.
#[derive(Debug, PartialEq, Eq)]
pub struct StructuredName<'p, 'c> {
    pub configuration: &'c StructuredNameConfiguration,

    pub timestamp: Option<Timestamp<'c>>,
    pub filename: Option<&'p str>,
    pub tags: Option<Tags<'p>>,
    pub extension: Option<&'p str>,
}

impl<'p, 'c> StructuredName<'p, 'c> {
    pub fn parse_from(
        path: &'p Utf8Path,
        configuration: &'c StructuredNameConfiguration,
    ) -> StructuredName<'p, 'c> {
        let mut filename = path.file_stem();
        let mut timestamp = None;
        let mut tags = None;

        if let Some(mut name) = filename.as_mut() {
            timestamp = configuration.timestamp_configuration.parse(&mut name);
            tags = configuration.tag_configuration.parse(&mut name);
        }

        StructuredName {
            timestamp,
            tags,
            filename,
            extension: path.extension(),
            configuration,
        }
    }

    /// Normalisation is when we want to use the main format (listed first) to keep everything
    /// tidy, instead of the one previously used.
    pub fn normalise(&mut self) {
        if let Some(timestamp) = self.timestamp.as_mut() {
            timestamp.normalise();
        }
    }
}

impl<'p, 'c> fmt::Display for StructuredName<'p, 'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(timestamp) = &self.timestamp {
            write!(f, "{}", timestamp)?;
        }

        if let Some(filename) = self.filename {
            write!(f, "{}", filename)?;
        }

        if let Some(tags) = &self.tags {
            write!(f, "{}", tags)?;
        }

        if let Some(extension) = self.extension {
            write!(f, ".{}", extension)?;
        }

        Ok(())
    }
}
