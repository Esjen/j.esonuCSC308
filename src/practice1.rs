fn main() {
    let multiplier: i32 = 3;
    let multiply = |x: i32| x * multiplier;

    println!("Result: {}", multiply(5));
}
// Rust forces you to take ownership to avoid the value having no reference when parent process is terminated.