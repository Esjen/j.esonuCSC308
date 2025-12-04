
fn string_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let my_string = String::from("Hello, Rust!");
    let length = string_length(&my_string);

    println!("Length: {}", length);
    println!("Original string: {}", my_string);
}
