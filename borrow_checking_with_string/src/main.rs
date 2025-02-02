fn main() {
    let string1 = String::from("santosh");
    let string2 = "shrestha";

    let result = longest(string1.as_str(), string2);
    //here we are pasing the &str the string slice because we dont want to 
    //transfer the owner ship of the data which is string1
    println!("the longest string is : {result}");

    let string3 = String::from("Sanish");
    let string4 = String::from("Gyawali");
    
    let result2 = shortest(string3.as_str(), string4.as_str());
    println!("the shortest string is : {result2}");
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }

}
fn shortest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() < y.len() {
        x
    }else{
        y
    }
}
