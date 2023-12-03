use crate::{
    configuration::{TimestampConfiguration, TimestampVariantConfiguration},
    StructuredName, StructuredNameConfiguration, StructuredPath, Tag, TagConfiguration, Tags,
    Timestamp, TimestampValue,
};
use camino::{Utf8Path, Utf8PathBuf};
use chrono::NaiveDate;
use pretty_assertions::assert_eq;

/// Allow us to get valid OS-specific paths to not fail on inconsistent separators.
pub fn create_path() -> Utf8PathBuf {
    Utf8PathBuf::from("some").join("folder")
}

/// Configuration should not be super exotic.
pub fn create_configuration() -> StructuredNameConfiguration {
    StructuredNameConfiguration {
        timestamp_configuration: TimestampConfiguration {
            date: TimestampVariantConfiguration {
                formats: vec!["%Y-%m-%d".to_string(), "%Y_%m_%d".to_string()],
            },
            date_time: TimestampVariantConfiguration {
                formats: vec!["%Y-%m-%d %Hh%M".to_string()],
            },
        },
        tag_configuration: TagConfiguration {
            tag_main_separators: vec![" -- ".to_string()],
            tag_between_separators: vec![' '],
        },
    }
}

/// Since we can cross-check everything from the content, test are simple to do.
pub fn check(path: StructuredPath, normalised_path: &Utf8Path) {
    let mut result = StructuredPath::parse_from(path.original_path, path.name.configuration);

    // Since we have the original path, we can recompute the whole thing.
    assert_eq!(result, path);
    // We want to keep everything as is. `Utf8Path` does not keep separator
    // information and a perfect match is not really needed. `create_path` allow
    // us to get OS-speficic paths so the test pass on every system.
    assert_eq!(result.to_string().as_str(), path.original_path.as_str());

    // The configuration allow multiple date formats and there is a way to
    // keep everything tidy.
    result.normalise();
    assert_eq!(result.to_string().as_str(), normalised_path.as_str());
}

#[test]
pub fn parse_path_without_date() {
    let config = create_configuration();
    let path = create_path().join("Some filename.txt");

    check(
        StructuredPath {
            original_path: &path,
            name: StructuredName {
                configuration: &config,
                timestamp: None,
                filename: Some("Some filename"),
                tags: None,
                extension: Some("txt"),
            },
        },
        &path,
    )
}

#[test]
pub fn parse_path_with_date() {
    let config = create_configuration();
    let path = create_path().join("2022-10-27-Some-filename.pdf");

    check(
        StructuredPath {
            original_path: &path,
            name: StructuredName {
                configuration: &config,
                timestamp: Some(Timestamp {
                    configuration: &config.timestamp_configuration.date,
                    format_index: 0,
                    value: TimestampValue::Date(NaiveDate::from_ymd_opt(2022, 10, 27).unwrap()),
                }),
                filename: Some("-Some-filename"),
                tags: None,
                extension: Some("pdf"),
            },
        },
        &path,
    );
}

#[test]
pub fn parse_path_with_date_time() {
    let config = create_configuration();
    let path = create_path().join("2022-10-27 15h35 Some filename.pdf");

    check(
        StructuredPath {
            original_path: &path,
            name: StructuredName {
                configuration: &config,
                timestamp: Some(Timestamp {
                    configuration: &config.timestamp_configuration.date_time,
                    format_index: 0,
                    value: TimestampValue::DateTime(
                        NaiveDate::from_ymd_opt(2022, 10, 27)
                            .unwrap()
                            .and_hms_opt(15, 35, 0)
                            .unwrap(),
                    ),
                }),
                filename: Some(" Some filename"),
                tags: None,
                extension: Some("pdf"),
            },
        },
        &path,
    );
}

#[test]
pub fn parse_path_with_secondary_date() {
    let config = create_configuration();
    let path = create_path().join("2022_10_27 Some filename.pdf");
    let normalised_path = create_path().join("2022-10-27 Some filename.pdf");

    check(
        StructuredPath {
            original_path: &path,
            name: StructuredName {
                configuration: &config,
                timestamp: Some(Timestamp {
                    configuration: &config.timestamp_configuration.date,
                    format_index: 1,
                    value: TimestampValue::Date(NaiveDate::from_ymd_opt(2022, 10, 27).unwrap()),
                }),
                filename: Some(" Some filename"),
                tags: None,
                extension: Some("pdf"),
            },
        },
        &normalised_path,
    );
}

#[test]
pub fn parse_path_with_secondary_timestamp_and_tags() {
    let config = create_configuration();
    let path = create_path().join("2022_10_27 Some filename -- tag test.pdf");
    let normalised_path = create_path().join("2022-10-27 Some filename -- tag test.pdf");

    check(
        StructuredPath {
            original_path: &path,
            name: StructuredName {
                configuration: &config,
                timestamp: Some(Timestamp {
                    configuration: &config.timestamp_configuration.date,
                    format_index: 1,
                    value: TimestampValue::Date(NaiveDate::from_ymd_opt(2022, 10, 27).unwrap()),
                }),
                filename: Some(" Some filename"),
                tags: Some(Tags::Tags(vec![
                    Tag {
                        separator: " -- ",
                        tag: "tag",
                    },
                    Tag {
                        separator: " ",
                        tag: "test",
                    },
                ])),
                extension: Some("pdf"),
            },
        },
        &normalised_path,
    );
}
