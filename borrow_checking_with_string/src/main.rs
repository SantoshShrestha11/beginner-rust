fn main() {
    let string1 = String::from("santosh");
    let string2 = "shrestha";

    let result = longest(string1.as_str(), string2);
    //here we are pasing the &str the string slice because we dont want to 
    //transfer the owner ship of the data which is string1
    println!("the longest string is : {result}");

}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }

}
