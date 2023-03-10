use std::process;

fn main() {
    let pid: u32 = process::id();
    println!("Rust pid: {pid}");
    
    // get the current time in microseconds, annotate the type
}

// make a struct called person that has an age

struct Person {
    age: u32,
}
