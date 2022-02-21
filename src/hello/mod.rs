pub fn world() {
    println!("Aloha, world.");
}

pub fn whatup() {
    println!("what's up?");
}

pub mod blah;

pub fn gotohell() {
    blah::gotohell();
}

pub use self::blah::testtwice;