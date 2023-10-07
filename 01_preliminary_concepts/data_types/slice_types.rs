fn main() {
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    let slice: &[i32] = &numbers[1..4];
    for num in slice {
        println!("Number: {}", num);
    }
}
