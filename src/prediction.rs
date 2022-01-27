mod tools;

use tools::csv;
use std::io::stdin;

fn main() {
    let theta = csv::get_theta();
    let mut km = String::new();

    println!("Please enter a mileage:");
    stdin()
        .read_line(&mut km)
        .expect("Failed to read user input.");

    let km = km.trim()
        .parse::<f64>()
        .expect("Bad input.");

    println!("The prediction of car price: {} $", (theta[0]+ (theta[1] * km)) as u32);
}