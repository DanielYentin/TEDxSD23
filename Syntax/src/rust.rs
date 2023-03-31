fn main() {
    let a: i32 = 0;
    let b: i32 = 10;
    let sum: i32 = sum_evens6(a, b);
    println!("Rust: sum of evens from {a} to {b} is {sum}");
}

fn sum_evens1(a: i32, b: i32) -> i32 {
    // direct c translation
    if a <= b {
        let mut sum: i32 = 0;
        for i in a..=b { // What is a..=b?  
            if i % 2 == 0 {
                sum += i;
            }
        }
        return sum;
    } 
    else {
        return 0;
    }
}

fn sum_evens2(a: i32, b: i32) -> i32 {
    // a..=b is a range, from a to b inclusive.
    // if a > b, the range is empty, therefore sum is 0.
    let mut sum = 0;
    for i in a..b+1 { 
        if i % 2 == 0 {
            sum += i;
        }
    }
    return sum;
}

fn sum_evens3(a: i32, b: i32) -> i32 {
    // i % 2 == 0 is a filter, lets use the filter method
    let mut sum = 0;
    for i in (a..=b).filter(|e| e % 2 == 0) { 
        sum += i;
    }
    return sum;
}

fn sum_evens4(a: i32, b: i32) -> i32 {
    // sum += i is a summation, lets use the sum method
    let sum = (a..b+1).filter(|e| e % 2 == 0).sum();
    return sum;
}

fn sum_evens5(a: i32, b: i32) -> i32 {
    // more straight forward
    return (a..b+1).filter(|e| e % 2 == 0).sum();
    
}

fn sum_evens6(a: i32, b: i32) -> i32 {
    // using "correct" rust syntax:
    // 1. no need for return (must omit semicolon at end of line)
    // 2. chain methods on new lines 
    (a..b+1)
        .filter(|e| e % 2 == 0)
        .sum()
}