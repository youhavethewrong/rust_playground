extern crate adder;

mod common;

// run tests only in this suite by executing
// `cargo test --test integration_test`

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
