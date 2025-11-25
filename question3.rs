use std::process::{Command, Child};
use std::thread::sleep;
use std::time::Duration;
use std::process::ExitStatus;

fn main() {
    // Spawn a long-running process: ping google.com
    let mut ping_child: Child = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("Failed to start ping process");

    println!("Spawned ping with PID: {}", ping_child.id());

    // Wait 5 seconds
    println!("Parent sleeping for 5 seconds...");
    sleep(Duration::from_secs(5));

    // Kill the child process
    println!("Killing ping process...");
    match ping_child.kill() {
        Ok(_) => println!("Ping process killed successfully"),
        Err(e) => eprintln!("Failed to kill ping process: {}", e),
    }

    // Wait for the child to exit and get exit status
    let exit_status: ExitStatus = ping_child
        .wait()
        .expect("Failed to wait on ping process");

    println!("Ping process exited with status: {}", exit_status);
}
