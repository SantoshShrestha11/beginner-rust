#![allow(unused)]
fn main() {
    let mut num = 5;
//creating immutable and a mutable raw pointer form references.
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //we cant derefference the raw pointer in hte safe block


    unsafe {
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }


    let address = 0x012345usize;
    let r = address as *const i32;

    //we should always call unsafe function in the unsafe block 
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }

{
    
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    }


use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

    {

        //in rust global variables are called static variables
        // here static variable is global variable  whose name should be in the uppercase for
        // constants
        static HELLO_WORLD: &str = "hello world";
        unsafe {

            println!("name is : {HELLO_WORLD}");
        }

    }
     {
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

    }
     

{
        static mut COUNTER:  u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe{
                COUNTER += inc;
            }
        }
        add_to_count(3);
        unsafe {
            println!("COUNTER:{}",COUNTER);
        }
    }
}
