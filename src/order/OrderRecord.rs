// #[derive(Debug)]
// pub struct OrderRecord {
//     name: String,
//     qty: i32,
//     id: u64,
// }

// pub impl OrderRecord {
//     // pub fn new(name: &str, qty: i32, id: u64) -> Self { Self { name, qty, id } }

//     pub fn get_details(&self)  {
        
//         // let str = concat!("Name: ", self.name, "; Quantity: ", self.qty);
//         // String::from(str)

//         println!("{:?}", &self);
//     }

//     pub fn update_quality(&self, new_qty: i32) {
//         self.qty = new_qty;
//     }

//     /// Get the order's name.
//     pub fn name(&self) -> String {
//         self.name
//     }

//     /// Get the order's qty.
//     pub fn qty(&self) -> i32 {
//         self.qty
//     }
// }

#[cfg(Test)]
mod tests {
    // #[test]
    // fn get_details_test() {
    //     let order = Order {
    //         name = "Hello",
    //         qty = 10,
    //         id = 1111_i64            
    //     };

    //     assert!(order.get_details().contains("Name: Hello; Quantity: 10;"))
    // }
}