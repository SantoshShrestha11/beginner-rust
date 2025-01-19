use std::{fs::File, io::{self, Read}};
fn main() {
    match read_from_file() {
        Ok(contents) => println!("The contents of the files are here below :\n{}", contents),
        Err(error) => println!("Error reading file : {}", error),
        
    }
}
fn read_from_file() -> Result<String, io::Error> {
    let mut file_content = String::new();

    File::open("sample.txt")?.read_to_string(&mut file_content)?;

    Ok(file_content)
}

