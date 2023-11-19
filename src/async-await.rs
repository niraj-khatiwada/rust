#[tokio::main]
async fn main() {
    let a = age().await;
    println!("{}", a)
}


async fn age() -> u8 {
    26
}