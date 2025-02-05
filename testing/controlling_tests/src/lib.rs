// $ cargo test -- --show-output
// the about used show output flag :-
// we can use this command to get the println inside the function even if the function passes the
// test    as I know that the test which passes , the resturn value will not show in the terminal
// while useing the test command but when we use the command mentioned above the command will print
// the thing the returning value of the function that passed too
//
//
// we can also use the test name to test the particulat test like here 
// if i want to test the this_test_will_fail test then I will use the following flag here 
// $ cargo test this_test_will_fail
#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(value, 10);
    }
    
    #[test]
    #[ignore]
    //we can use ignore, to ignore this function while using the test command 
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(value, 5);
    }
}
