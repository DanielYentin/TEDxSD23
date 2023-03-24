use std::process;
use std::time::Instant;

const SIZE: usize = 10_000_000; 
const LOOPS: usize = 1_00;

struct Person {
    age: i32
}

fn fill_mem(array: &mut Vec<Person>) {
    for i in 0..array.capacity() {
        array.push(Person {age: i as i32});
    }
} 

fn update_mem(array: &mut Vec<Person>) {
    for i in 0..array.capacity() {
        array[i].age += 1;
    } 
}

fn main() {
    let pid: u32 = process::id();
    println!("Rust pid: {pid}");
    println!("Total iterations: {}", SIZE*LOOPS);
    
    loop {
        let start = Instant::now();

        //------------------------------------------------------
        let mut array: Vec<Person> = Vec::with_capacity(SIZE);
        fill_mem(&mut array);
        for _ in 0..LOOPS {
            update_mem(&mut array);
        }
        // NO FREE!!!
        //------------------------------------------------------

        let duration: f64 = start.elapsed().as_nanos() as f64 / 100_000_000_0.0;
        println!("{duration:.5}");
    }
}