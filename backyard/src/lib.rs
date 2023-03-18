pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn set_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// using use to create a shorter name

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // absolute path
        // crate::front_of_house::hosting::add_to_waitlist();
        hosting::add_to_waitlist();

        // relative path
        hosting::add_to_waitlist()
    }
}
