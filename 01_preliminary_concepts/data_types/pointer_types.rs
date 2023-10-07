fn main() {
    let num: i32 = 42;
    let ptr: *const i32 = &num;
    unsafe {
        println!("Value via pointer: {}", *ptr);
    }
}
