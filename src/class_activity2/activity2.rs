use std::thread;

fn main() {
    let worker = thread::spawn(|| {
        for i in 1..5 {
            println!("worker: {}", i);
        }
    });

    worker.join().unwrap();
    println!("Main thread: done.");
    
}