//#[derive{Debug}]// this is the debug trait here and very use full for the debugging 
//it can be called using the {:?} and {:#?}  the second one gives the output in the prety-printed
//debug output
// fn main() {
//     let some_val: Option<i32> = Some(5);
//     let no_val: Option<i32> = None;
//
//     match some_val {
//         Some(val) => println!("found a value here {}",val),
//     None => println!("cound not found a val here"),
// }
//
//
// match no_val{
//     Some(val) => println!("found a value here {}",val),
// None => println!("cound not found a val here"),
// }
// }

fn main() {
    let value: Option<i32> = Some(60);
    println!("the value here is {}",value.unwrap());
    
//    let no_value: Option<i32> = None;
 //   println!("the val here is {}", no_value.unwrap()); //here this line will activate the panci
    //mode at the run time so this mode .unwrap is not suggested to use. 
    //we can use unwrap_or(default) function here.
     let no_value: Option<String> = None;
     println!("the val here is {:#?}", no_value.unwrap_or(String::from("santosh"))); //here this line will activate the panci
    // this is how we can use the unwrap_or(default) functon and the value in place of the default
    // should be the the type that was introduced in the declaration of the function.

}
