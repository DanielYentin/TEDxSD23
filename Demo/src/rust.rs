// All other types (heap) are passed by reference

fn main() {
    let array: Vec<i32> = vec![9, 10, 11]; // 5
    print_array(&array); // 6
    print_array(&array); // 7
    // update_heap(&mut var_heap); // 8
}

fn print_array(array: &Vec<i32>) {
    println!("Heap = {:?}", array);
}

fn update_array(array: &mut Vec<i32>) {
    array.push(1);
}