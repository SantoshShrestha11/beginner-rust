use std::thread;
fn main() {
    let list = vec![1,2,3,4,5];
    println!("vals before the defination of the colosures {list:?}");

    let onlyborrow = || println!("form borrowing place {list:?}");

    onlyborrow();
   println!("after borrowing {list:?}");

     let mut list2 = vec![1,2,4,5,5,5];

    println!("vals before the defination of the colosures {list2:?}");
    let mut borrows_mutably = || list2.push(3);

    borrows_mutably();
    println!("after borrowing {list2:?}");


    // some thread test
    let list3 = vec![1,23,1,31,31,31,31,4];
    println!("before defining the colosures {list3:?}");

    thread::spawn(move || println!("form thread: {list3:?}"))
        .join()
        .unwrap();
    // println!("after moving the list3 to the colosures {list3:?}");
    // it will give us error because the value have already moved to the new place thread spawn
    // uing the move 
    //
    //
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    //I am inside the fn main already so continueing
    let mut list5 =[
        Rectangle {width: 10, height: 1},
        Rectangle {width: 11, height: 14},
        Rectangle {width: 104, height: 14},
        Rectangle {width: 101, height: 141},
        Rectangle {width: 103, height: 141},
    ];
    //here the function sort_by_key takes the reference of the list5 not the ownership 
    //so it mainly depends on the behaviour of the colosures
    
    list5.sort_by_key(|r| r.width);
    println!("{list5:#?}")

}
