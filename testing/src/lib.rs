use core::panic;

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess { 
        if value<1 {
            panic!("the guessing value should be greater of equal to 1");
        }else if value>100 {
            panic!("the guessing value shoud be less or equal to 100");
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
// here test should panic if the test does panics it will give ok
// if it does not panics the should_panic we will fail the test because of the should panic attribute we have used below
    #[test]
    // the panic message in the should_panic attribute should be same as the panic message up side
    #[should_panic(expected= "the guessing value shoud be less or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
