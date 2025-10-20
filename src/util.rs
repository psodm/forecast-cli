use chrono::{Datelike, Local, NaiveDate, Weekday};
use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;

pub fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
}

pub fn extract_number(hyperlink: &str) -> f64 {
    // match either ,""<hours>"" ) or ," <hours>" )
    let re = Regex::new(r#",\s*"{1,2}(\d+(?:\.\d+)?)"{1,2}\)"#).unwrap();
    if let Some(caps) = re.captures(hyperlink) {
        let hours_str = &caps[1];
        hours_str.parse::<f64>().unwrap_or(0.0)
    } else {
        0.0
    }
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
