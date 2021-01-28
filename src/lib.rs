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

fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
