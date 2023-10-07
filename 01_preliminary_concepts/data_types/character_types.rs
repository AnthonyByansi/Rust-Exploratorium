fn main() {
    // Character types 
    let heart: char = '❤';
    println!("I {} Rust!", heart);
    
    // Character Escapes
    let newline = '\n';
    let tab = '\t';
    
    // Character Methods
    /*
    `is_digit()`: checks if the character is a digit
    `is_alphabetic()`: checks if the character is alphabetic
    `is_whitespace()`: checks if the character is a whitespace
    `to_lowercase()`: coverts the character to lowercase
    `to_uppercase()`: Coverts the character to Uppercase
    
    */
    
    let c = 'A';
    let is_digit = c.is_digit(10); // false
    let is_alphabetic = c.is_alphabetic(); // true

 // Pattern matching with Characters
 
 let a = '3';
 match a{
     '0'..='9' => println("Digit: {}",a),
     _=> println("Not a digit"),
 }
 
 // Sting Manipulation => allows you ti iterate over the characters in a string using methods like `chars()`
 let my_string = "Hello, Rust!";
 for c in my_string.chars() {
    println!("Character: {}", c);
}


// String Slices

let text = "Hello, Rust!";
let slice = &text[0..5]; // "Hello"

// String Interpolation
let name = "Anthony";
let message = format!("Hello, {}!", name);


}
