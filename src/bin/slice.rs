
fn show(arr: &[u8]) {
    for i in arr {
        print!("{} ", i);
    }
    println!("");
}


fn main() {
    let a: [u8; 3] = [1, 2, 3];

    let slice_a = &a[..];
    show(slice_a);

    let b: [u8; 4] = [1, 2, 3, 4];
    show(&b[..2]);


    let mut v1: Vec<i32> = vec![1, 2, 3];
    let v2 = vec![0; 10];
    println!("{}", v1[0]);

    for i in &v1 {
        print!("{}", i);
    }

    println!("");

    for i in &mut v1 {
        *i = *i + 1;
        print!("{}", i);
    }
}