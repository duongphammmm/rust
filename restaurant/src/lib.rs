mod front_of_house;

mod back_of_house {
    // public struct still needs to declare public field on case-by-case basis
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // need public fn summer to construct new breakfast (else, cannot set private seasonal_fruit)
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // public enum has automatically public variants
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// By convention, when bringing function into scope, bring the function's parent.
// When bringing structs, enums and other items into scope, bring the full item's path.
// The only exception is if two items from different paths have the same name, then 
// bring the items' parents instead to prevent ambiguity.
//
// Another way to import items with the same name:
use std::fmt::Result;
use std::io::Result as IoResult;

// Re-exporting names to external code
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
