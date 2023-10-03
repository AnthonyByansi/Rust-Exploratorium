// variables.rs

fn main() {
    // Variable Declaration
    // In Rust, you declare variables using the `let` keyword.
    
    // Immutable Variable
    let x = 5;  // This variable is immutable by default.
    println!("Immutable variable x: {}", x);
    
    // Mutability
    // To make a variable mutable, use the `mut` keyword.
    
    let mut y = 10;  // This variable is mutable.
    println!("Original mutable variable y: {}", y);
    
    y = 15;  // You can change the value of a mutable variable.
    println!("Modified mutable variable y: {}", y);
    
    // Variable Shadowing
    // Rust allows you to shadow a variable by declaring a new one with the same name.
    
    
}

// variables.rs

fn main() {
    // Variables in Rust

    // 1. Declaration and Initialization
    // Declare a variable 'x' and initialize it with the value 5.
    let x = 5;

    // 2. Mutable Variables
    // Variables are immutable by default, but you can make them mutable using 'mut'.
    let mut y = 10;

    // 3. Printing Variables
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // 4. Variable Shadowing
    // You can redefine a variable in the same scope, effectively shadowing the previous one.
    let y = "I'm different now!";
    println!("The value of y is: {}", y);
     
         // Shadowing- Exampl2
    let z = "initial";  // Declare z with an initial value.
    println!("Original variable z: {}", z);
    
    let z = "shadowed";  // Shadow the variable z with a new value.
    println!("Shadowed variable z: {}", z);
    
    let z = z.len();  // Shadowing can also change the type of the variable.
    println!("Shadowed variable z (length): {}", z);


    // 5. Constants
    // Declare a constant with the 'const' keyword.
    const Z: i32 = 100;
    println!("The value of Z (a constant) is: {}", Z);

    // 6. Data Type Annotation
    // You can explicitly annotate the data type of a variable.
    let a: i64 = 1234567890;
    println!("The value of a (i64) is: {}", a);

    // 7. Type Inference
    // Rust can infer the data type from the value assigned.
    let b = 3.14; // 'b' is inferred as f64.
    println!("The value of b (f64) is: {}", b);

    // 8. Multiple Variables
    let (c, d) = (42, "Hello");
    println!("The values of c and d are: {} and {}", c, d);

    // 9. Underscores in Numeric Literals
    // Use underscores for readability in large numeric literals.
    let million = 1_000_000;
    println!("One million written with underscores: {}", million);
    
    // 10. Booleans
    let is_rust_cool = true;
    println!("Is Rust cool? {}", is_rust_cool);
    
    // 11. Characters
    let emoji = '😀';
    println!("An emoji: {}", emoji);
    
    // 12. Tuples
    let person = ("Alice", 30);
    println!("Name: {}, Age: {}", person.0, person.1);
}
