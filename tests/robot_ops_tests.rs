// use robot_ops;
extern crate foo as f;

#[test]
fn give_double_test() {
    assert_eq!(f::robot_ops::give_double(10), 20);
}

#[test]
fn give_triple_test() {
    assert_eq!(f::robot_ops::give_triple(10), 30);
}
