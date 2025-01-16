fn main() {
    let sentence = "Hello there, I am Santosh Shrestha .";
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for word in sentence.split_whitespace() {
        if let some(first_char) = word.chars().next() {
            if vowels.contains(&first_char.to_ascii_lowercase()) {

            }
        }
    }

}
