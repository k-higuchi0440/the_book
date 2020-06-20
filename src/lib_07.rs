#[allow(dead_code)]
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

// abloute path
// use crate::front_of_house::hosting;

// relative path and re-export
pub use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path witout `use`
    // create::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
