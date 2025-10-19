use chrono::{Datelike, Local, NaiveDate, Weekday};
use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn link_to_float(hyperlink: &str) -> f64 {
    let Some(number) = extract_number(hyperlink) else {
        return 0.0;
    };
    number
}

fn extract_number(hyperlink: &str) -> Option<f64> {
    let re = Regex::new(r#",\s*""(\d+)""\)"#).unwrap();
    let cap = re.captures(hyperlink)?;
    cap.get(1)?.as_str().parse::<f64>().ok()
}

pub fn get_first_day_of_current_week() -> NaiveDate {
    let today = Local::now().naive_local().date();
    let weekday = today.weekday();

    // Calculate the number of days to subtract to get to Monday
    // Monday is 0, Tuesday is 1, ..., Sunday is 6
    let days_to_subtract = match weekday {
        Weekday::Mon => 0,
        Weekday::Tue => 1,
        Weekday::Wed => 2,
        Weekday::Thu => 3,
        Weekday::Fri => 4,
        Weekday::Sat => 5,
        Weekday::Sun => 6,
    };

    today - chrono::Duration::days(days_to_subtract as i64)
}
