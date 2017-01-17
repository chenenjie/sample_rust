
use std::sync::mpsc;
use std::thread;

const THREAD_COUNT :i32 = 2;

fn main() {
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();

    for id in 0..THREAD_COUNT {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            thread_tx.send(id + 1).unwrap();
            println!("send {}", id + 1);
        });
    }

    thread::sleep_ms(2000);
    println!("wake up");

    for _ in 0..THREAD_COUNT {
        println!("receive {}", rx.recv().unwrap());
    }
}