fn hello(name: impl Into<String>) {
    println!("Hello, {}", name.into());
}

fn main() {
    hello("world");
    hello("world".to_string());
}
