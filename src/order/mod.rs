mod domains;

use rand::Rng; 
use domains::OrderRecord;

pub fn place_new_order(name: &str, qty: i32) -> OrderRecord {
    let mut rng = rand::thread_rng();
    let order = OrderRecord {
        name: name.to_string(),
        qty,
        id: rng.gen::<u64>()
    };

    println!("A new order has been placed!");
    
    let res = order.get_details();
    println!("{}", res);

    return order;
}