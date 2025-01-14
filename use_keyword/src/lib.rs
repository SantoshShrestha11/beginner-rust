#[allow(dead_code)]
mod fornt_of_house {
    pub mod hosting{
        pub fn add_to_waitlist() {}
    }
}

#[allow(dead_code)]
mod customer {
use crate::fornt_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
