use unicode_segmentation::UnicodeSegmentation;

fn to_pig_latin(input: &str) -> String{
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut result = String::new();

    for word in input.split_whitespace() {
        let first_char = word.chars().next().unwrap();
        if vowels.contains(&first_char) {
            result.push_str(&format!("{word}-hay"));
        } else {
            let mut chars = word.graphemes(true);
            let first_cluster = chars.next().unwrap();
            let rest: String = chars.collect();
            result.push_str(&format!("{rest}-{first_cluster}ay "));
        }
    }
    result.trim_end().to_string()
}
fn main() {
    let sentence = "Hello there , My name is Santosh Shrestha.";
    let pig_latin_sentence = to_pig_latin(sentence);
    println!("Pig latin: {}", pig_latin_sentence);
}
