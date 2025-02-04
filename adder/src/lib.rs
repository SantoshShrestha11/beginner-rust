pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
#[derive(Debug)]
// #[derive(Debug, PartialEq)]
// we have to use these lines when we deal with the  assert_eq and assert_ne 
// while usign the struct and enums because they are deriveable traits 
struct Rectangle {
    width: u32,
    height: u32,
}
#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn another() {
        panic!("make this test fail");
    }
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width:5,
        height:1,
    };
    assert!(larger.can_hold(&smaller));
}
#[test]
fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width:5,
        height:1,
    };
    assert!(!smaller.can_hold(&larger));
}
#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}
