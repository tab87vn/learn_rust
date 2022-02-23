#[derive(Debug)]
pub struct OrderRecord {
    pub name: String,
    pub qty: i32,
    pub id: u64,
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
