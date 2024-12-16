fn main() {
    println!("Enter your weight (kg): ");
    let mut weight = String::new();
    
    std::io::stdin().read_line(&mut weight).unwrap();
    let weight: f32 = weight.trim().parse().unwrap();

    println!("Your weight on mars is {}kg", calculate_weight_on_mars(weight));    
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
