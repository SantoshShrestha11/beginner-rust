fn main() {
    let mut string: String = String::from("hello this is santosh");
    string.push_str(&"shrestha".repeat(100));
    println!("{string}");

}
