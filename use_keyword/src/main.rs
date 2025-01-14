// use std::io;
// use std::io::Write;
//we can use the nested path like this way 
//use std::io{self, wirte};
/* use use_keyword::customer::{self, eat_at_restaurant}; */
// use use_keyword::*;
// we can use this * symbol to bring every pub this insider the use_keyword mod here
// use use_keyword::customer;
use use_keyword::customer::eat_at_restaurant;
fn main() {
    eat_at_restaurant();
}
