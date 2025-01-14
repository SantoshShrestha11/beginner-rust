#[allow(dead_code)]
fn serve_customer() {
    println!("Serving customer...");
}
mod front_of_house; 
mod back_of_house; 
// A public function using all components
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::serve();

    // Using a public struct with a private field
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat"); // Allowed
    println!("I'd like {} toast, please.", meal.toast);

    // Uncommenting the line below will cause a compilation error because
    // 'seasonal_fruit' is private
    // meal.seasonal_fruit = String::from("Blueberries");

    // Using a public enum
    let order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    match order1 {
        back_of_house::Appetizer::Soup => println!("Ordered soup"),
        back_of_house::Appetizer::Salad => println!("Ordered salad"),
    }
}
