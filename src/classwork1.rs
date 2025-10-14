use std::io;

fn smart_weather_temperature_converter() {
    let mut tempincelsius = String::new();
    println!("Enter the temperature in celsius:");
    io::stdin().read_line(&mut tempincelsius).expect("Failed to read line");
    let tempincelsius: f32 = tempincelsius.trim().parse().expect("Please enter a correct value");
    let tempinfarenheit = (tempincelsius * 9.0 / 5.0) + 32.0;
    println!("The corresponding temperature in farenheit is: {}", tempinfarenheit);
}

fn main() {
    println!("Welcome to the Smart Weather Temperature Converter!");
    smart_weather_temperature_converter();
}



