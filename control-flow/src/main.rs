fn main() {
    // Conditional control flow
    let proceed = true;
    if proceed {
        println!("Proceed");
    } else {
        println!("Not Proceed");
    }

    let height = 180;
    if height > 170 {
        println!("Tall");
    } else if height > 150 {
        println!("Average");
    } else {
        println!("Short");
    }
}
