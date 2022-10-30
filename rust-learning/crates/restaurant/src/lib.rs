mod front_of_house;

pub use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
use std::collections::HashMap as StdHashMap;
// use std::io::{self, Write};
// use std::collections::*;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::BreakFast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();

    let mut map = StdHashMap::new();
    map.insert(1, 2);
}

fn _serve_order() {}

mod back_of_house {
    fn _fix_incorrect_order() {
        _cook_order();
        super::_serve_order();
    }

    fn _cook_order() {}

    pub struct BreakFast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}
