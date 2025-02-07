pub fn capitalize_first(input: &str) -> String {
    let mut data: String = input.to_string(); 
    if !data.is_empty() {
        let first_char = data.remove(0);
        data.insert(0, first_char.to_ascii_uppercase());
    }
    data
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn testing_here() {
//         let some_string: &str = "here";
//
//         assert_eq!(capitalize_first(some_string), "Here".to_string());
//     }
// }
