fn main(){
    // Floating-Point types
    /*
    
    let x: f32 = 3.14;
    let y: f64 = 2.71828;
    let product = x * y;
    println!("Product: {}", product);
    
    */
    
    // You can''t directly perform arithmetic Operations between values of different floating-point precisions without explicit Conversion
    
    let x: f32 = 3.14;
    let y: f64 = 2.71828;
    let product = x as f64 * y; // Convert 'x' to f64 before multiplication
    println!("Product: {}", product);
    
}
