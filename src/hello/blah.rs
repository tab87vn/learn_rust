pub fn gotohell() {
    println!("hey, go to hell");
}

pub fn testtwice() {
    println!("Test twice!");
}

pub fn return_hundred() -> i32 {
    100
}

#[test]
fn test_return_hundred() {
    assert_eq!(return_hundred(), 100);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_hundred() {
        assert_eq!(return_hundred(), 100);
    }
    
    
}