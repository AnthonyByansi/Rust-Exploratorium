// error_handling.rs

use std::fs::File;
use std::io::error;

fn main() -> Result<(), Error> {
    let file = File::open("file.txt")?;
    // Other file operations...
    Ok(())
}