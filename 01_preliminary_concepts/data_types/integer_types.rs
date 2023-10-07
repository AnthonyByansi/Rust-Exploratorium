fn main(){
    // integer types
    
    /*
    let a: i32 = 42;
    let b: u64 = 70;
    let sum = a + b;
    
    println!("sum: {}", sum);
    */
    
    // The above code causes an error as you can not directly perform arithmetic operations between values of different types without explicit type convertion.
    // Mixing signed and unsigned integers in arithmetic operations requires us to handle `OVERFLOWS` and `UNDERFLOWS`
    
    let a: i32 = 42;
    let b: u64 = 70;
    let sum = a as u64 + b; // here we convert `a` to u64 before addition
    println!("Sum: {}", sum);
}
