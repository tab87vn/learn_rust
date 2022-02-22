
// use crate::order::OrderRecord::OrderRecord;
use rand::Rng; 

// use OrderRecord;

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

#[derive(Debug)]
pub struct OrderRecord {
    name: String,
    qty: i32,
    id: u64,
}

impl OrderRecord {
    // pub fn new(name: &str, qty: i32, id: u64) -> Self { Self { name, qty, id } }

    pub fn print_details(&self)  {
        println!("{:?}", &self);
    }
    
    pub fn get_details(&self) -> String {
        
        let mut res = String::from("Name: ");
        res.push_str(&self.name);
        res.push_str("; Quantity: ");
        res.push_str(&self.qty.to_string());

        res
    }

    // pub fn update_quality(&self, new_qty: i32) {
    //     self.qty = new_qty;
    // }

    // /// Get the order's name.
    // pub fn name(&self) -> String {
    //     self.name
    // }

    // /// Get the order's qty.
    // pub fn qty(&self) -> i32 {
    //     self.qty
    // }
}
