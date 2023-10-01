fn main() {
    let mut input = String::new();

    println!("Enter your weight on earth: ");
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let weight_on_earth: f32 = input.trim().parse().unwrap();
            let weight_on_mars = calculate_weight(weight_on_earth);
            println!("Your weight on mars is: {}kg", weight_on_mars)
        }
        Err(e) => println!("Error: {e}"),
    }
}

fn calculate_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
