#[derive(Default)]
struct Point3d{
    x: i32,
    y: i32,
    z: i32,
}


fn main(){
    let origin = Point3d::default();

    let point = Point3d {
        y: 1, ..origin
    };

    let Point3d{x: x0, y: y0, ..} = point;

    println!("{}", x0);
    println!("{}", origin.x);
}