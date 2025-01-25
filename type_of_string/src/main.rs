fn main() {
    // String literal (implicitly &str)
    let s1: &str = "Hello, world!"; 

    // String (owned string)
    let mut s2 = String::from("Hello, Rust!"); 
    // Concatenation
    s2.push('!'); 
    //here we are appending just one character so no need to use the push_str() 


    // Borrowed string slice from String
    let s3: &str = &s2; 

    // Sliced string from String
    let s4: &str = &s2[0..5]; 

        // Printing
    println!("String literal: {}", s1); 
    println!("Owned string: {}", s2); 
    println!("Borrowed slice: {}", s3); 
    println!("Sliced string: {}", s4); 
}
