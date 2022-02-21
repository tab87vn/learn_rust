mod Order;

pub fn place_new_order(name: &str, qty: i32) -> Order {
    let mut rng = rand::thread_rng();
    let order = Order {
        name,
        qty,
        id = rng.gen::<u64>()
    };

    println!("A new order has been placed!");    
}