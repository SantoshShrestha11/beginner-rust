mod front_of_house { // we should use the mod to declare a module
    // and we should use the pub mod to make the module public
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
        
        pub fn seat_at_table() {
            println!("Seated at table");
        }
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_pament() {}
    }
}
