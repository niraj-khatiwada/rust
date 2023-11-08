use std::time::Duration;

use humantime::format_duration;

fn main() {
    let d = Duration::from_secs(9876);
    println!("{:?}", format_duration(d).to_string())
}






