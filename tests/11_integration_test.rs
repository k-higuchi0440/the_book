use the_book::lib_11;

mod common;

#[test]
fn add_two() {
    common::setup();
    assert_eq!(lib_11::add(2, 2), 4);
}
