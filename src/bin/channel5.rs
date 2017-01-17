
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx): (mpsc::SyncSender<i32>, mpsc::Receiver<i32>) = mpsc::sync_channel(0);

    let new_thread = thread::spawn(move || {
        println!("before send");
        tx.send(1).unwrap();
        println!("after send");
    });

    println!("before sleep");
    thread::sleep_ms(5000);
    println!("after sleep");

    println!("receive {}", rx.recv().unwrap());
    new_thread.join().unwrap();
}