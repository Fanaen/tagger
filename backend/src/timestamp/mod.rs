use self::configuration::{TimestampConfiguration, TimestampFormat, TimestampVariantConfiguration};
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt;

pub mod configuration;

/// Temporal data (and its format) found in the file/directory name. May be a date or a datetime.
#[derive(Debug, PartialEq, Eq)]
pub struct Timestamp<'c> {
    pub configuration: &'c TimestampVariantConfiguration,
    pub format_index: usize,
    pub value: TimestampValue,
}

/// What type of temporal data are we talking about here.
#[derive(Debug, PartialEq, Eq)]
pub enum TimestampValue {
    Date(NaiveDate),
    DateTime(NaiveDateTime),
}

impl<'c> Timestamp<'c> {
    /// Normalisation is when we want to use the main format (listed first) to keep everything
    /// tidy, instead of the one previously used.
    pub fn normalise(&mut self) {
        self.format_index = 0
    }
}

impl<'c> fmt::Display for Timestamp<'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format = self.configuration.formats.get(self.format_index).unwrap();
        match self.value {
            TimestampValue::Date(date) => write!(f, "{}", date.format(format))?,
            TimestampValue::DateTime(date_time) => write!(f, "{}", date_time.format(format))?,
        }
        Ok(())
    }
}

impl TimestampConfiguration {
    /// Try to read temporal data in the file/directory name. Subtract the temporal data from name.
    pub fn parse<'c>(&'c self, filename: &mut &str) -> Option<Timestamp<'c>> {
        for format in self.date_time.formats() {
            if let Some(timestamp) = format.parse_date_time(filename) {
                return Some(timestamp);
            }
        }

        for format in self.date.formats() {
            if let Some(timestamp) = format.parse_date(filename) {
                return Some(timestamp);
            }
        }

        None
    }
}

impl<'c> TimestampFormat<'c> {
    pub fn parse_date<'f>(&self, filename: &'f mut &str) -> Option<Timestamp<'c>> {
        if let Ok((date, remainder)) = NaiveDate::parse_and_remainder(filename, self.format) {
            *filename = remainder;
            Some(self.new_timestamp(TimestampValue::Date(date)))
        } else {
            None
        }
    }

    pub fn parse_date_time<'f>(&self, filename: &'f mut &str) -> Option<Timestamp<'c>> {
        if let Ok((date, remainder)) = NaiveDateTime::parse_and_remainder(filename, self.format) {
            *filename = remainder;
            Some(self.new_timestamp(TimestampValue::DateTime(date)))
        } else {
            None
        }
    }
}
