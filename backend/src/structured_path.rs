use crate::{StructuredName, StructuredNameConfiguration};
use camino::Utf8Path;
use std::fmt;

/// The result of a parsed path.
#[derive(Debug, PartialEq, Eq)]
pub struct StructuredPath<'p, 'c> {
    pub original_path: &'p Utf8Path,
    pub name: StructuredName<'p, 'c>,
}

impl<'p, 'c> StructuredPath<'p, 'c> {
    pub fn parse_from(
        path: &'p Utf8Path,
        configuration: &'c StructuredNameConfiguration,
    ) -> StructuredPath<'p, 'c> {
        StructuredPath {
            name: StructuredName::parse_from(path, configuration),
            original_path: path,
        }
    }

    /// Normalisation is when we want to use the main format (listed first) to keep everything
    /// tidy, instead of the one previously used.
    pub fn normalise(&mut self) {
        self.name.normalise();
    }
}

/// A way to get the result as a simple string.
impl<'p, 'c> fmt::Display for StructuredPath<'p, 'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let filename = format!("{}", self.name);
        // `with_file_name` allow us to keep the separator as is.
        // It allocates a little but it's so much simpler.
        let path = self.original_path.with_file_name(filename);

        write!(f, "{}", path)?;

        Ok(())
    }
}
