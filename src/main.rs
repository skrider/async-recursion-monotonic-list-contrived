#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let depth: i32 = args.nth(1).unwrap().parse().unwrap();
    println!("Hello, world! {}", depth);
}
