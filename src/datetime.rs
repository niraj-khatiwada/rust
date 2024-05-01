use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

// Naive date means date with format 2024-04-20T23:04:59 without timezone
fn main() {
    let naive_date = NaiveDate::from_ymd_opt(2024, 4, 20).unwrap();
    let naive_time = NaiveTime::from_hms_opt(23, 4, 59).unwrap();
    println!("{:?}", NaiveDateTime::new(naive_date, naive_time)); // This will return "2024-04-20T23:04:59"

    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

    let dt = NaiveDateTime::new(d, t);
    println!("{}", dt);
    println!(
        "{}",
        NaiveDateTime::parse_from_str("2024-05-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
    );
    println!(
        "{}",
        NaiveDateTime::from_str("2024-05-01T16:54:05.372").unwrap()
    );
}
