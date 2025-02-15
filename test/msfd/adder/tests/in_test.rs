use adder;
mod common;
#[test]
fn test_add() {
    common::setup();
    assert_eq!(adder::add_two(3), 5);
}
