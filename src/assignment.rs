use std::io;

fn main() {
    println!("=== Smart Energy Company (SEC) Billing Calculator ===");

    let mut input = String::new();
    println!("Enter your energy consumption in kWh:");
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let units: f32 = input.trim().parse().expect("Please enter a valid number");

    let rate: f32 = if units > 200.0 {
        30.0
    } else if units > 100.0 {
        25.0
    } else {
        20.0
    };

    let total_bill = units * rate;

    println!("----------------------------------------");
    println!("Energy Consumption: {:.2} kWh", units);
    println!("Rate per unit: ₦{:.2}", rate);
    println!("Total Bill: ₦{:.2}", total_bill);
    println!("----------------------------------------");
}
