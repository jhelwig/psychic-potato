use super::*;

use std::time::Instant;

#[test]
fn shot_string_metrics_parser_valid_input() {
    let input = ",,,,avg:,1.0,2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,\n,,,,SD:,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0,1.1,1.2,1.3,\n";
    let result = shot_string_metrics_parser(input);
    assert!(result.is_ok());
    let (rest, metrics) = result.unwrap();
    assert_eq!(rest, "");
    assert!(matches!(metrics, ShotMarkerStringMetrics {}));
}

#[test]
fn export_parser_invalid_date() {
    let input = "ShotMarker Archived Data (generated Invalid 32, 2023)\nExported 0 string from Jan 1 of 100 total in archive\n";
    let result = export_parser(input);
    assert!(result.is_err());
}

#[test]
fn export_parser_minimum_info() {
    let input = "ShotMarker Archived Data (generated Jan 1 2023)\nExported 0 string from Jan 1 of 1 total in archive\n";
    let result = export_parser(input);
    assert!(result.is_ok());
    let (rest, export) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(export.generated_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert_eq!(export.string_count, 0);
    assert_eq!(export.string_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert!(export.strings.is_empty());
}

#[test]
fn export_parser_extra_spaces() {
    let input = "ShotMarker Archived Data (generated  Jan  1  2023  )\nExported  0  strings  from  Jan  1  of  100  total in archive\n";
    let result = export_parser(input);
    assert!(result.is_ok());
    let (rest, export) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(export.generated_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert_eq!(export.string_count, 0);
    assert_eq!(export.string_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert!(export.strings.is_empty());
}

#[test]
fn export_parser_incomplete_header() {
    let input =
        "ShotMarker Archived Data (generated Jan 1 2023)\nExported 0 string from Jan 1 of 100\n";
    let result = export_parser(input);
    assert!(result.is_err());
}

#[test]
fn export_parser_lowercase_input() {
    let input = "shotmarker archived data (generated jan 1 2023)\nexported 0 strings from jan 1 of 50 total in archive\n";
    let result = export_parser(input);
    assert!(result.is_ok());
    let (rest, export) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(export.generated_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert_eq!(export.string_count, 0);
    assert_eq!(export.string_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert!(export.strings.is_empty());
}

#[test]
fn export_parser_different_dates() {
    let input = "ShotMarker Archived Data (generated Dec 31 2023)\nExported 0 strings from Jan 1 of 100 total in archive\n";
    let result = export_parser(input);
    assert!(result.is_ok());
    let (rest, export) = result.unwrap();
    assert_eq!(rest, "");
    assert_eq!(export.generated_date, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    assert_eq!(export.string_count, 0);
    assert_eq!(export.string_date, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    assert!(export.strings.is_empty());
}

#[test]
fn example_files() {
    let example_file_contents = [
        include_str!("../examples/SM_shotslog_Jan_07.csv"),
        include_str!("../examples/SM_shotslog_Jan_14.csv"),
        include_str!("../examples/SM_shotslog_Jan_21.csv"),
        include_str!("../examples/SM_shotslog_Jan_28.csv"),
    ];
    for (idx, example_file_content) in example_file_contents.iter().enumerate() {
        let start_time = Instant::now();
        let result = export_parser(example_file_content);
        let elapsed_time = start_time.elapsed();
        println!("Example file {idx} parsed in {elapsed_time:?}");

        // Ensure parsing was successful and there were no remaining characters after parsing.
        assert!(
            result.is_ok(),
            "Failed to parse example file {idx}: source: {:?}, error: {:?}",
            std::error::Error::source(&result.clone().unwrap_err()),
            result.unwrap_err(),
        );
        let (rest, export) = result.unwrap();
        assert!(rest.is_empty(), "Non-empty rest after parsing example file {idx}: {:?}", rest);
        assert!(
            !export.strings.is_empty(),
            "No strings found in example file {idx}: {}",
            example_file_content
        );
    }
}
