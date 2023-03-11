use std::process;
use std::time::Instant;

const TESTS: usize = 1_000_000_0; 

struct Person {
    age: u32
}
impl Person {
    fn new(n: u32) -> Person {
        return Person {age: n};
    }
}

fn fill_mem(array: &mut Vec<Person>) {
    for i in 0..TESTS {
        array.push(Person::new(i as u32));
    }
} 

fn main() {
    let pid: u32 = process::id();
    println!("Rust pid: {pid}");
    
    loop {
        let start = Instant::now();

        let mut array: Vec<Person> = Vec::with_capacity(TESTS);
        fill_mem(&mut array);

        let duration: f64 = start.elapsed().as_nanos() as f64 / 100_000_000_0.0;
        println!("{duration:.5} sec");
    }
}

