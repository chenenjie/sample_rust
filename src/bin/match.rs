

fn main() {
    let day = 5;

    match day {
        0 | 6 => println!("weekend "),
        a @ 1 ... 5 => println!("weekday {}", a),
        _ => println!("invalid"),
    }
}