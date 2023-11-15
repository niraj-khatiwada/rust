fn main() {
    println!("{}", chrono::DateTime::UNIX_EPOCH);
    println!("{}", chrono::Utc::now());
    println!("{}", chrono::Duration::seconds(100));
    println!("Current TS UTC: {:?}", chrono::Utc::now());
    println!("Current TS Local: {:?}", chrono::Local::now());
}


