use std::io;
fn main() -> io::Result<()>{
    let texts: &str = "Hello there,\n My name is Santosh Shrestha. \n I am currently learning some of of the string operations. \n Are you getting something.";
    
    if let Some(last_char) = last_char_of_first_line(texts) {
        println!("The last character of the first line is : '{}'",last_char);
    } else {
        println!("No lines found in the text.");
    }
    Ok(())
}
fn last_char_of_first_line(text: &str) -> Option<char> {
    text
        .lines() //this will make the lines iterable for the place of the \n in the sentence
// The ? operator is applied to the Option returned by .next().
// It does the following:
    // If .next() returns Some(value), the value (the first line as &str) is unwrapped, and the program continues.
    // If .next() returns None, the entire function or expression will return None immediately.
        .next()? //it is used with the iterator to get the next item form the iterator
    //each time we call the .next() , the iterator moves forward and give you the next value and
    //if there are no more items left, it returs None.
        .chars()// like the .lines() it will make the character iterable
        .last()// this will just take the last iterable value in this case the last char.
    //
}
