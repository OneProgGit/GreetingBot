//! Used for formatting date

use chrono::{Datelike, NaiveDateTime, Timelike};

/// Defines all days of the week in American format.
const WEEKDAYS: [&str; 7] = [
    "воскресенье",
    "понедельник",
    "вторник",
    "среда",
    "четверг",
    "пятница",
    "суббота",
];

/// Defines all months. The 0th is empty, because there is no 0th month!
const MONTHS: [&str; 13] = [
    "",
    "января",
    "февраля",
    "марта",
    "апреля",
    "мая",
    "июня",
    "июля",
    "августа",
    "сентября",
    "октября",
    "ноября",
    "декабря",
];

/// Formats datetime from NaiveDataTime to human-readable format in Russian.
pub fn format_datetime_russian(dt: NaiveDateTime) -> String {
    log::info!("Formatting datetime to Russian...");
    let weekday = WEEKDAYS[dt.weekday().num_days_from_sunday() as usize];
    let day = dt.day();
    let month = MONTHS[dt.month() as usize];
    let year = dt.year();
    let hour = dt.hour() + 3;
    let min = dt.minute();

    format!("{weekday}, {day} {month} {year} года, {hour:02}:{min:02}")
}

#[cfg(test)]
mod date_tests {
    use chrono::NaiveDate;

    use crate::date::format_datetime_russian;

    #[test]
    fn test_format_datetime_russian() {
        let date = NaiveDate::from_ymd_opt(2026, 1, 15)
            .expect("Failed to create datetime")
            .and_hms_opt(3, 1, 0)
            .expect("Failed to add hours, minutes and seconds to date");
        assert_eq!(
            format_datetime_russian(date),
            "четверг, 15 января 2026 года, 06:01"
        )
    }
}
