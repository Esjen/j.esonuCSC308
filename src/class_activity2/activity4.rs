use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Work complete").unwrap();
    });

    println!("Message received: {}", rx.recv().unwrap());
}