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
    let weekday = WEEKDAYS[dt.weekday().num_days_from_sunday() as usize];
    let day = dt.day();
    let month = MONTHS[dt.month() as usize];
    let year = dt.year();
    let hour = dt.hour() + 3;
    let min = dt.minute();

    format!("{weekday}, {day} {month} {year} года, {hour:02}:{min:02}")
}
