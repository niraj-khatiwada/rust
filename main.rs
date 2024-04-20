use std::str::FromStr;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

// Naive date means date with format 2024-04-20T23:04:59 without timezone
fn main() {
    // let naive_date = NaiveDate::from_ymd_opt(2024, 4, 20).unwrap();
    // let naive_time = NaiveTime::from_hms_opt(23, 4, 59).unwrap();
    // println!("{:?}", NaiveDateTime::new(naive_date, naive_time)); // This will return "2024-04-20T23:04:59"

    let d: DateTime<Utc> = DateTime::from_str("2024-04-19T00:00:00Z").unwrap();

    println!("{:?}", d)
}
