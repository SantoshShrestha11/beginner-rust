fn composing_using_push(input: &str) -> String {
    let mut s = input.to_string();
    //here .to_string will not change the original value so we have to store it in a new variable.
    s.push_str(" end");
    // the push_str will not return any type or change any type but the value passing in it,
    // that should be in the String type so we should convert it into String type
    s
}
fn composing_using_macro(input: &str) -> String {
    format!("{} end",input)
}
fn main() {
    let a = "hello there this is the string sample";
    let b = "this is antother sample";
    println!("here :{}",composing_using_push(a));
    println!("here :{}",composing_using_macro(b));


}
