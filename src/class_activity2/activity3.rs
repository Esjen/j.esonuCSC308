use std::thread;

fn main() {
    let message = String::from("Thread says hello!");

    let handle = thread::spawn(move || {
        println!("{}", message);
    });

    handle.join().unwrap();
}