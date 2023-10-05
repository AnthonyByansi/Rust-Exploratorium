use std::fs::File;
use std::io::{Error, Read};

fn main() {
    // Attempt to open a file
    match open_file("file.txt") {
        Ok(file) => {
            // File opened successfully, read its content
            match read_file_content(&file) {
                Ok(content) => {
                    // Successfully read content
                    println!("File content: {}", content);
                }
                Err(err) => {
                    // Handle read error
                    eprintln!("Error reading file: {}", err);
                }
            }
        }
        Err(err) => {
            // Handle open file error
            eprintln!("Error opening file: {}", err);
        }
    }

    // Using `unwrap` to handle errors (not recommended in production)
    let content = read_file_content(&open_file("file.txt").unwrap());
    println!("File content: {}", content.unwrap_or_else(|err| format!("Error: {}", err)));

    // Using `expect` with a custom error message
    let content = read_file_content(&open_file("file.txt").expect("Failed to open the file."));
    println!("File content: {}", content.expect("Failed to read the file."));
}

fn open_file(filename: &str) -> Result<File, Error> {
    // Attempt to open a file
    File::open(filename)
}

fn read_file_content(file: &File) -> Result<String, Error> {
    let mut content = String::new();
    // Attempt to read file content
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(err) => Err(err),
    }
}
