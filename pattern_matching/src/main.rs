fn main() {

    let v: Vec<char> = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("before {} is at index {}", value, index);
    }
    //this proves that the enumerate() method first returns teh index and
    //then the value of that index you can always gd
    for (value, index) in v.iter().enumerate() {
        println!("after {} is at index {}", value, index);
    }
}
