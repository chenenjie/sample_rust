#[macro_use]
extern crate ndarray;

use ndarray::{Array3, arr3};

fn main() {
    let mut temperature = Array3::<f64>::zeros((3, 4, 5));

    temperature[[2, 2, 2]] += 0.5;

    let a = arr3(&[[[1, 2, 3],
                    [4, 5, 6]],
                    [[7, 8, 9],
                    [10, 11, 12]]]);
                
    assert_eq!(a.shape(), &[2, 2, 3]);


    let b = a.slice(s![.., 0..1, ..]);


    let c = arr3(&[[[1, 2, 3]],
                    [[7, 8, 9]]]);

    assert_eq!(b, c);
    assert_eq!(b.shape(), &[2, 1, 3]);
    
}