extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample,Range};

fn main() {
    // let mut rng = rand::thread_rng();
    // if rng.gen() {
    //     println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>());
    // }

    // let tuple = rand::random::<(i32)>();
    // println!("{:?}", tuple);
    
    let between = Range::new(10, 10000);
    let mut rng = rand::thread_rng();
    let mut sum = 0;
    for _ in 0..1000 {
        println!("{:?}",between.ind_sample(&mut rng));
    }
    println!("{}", sum);
}