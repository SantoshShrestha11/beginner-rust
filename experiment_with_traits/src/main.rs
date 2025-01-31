//this is how you append some thing to a vector 
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut s = self;
        s.push("Bar".to_string());
        s
    }
}
//this is how you implement some of the appending things in each and evely element
//of the vector
trait AppendBarOnEveryElement{
    fn append_bar_in_every_element(self) -> Self;
}
impl AppendBarOnEveryElement for Vec<String> {
    fn append_bar_in_every_element(self) -> Self {
        let mut s = self;
        s.iter_mut().for_each(|s| s.push_str("bar"));
        s
    }

}
fn main() {
    let mut test = vec!["foo".to_string()].append_bar();
    println!("{:#?}",test);
    let mut test1 = vec!["foo".to_string(), "some".to_string(), "too".to_string()].append_bar_in_every_element();
    println!("{:#?}",test1);
// here the pop will return the Option<T> so we have to unwrap it
// the pop will just remove the last element 
//here you should unwrap is not it will print the Option<T> 
    println!("{:#?} is poped",test.pop());
    println!("{} is poped",test1.pop().unwrap());
}

