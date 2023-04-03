fn main() {
	let mut a = 2;
	let b = 3;
	add_a_to_b(a, b);
	add_a_to_b(a, b);
}

fn add_a_to_b(mut a: i32, b: i32) {
	a = a + b;
	println!("{a}");
}
