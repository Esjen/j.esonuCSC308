use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    // Run the command `echo "Rust Process Management"` and capture output
    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .expect("Failed to execute command");

    // Convert stdout bytes to String
    let stdout_str = String::from_utf8_lossy(&output.stdout);

    println!("Captured output: {}", stdout_str.trim());

    // Write output to output.txt
    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(stdout_str.as_bytes())
        .expect("Failed to write to file");

    println!("Output written to output.txt");
}
