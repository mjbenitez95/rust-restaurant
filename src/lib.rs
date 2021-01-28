/*
    The module tree for this structure is:

    crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
        │   ├── take_order
        │   ├── serve_order
        │   └── take_payment
        └── eat_at_restaurant()
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // here, we use super because we expect the structure between
        // the module back_of_house and the function serve_order()
        // won't change
        super::serve_order();
    }

    fn cook_order() {}
}

fn serve_order() {}

// we use the 'use' keyword with an absolute path
pub use crate::front_of_house::hosting;

// alternatively, we can use a relative path:
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // with the use key word above, we have brought the hosting module into scope
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like to change it to {} toast please!", meal.toast);

    // next line is invalid because seasonal fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    let order_for_table_6 = back_of_house::Appetizer::Soup;
    let order_for_table_2 = back_of_house::Appetizer::Salad;
}
