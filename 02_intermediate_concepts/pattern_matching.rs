fn main(){
    // Pattern Matching in Rust: it used in `match` expressions, `if let` expressions, and function arguments
     
     // It is important to note also that patterns are used to destructure and match values
     // They consit of various elements like literals, variables, wildcards, enums, structs etc 
     
    let x = 5;
    
    // Using Range operators
    
    match x {
        1..=5 => println!("Between 1 and 5 (inclusive)"),
        6..=10 => println!("Between 6 and 9 (Exclusive)"),
        _=> println!("Didn't match any range"),
    }
    
    // Using the or operator (`|`). It allows you match a value against multiple patterns in a single Search
    
    let y: char = 'M';
    
    match y {
        'A' | 'E' | 'I' | 'O' | 'U' => println!("Vowel"),
        _ => println!("Constant"),
    }
    
    // The wildcard (`_`) helps to match any value. It is used as a Catch-all pattern when you want to handle all other cases.
    
    // Variable Binding- To bind the matched value to avariable for later use
    
    let z: Option<i32> = Some(5); // rember that to use some and None, You need to Use Option Types 
    
    match z {
        Some(val) => println!("Got a Value: {}", val),
        None => println!("No Value"),
    }
    
    // Reference Operator (`&`), When Matching references, you can use the `&` operator to match by reference
    
    Let m: &i32= &6;
    
    match m{
        
    &val => println!("Matched by reference: {}", val),
    }
    
    // Deconstructing structs and Enums
}
