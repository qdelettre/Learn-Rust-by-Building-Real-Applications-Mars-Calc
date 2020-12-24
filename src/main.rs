use std::io;
mod input;
mod weight;

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let user_input = input::prompt(input, output, "Enter your weight (kg): ");

    let f = input::get_value(&user_input).expect("Failed to parse");
    let mars_weight = weight::get_on_mars(f);

    println!("Weight on Mars : {}kg", mars_weight);
}
