fn main() {
    let greeting: &str = "Hello, ";
    let name: String = String::from("Alice");
    let message = greeting.to_string() + &name;
    println!("{}", message);
}
