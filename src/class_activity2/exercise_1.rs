use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel();

    let num_threads = 3;
    let messages_per_thread = 5;

    for i in 1..=num_threads {
        let thread_tx = tx.clone();
        let prefix = format!("T{}:", i);

        thread::spawn(move || {
            for j in 1..=messages_per_thread {
                let msg = format!("{} message {}", prefix, j);
                thread_tx.send(msg).unwrap();

                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    drop(tx);

    for received in rx {
        println!("Received: {}", received);
    }
}
