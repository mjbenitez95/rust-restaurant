use std::collections::HashMap;
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result {
    // --snip--
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Hello, world!");
}
