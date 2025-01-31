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
    let test = vec!["foo".to_string()].append_bar();
    println!("{:#?}",test);
    let test1 = vec!["foo".to_string(), "some".to_string(), "too".to_string()].append_bar_in_every_element();
    println!("{:#?}",test1);
}

