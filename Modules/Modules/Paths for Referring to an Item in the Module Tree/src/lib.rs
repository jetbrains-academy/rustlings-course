mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path: ERROR!!! access to private components
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: ERROR!!! access to private components
    front_of_house::hosting::add_to_waitlist();
}