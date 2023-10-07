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
    
    
}
