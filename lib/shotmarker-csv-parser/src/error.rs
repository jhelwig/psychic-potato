#[remain::sorted]
#[derive(Debug, Clone, thiserror::Error)]
pub enum ShotMarkerCsvParserError {
    #[error("Invalid date format: year: {year}, month: {month}, day: {day}")]
    InvalidDate {
        year:  i32,
        month: u32,
        day:   u32,
    },
    #[error(
        "Invalid time format: hours: {hours}, minutes: {minutes}, seconds: {seconds}, PM? {is_pm}"
    )]
    InvalidTime {
        hours:   u32,
        minutes: u32,
        seconds: u32,
        is_pm:   bool,
    },
    #[error("Unexpected number of shot strings: expected {expected}, found {found}")]
    UnexpectedStringCount {
        expected: usize,
        found:    usize,
    },
}
