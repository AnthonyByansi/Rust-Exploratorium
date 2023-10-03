// ownership_borrowing.rs

fn main() {
    // Ownership and Borrowing in Rust

    // 1. Ownership
    // - Ownership rules help manage memory and prevent data races.
    // - Each value in Rust has a variable that's its "owner." ANd there can be one owner at a time
    // - When the owner goes out of scope, Rust automatically deallocates the memory.

    // Ownership example
    let s1 = String::from("Hello"); // s1 is the owner of the String "Hello"
    let s2 = s1; // Ownership of the String is moved from s1 to s2
    // The next line would result in an error because s1 is no longer valid.
    // println!("s1: {}", s1);

    // Move Semantics
    // - When s1 is assigned to s2, ownership is moved, and s1 is no longer valid.
    let s2 = s1;

    // 2. Borrowing
    // - You can borrow references to values without transferring ownership.
    // - Borrowing can be either mutable or immutable.
    // - Immutable references allow multiple readers at the same time.
    // - Mutable references ensure exclusive access and prevent data races.

    // Borrowing example
    let s3 = String::from("Hello");
    let len = calculate_length(&s3); // Borrow an immutable reference to s3
    println!("Length of '{}' is: {}", s3, len);

    // 3. Mutable Borrowing
    // - Mutable references allow changing the borrowed value.
    // - But only one mutable reference is allowed in a scope to prevent data races.

    let mut s4 = String::from("Rust");
    change_string(&mut s4); // Borrow a mutable reference to s4
    println!("Changed string: {}", s4);

    // 4. Slices
    // - Slices are references to a portion of a data structure.
    // - Useful for working with parts of strings, arrays, or vectors.

    let my_string = String::from("Rust is awesome");
    let word = first_word(&my_string); // Slice the first word
    println!("The first word is: {}", word);

    // 5. Dangling References
    // - Rust ensures references are always valid.
    // - Prevents the use of references to values that no longer exist.

    // Uncommenting the next line would result in a compilation error.
    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change_string(s: &mut String) {
    s.push_str(" is powerful");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// This function would create a dangling reference and is commented out.
// fn dangle() -> &String {
//     let s = String::from("I will dangle!");
//     &s
// }
