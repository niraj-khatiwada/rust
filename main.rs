use chrono::Datelike;
use chrono::Local;
use chrono::Utc;

fn main() {
    println!("UTC Time: {:?}", Utc::now());
    println!(
        "Local Time: {:?}. Year={} | Month={} | Day={}",
        Local::now(),
        Local::now().year(),
        Local::now().month(),
        Local::now().day()
    );
    println!(
        "Formatted Date {}",
        Local::now().format("%Y/%m/%d %H-%M-%S").to_string()
    )
}
