

use std::thread;

static mut VAR: i32 = 5;

fn main() {
    let new_thread = thread::spawn(move || {
        unsafe {
            println!("static value in new thread: {}", VAR);
            VAR = VAR + 1
        }
    });

    new_thread.join().unwrap();
    unsafe{
        println!("static value in main thread: {}", VAR);
    }
}