extern crate rand;
extern crate threadpool;
extern crate num_cpus;

use rand::Rng;
use rand::distributions::{IndependentSample,Range};
use std::time::SystemTime;
use std::fs::File;
use std::io::BufWriter;
use std::io::prelude::*;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;


fn caculate() -> Vec<f32>{
    let mut price = 10000;
    let bottom = 1;   
    let mut rng = rand::thread_rng();
    let mut queue = vec![];
    for i in 0..28 {
        let top = (price/(30-i))*2;
        let between = Range::new(1, top);
        let consume = between.ind_sample(&mut rng);
        queue.push((consume as f32) /100.0);        
        price = price - consume;
    }
    let between = Range::new(1, price - 1);
    let consume = between.ind_sample(&mut rng);
    queue.push((consume as f32) /100.0);
    queue.push((price - consume) as f32/100.0);
    return queue;
}

fn main(){
    let threads = num_cpus::get();
    println!("thread : {}", threads);
    let pool = ThreadPool::new(threads);
    let (tx, rx) = channel();

    let jobs = 200000;
    let now = SystemTime::now();
    for _ in 0..jobs{
        let tx = tx.clone();
        pool.execute(move||{
            tx.send(caculate());
        });
    }
    let mut f = BufWriter::new(File::create("foo.txt").unwrap());
    // let result = rx.iter().take(jobs).
    for _ in 0..jobs{
        let queue = rx.recv();
        f.write(format!("{:?} \n", queue.unwrap()).as_bytes()).unwrap();
    }
    // println!("{:?}", result);
    f.write(format!("{:?}", now.elapsed()).as_bytes()).unwrap();
    println!("{:?}", now.elapsed());
}