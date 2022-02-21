pub struct Order {
    name: &str,
    qty: i32,
    id: u64,
}

pub impl Order {
    pub fn get_details(&self) -> String {
        String::from("Name: {}; quantity: {}; ID: {}", 
            self.name, self.qty, self.id)
    }

    pub fn update_quality(&self, new_qty: i32) {
        self.qty = new_qty;
    }
}

#[cfg(Test)]
mod tests {
    #[test]
    fn get_details_test() {
        let order = new Order {
            name = "Hello",
            qty = 10,
            
        }

        assert_eq!()
    }
}