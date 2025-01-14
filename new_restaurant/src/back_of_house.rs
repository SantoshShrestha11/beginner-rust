pub struct Breakfast {
    pub toast: String,
    _seasonal_fruit: String,
}

impl Breakfast {
    // Public associated function to construct a Breakfast instance
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            _seasonal_fruit: String::from("peaches"),
        }
    }
}

pub enum Appetizer {
    Soup,
    Salad,
}
