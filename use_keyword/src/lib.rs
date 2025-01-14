#[allow(dead_code)]
mod fornt_of_house {
    pub mod hosting{
        pub fn add_to_waitlist() {}
    }
}

#[allow(dead_code)]
pub mod customer {
use crate::fornt_of_house::hosting;
    //we can use this method and this method is called the idomatic method use paths 
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
        //super::fornt_of_house::hosting::add_to_waitlist();
        //this is the antoher method out there 
        //here using super hosing to refrence parent module
        //the second one is not good to use
    }
}
