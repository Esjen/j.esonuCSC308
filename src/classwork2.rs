use std::io;

fn cafe_discount_calculator() {
    let mut total_bill = String::new();
    println!("Enter the total bill amount (In Naira):");
    io::stdin().read_line(&mut total_bill).expect("Failed to read line");

    let total_bill: f32 = total_bill.trim().parse().expect("Please enter a correct value");

    let discount: f32 = if total_bill > 10000.0 {
        0.15 * total_bill   
    } else if total_bill > 5000.0 {
        0.10 * total_bill   
    } else {
        println!("No discount, your total bill is: {}", total_bill);
        return;
    };

    let final_bill_amount = total_bill - discount;
    println!("Your discounted bill is: {}", final_bill_amount);
}

fn main() {
    println!("Welcome to the Cafe Discount Calculator!");
    cafe_discount_calculator();
}
