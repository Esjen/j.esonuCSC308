fn main() {
    let factorial = |f: &dyn Fn(u64) -> u64, n: u64| -> u64 {
        if n <= 1 {
            1
        } else {
            n * f(f, n - 1)
        }
    };

    let result = factorial(&factorial, 5);
    println!("Factorial of 5 is {}", result);
}
