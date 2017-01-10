fn main() {
    println!("3 + 1 = {}", process(3, inc));
    println!("3 + 1 = {}", process(3, dec));
}


fn inc(n: i32) -> i32 {
    n + 1
}

fn dec(n: i32) -> i32 {
    n - 1 
}

fn process<F>(n: i32, func: F) -> i32 
    where F: Fn(i32) -> i32{
    func(n)
}