fn main() {
    println!("global: {}", std::env::var("GLOBAL").unwrap());
    println!("local: {}", std::env::var("LOCAL").unwrap());
}
