struct Point {
    x: i64,
    y: i64,
}

fn main(){
    let point = Point {x: 0, y: 0};

    match point {
        //上下两种都可以
        // Point {x, y} => println!("({}, {})", x, y),
        //重命名的方式
        Point{x: x1, y: y1} => println!("({}, {})", x1, y1),
        //忽略x
        // Point{y, ..} => println!("y is {}", y);
    }
}