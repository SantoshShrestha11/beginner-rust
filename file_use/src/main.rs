use std::{fs::{File, OpenOptions}, io::{self, Read, Write}};
fn main() -> io::Result<()> {
    println!("Enter text to append:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;
    append_to_file("test_file.txt", &input)?;

    let file_contents = read_file_content("test_file.txt")?;
    println!("File contents;\n{}", file_contents);

    Ok(())
}
fn append_to_file(filename: &str, input: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(input.as_bytes())?;

    Ok(())
}
fn read_file_content(filename: &str) ->io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)

} 
