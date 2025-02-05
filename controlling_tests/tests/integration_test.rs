//so to use the use command or what ever you say you just have to look the cargo.toml file there
//there is the name of the file or package which we should we use here
//[package]
// name = "controlling_tests"
// version = "0.1.0"
// edition = "2021"
// here in this case we use the name of the package and the thing that we want to use form 
// the library crate we dont have the binary crate here
//  We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. 
//  Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test
use controlling_tests::add_two;
mod common;
#[test]
fn it_adds_two(){
    common::setup();
    let result= add_two(2);
    assert_eq!(result,4);
}
