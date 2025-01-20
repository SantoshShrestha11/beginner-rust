use std::{fs::{File, OpenOptions}, io::{self, Read, Write}};
// here also we did some error handeling in the main function .
fn main() -> io::Result<()> {
    println!("Enter text to append:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;
    append_to_file("test_file.txt", &input)?;

    let file_contents = read_file_content("test_file.txt")?;
    //here we are using the  ? operator for the shortcuts of the error handeling with the match
    //statement 
    println!("File contents;\n{}", file_contents);

    Ok(())
}
//using alias of Result<(), io::Error> as io::Result<()>
fn append_to_file(filename: &str, input: &str) -> io::Result<()> {
    //here io::Result<()> is used because we are not returning the data
    //and just want to append some string in the data
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;
    file.write_all(input.as_bytes())?;

    Ok(())
}
//using alias of io::Result<String, io::Error>
fn read_file_content(filename: &str) ->io::Result<String> {
    // here io::Result<String> is used because we want to read the data in the string
    //and the data is meaning full
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)

} 
