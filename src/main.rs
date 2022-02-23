// mod hello;
// mod robot_ops;

// use hello::{world, whatup};

// use foo::test_hello_world;

use foo::calculator::*;
// use foo::order::domains::OrderRecord; // make domains public to use this
use foo::order::*;

extern crate test_crate_hello_world as ext_hello;
extern crate foo as f;

fn main() {
    // println!("Hello world")
    // test_hello_world();

    // world();
    // whatup();
    // foo::robot_ops::speak("hhe");
    // foo::
    // hello::blah::gotohell();
    // hello::testtwice();

    foo::calculator::pub_add(1, 2);
    
    f::test_hello_world();
    f::hello::blah::gotohell();
    let res = pub_add(1, 2);
    println!("{}", res);

    println!("{}", ext_hello::hello());

    

    // let orders = Vec<OrderRecord>();

    f::test_create_order();

    // f::collections::test_list_two();
    // f::boxes::test_boxes();

    let order = OrderRecord::new("Hello".to_string(), 12, 212212);
    println!("{:?}", order);

}