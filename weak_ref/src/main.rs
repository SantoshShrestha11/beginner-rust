#![allow(unused)]
use std::rc::{Rc, Weak};
use std::cell::RefCell;


#[derive(Debug)]
struct Node{
    value: i32, 
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}
fn main() {
    let node1 = Rc::new(RefCell::new(Node{
        value: 1,
        next: None,
        prev: None
    }));

    let node2 = Rc::new(RefCell::new(Node{
        value: 2,
        next: None,
        prev: Some(Rc::downgrade(&node1)),
    }));

    node1.borrow_mut().next = Some(Rc::clone(&node2));

    println!("Node 1 strong count : {}", Rc::strong_count(&node1));
    println!("Node 2 strong count : {}", Rc::strong_count(&node2));
    println!("Node 2 weak count : {}", Rc::weak_count(&node2));

    if let Some(prev_node) = node2.borrow().prev.as_ref().and_then(|weak| weak.upgrade()) {
        println!("Node 2's previous node: {:?}", prev_node.borrow().value);
    }else {
        println!("Node 2's previous node has been dropped");
    };
}
