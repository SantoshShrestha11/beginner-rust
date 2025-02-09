fn main() {
    // creating a new , empty string
    let s = String::new();
    println!("Empty string: {:?}", s);

    //creating a string form a string literal using to_string and 
    //using 'to_string' and String:form
    //this is the hard coaded value of the data given below and here the type is the string
    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = String::from("Initial contents");
    println!("String created with to_string: {}", s1);
    println!("String created with String::from: {}", s2);

    // Storing greetings in different language
    let hello = String::from("こんにちは"); //hello in japanese
    println!("Greeting in japanese: {}", hello);
    // strings are stored in the UTF - 8 or 16 
    // so any things that are in the format can we used here

    //appending to a sting using push_str and push 
    let mut s3 = String::from("foo");
    s3.push_str("bar");
    println!("After push_str: {}", s3);

    // concatenating string with + operator
    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; //note that :s4 is moved here
    // the thing here is the s5 is added to the s4 in s4 and then moved to the s6
    println!("After + operator: {}", s6);

    //using 'format!' macro for complex concatenations
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");
    let result = format!("{s7}-{s8}-{s9}");
    println!("Using format! macro: {}", result);

    //UTF-8 internal representation and lsicing 
    let hindi = "नमस्ते";
    println!("UTF-8 bytes: {:?}", hindi.as_bytes());
    println!("unixode scalar values: {:?}", hindi.chars().collect::<Vec<_>>());

    //slicing a valid porting of a UTF-8 string
    let slice = &hindi[0..6];//"नम"
    println!("Sliced string: {}", slice);

    //handeling potentioal runtime errrors in slicing
    //uncommenting the line below will panic at runtime
    //let invalid_slice = &hindi[0..1];

    if let Some(first_char) = hindi.chars().last() {
        //here the hindi.char() modes creates an iterator ove the unicode 
        //sclar values of the string hindi
        //and the .nth() will return the elements at the specified zero-based index.
        // incase of the .nth(0) this will retrives the first character of the string,
        // if the string is empty then,
        //it will return the None 
        println!("First character: {}", first_char);
    } else {
        println!("No first character found.");
    }
}
