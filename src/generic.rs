use std::ops::Add;

pub fn run() {
    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.6 = {}", get_sum_gen(5.2, 4.6));
}

fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}
