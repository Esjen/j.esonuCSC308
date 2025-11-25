use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Parent PID: {}", std::process::id());

    // Spawn `sleep 5`
    let mut sleep_child: Child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn sleep");

    println!("Spawned sleep 5 with PID: {}", sleep_child.id());

    // Spawn `ls -la`
    let mut ls_child: Child = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to spawn ls");

    println!("Spawned ls -la with PID: {}", ls_child.id());

    // Spawn echo
    let mut echo_child: Child = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("Failed to spawn echo");

    println!("Spawned echo with PID: {}", echo_child.id());

    // Sleep so you can check child processes
    println!("Sleeping 6 seconds in parent to allow checking child processes...");
    sleep(Duration::from_secs(6));

    // Wait for children to finish
    let _ = sleep_child.wait();
    let _ = ls_child.wait();
    let _ = echo_child.wait();

    println!("All children finished.");
}
