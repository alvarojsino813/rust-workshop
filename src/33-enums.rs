use std::fs::File;
use std::io::{self, Read};

fn read_file_contents(filename: &str) -> Result<String, io::Error> {

    let mut file = File::open(filename)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    return Ok(contents);
}

fn main() {
    match read_file_contents("example.txt") {
        Ok(contents) => println!("Contents of the file: {}", contents),
        Err(error) => eprintln!("Error reading file: {}", error),
    }
}
