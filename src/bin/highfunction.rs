
fn inc(n: i32) -> i32 {
    n + 1
}

type IncType = fn(i32) -> i32;

fn main() {
    let func: IncType = inc;
    println!("3 + 1 = {}", func(3));
    println!("3 + 1 = {}", inc(3));
}