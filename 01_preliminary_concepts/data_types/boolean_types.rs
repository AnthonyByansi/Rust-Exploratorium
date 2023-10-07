fn main(){
    // Boolean Types
    
    let is_rust_cool: bool = true;
    if is_rust_cool {
        println!("Rust is cool! 😍😘");
    }else{
        println!("Rust isn't cool 😒");
    }
    
    // boolean Expressions
    /*
    Boolean types are often used in conditional statements(Like `if`, `else`, `while`, `match` etc)
    to control the flow of te program based on conditions. They can include comparisons (e.g `==`, `!=`, `<`, `>`, `<=`, `>=`) and logical operators (e.g `&&` for AND, `||` for logical OR, `!` for logical NOT)
    
    */
    
    let x = 5;
    let y = 10;
    if x < y && x != 0 {
        // Rust gets Cool 👌
    }
    // Boolean Operators
    let is_sunny = true;
    let is_weekend = false;
    
    if is_sunny && !is_weekend {
        // This block gets executed as the conditions are true.
    }
    
    // Boolean Functions and Matches 
    
    let my_string = "Rust is Cool";
    let starts_with_rust = my_string.starts_with("Rust");
    // starts_with_hello will be true
    
    
    // pattern Matching with Booleans
    
    let is_rust_cool = true;
    match is_rust_cool {
        true => println!("Rust is cool 💕"),
        false => println!("Rust isn't cool nigga! 😒"),
    }
    /*
     Module Methods
     The `std::cmp` module proovides several methods for comparing values
    
    `min(a,b)`: Return the smaller of two values
    `min(a, b)`: Returns the smaller of two values
    `min_by(a,b, compare)`: Returns the smaller of two values using a custom comparison function
    `max_by(a, b, compare)`: Returns the larger of two values using a custom comparison Function
    
    */
    
    use std::cmp::{min, max, min_by};
    
    let x = 5;
    let y = 8;
    let smaller = min(x,y); // 5
    let larger = max(x,y); // 8 
    
    


    let a = 5;
    let b = 10;

    // Define a custom comparison function that compares the absolute values of numbers.
    fn compare_abs(a: &i32, b: &i32) -> std::cmp::Ordering {
        a.abs().cmp(&b.abs())
    }

    let smaller = min_by(a, b, compare_abs);

    println!("The smaller number (by absolute value) is: {}", smaller);
    
    // bool methods
    let is_true = true;
    let is_false = is_true.not(); // false
    
    // String Methods
    /*
    `Contains()`: Checks if a string contains a specified substring
    `starts_with()`: Checks if a string start with a specified prefix
    `ends_with()`: checks if ths strinf Ends with a specified suffix
    `is_empty`: Checks if the string is empty
    */
    
    let my_string = "Hello, world!";
    let contains_hello = my_string.contains("Hello"); // true
    let starts_with_hello = my_string.starts_with("Hello"); // true
    let is_empty = my_string.is_empty(); // false
    
    /*
    Options and Result Methods
    `Option` and `Reesult` types have methods to work with boolean conditions
    
    `Option::is_some()`: Checks is an `Option` is Some
    `Option::is_none()`: checks if an `option` is `None`.
    `Result::is_ok()`: Checks if a `result` is `Ok`.
    `Result::is_err()`: Checks if a `Result` is `Err`
    
    */
    
    let maybe_value: Option<i32> = Some(42);
    let is_some = maybe_value.is_some(); // true
    let is_none = maybe_value.is_none(); // false


}
