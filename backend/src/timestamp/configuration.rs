use super::{Timestamp, TimestampValue};

/// The timestamp formats stored on the disk.
#[derive(Debug, PartialEq, Eq)]
pub struct TimestampConfiguration {
    pub date: TimestampVariantConfiguration,
    pub date_time: TimestampVariantConfiguration,
}

// Contains the formats for either date or date_time. First one is the default.
#[derive(Debug, PartialEq, Eq)]
pub struct TimestampVariantConfiguration {
    /// First one is the default.
    pub formats: Vec<String>,
}

impl TimestampVariantConfiguration {
    pub fn formats(&self) -> TimestampFormatIterator {
        TimestampFormatIterator {
            configuration: self,
            format_index: None,
        }
    }
}

/// This is meant to make the method [TimestampConfiguration::parse] easier to read.
pub struct TimestampFormat<'c> {
    configuration: &'c TimestampVariantConfiguration,
    format_index: usize,
    pub(crate) format: &'c str,
}

impl<'c> TimestampFormat<'c> {
    pub fn new_timestamp(&self, date: TimestampValue) -> Timestamp<'c> {
        Timestamp {
            configuration: self.configuration,
            format_index: self.format_index,
            value: date,
        }
    }
}

/// This iterator is meant to make the method [TimestampConfiguration::parse] easier to read.
pub struct TimestampFormatIterator<'c> {
    configuration: &'c TimestampVariantConfiguration,
    format_index: Option<usize>,
}

impl<'c> Iterator for TimestampFormatIterator<'c> {
    type Item = TimestampFormat<'c>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.format_index.map(|i| i + 1).unwrap_or_default();
        self.format_index = Some(index);

        self.configuration
            .formats
            .get(index)
            .map(|format| TimestampFormat {
                configuration: self.configuration,
                format_index: index,
                format,
            })
    }
}
