#[allow(dead_code)]
pub fn add(a:i32, b:i32) -> i32 {
    a+b
}

#[cfg(test)]
mod tests{
    use super::* ;
    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result ==4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to fout"))
        }
    }
}
