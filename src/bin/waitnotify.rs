
use std::sync::{Arc, Mutex, Condva};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condva::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let &(ref lock, ref cvar) = &*pair2;
        let mut started = lock.lock().unwrap();

        *started = true;
        cvar.notiry_one();
        println!("notify main thread");
    });

    let &(ref lock, ref cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        println!("before wait");
        started = cvar.wait(started).unwrap();
        println!("after wait");
    }
}