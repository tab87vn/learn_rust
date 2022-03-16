// use robot_ops::robot_ops::speak;

pub mod robot_ops;
pub mod hello;
pub mod order;
pub mod collections;
pub mod boxes;
pub mod concurrency;
pub mod file;
mod traits;

pub fn test_hello_world() {
    hello::world();
    robot_ops::speak(&String::from("heheh, hello!!!"));
}

pub fn test_create_order() {
    let order = order::place_new_order("Test", 10);
}

pub mod calculator {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn pub_add(a: i32, b: i32) -> i32 {
        add(a, b)
    }

    #[cfg(test)]
    mod tests {
        // use crate::calculator;
        use super::*;
    
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    
        #[test]
        fn calculator_add_test() {
            assert_eq!(add(1, 2), 3)
        }
    
        #[test]
        fn calculator_multiply_test() {
            assert_eq!(multiply(2, 2), 4)
        }
    }
}