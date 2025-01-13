fn serve_customer() {
    println!("Serving customer...");
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to wait list");
        }
        pub fn seat_at_table() {
            println!("Seated at table.");
        }
    }

    mod serving {
        pub fn take_order() {
            println!("Taking order.");
        }

        fn serve_order() {
            println!("Srving order");
        }

        pub fn take_payment() {
            println!("Payment received.");
        }

        pub fn notify_parent() {
            super::serve_customer();
        }//using teh super to call a function from the parent module
    }
    
    pub fn serve() {
        hosting::seat_at_table();
        serving::take_order();
        serving::take_payment();
        serving::notify_parent();
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfact {
        //public associated funciton to construct a breakfast instance
        pub fn summer(toast: &str)-> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
// A public funciton using all components
pub fn eat_at_resturant() {
    //absolute path
    crate::front_of_house::hosting::add_to_waitlist():

    //relative path
    front_of_house::serve();

    //using a public struct with a private field
    let mut meal = back_of_house::Breakfast::seasonal_fruit("Rye");
    meal.toast = String::from("wheat"); //allowed
    println!("I'd like {} toast, please.", meal.toast);

    //uncommenting the line below will caue a complicaton error because,
    //'seasonal_fruit' is private
    //mean.seasonal_fruit = String::from("blueberries");

    //using a public enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    match order1{
        back_of_house::Appetizer::Soup => println!("Ordered soup");
        back_of_house::Appetizer::Salad => println!("Ordered salad");
    }

}



