// extern crate foo;

use foo::calculator::*;

#[test]
fn calculator_add_integration_test() {
    assert_eq!(pub_add(1, 2), 3)
}