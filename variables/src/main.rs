
fn main() {
    // Unmutatable variables
    let message1 = "Name: Sanodiya, Pradumn";
    let weight = 180.0;

    let killo = weight / 2.2;
    println!("{} {}", message1, killo);

    // Mutatable variables
    let mut message2 = String::from("Name: Sanodiya, Ankit");
    let mut age = 20;
    println!("{} {}", message2, age);
    message2.clear();
    message2.push_str("Name: Sanodiya, Pradumn");
    age = 29;
    println!("{} {}", message2, age);
}
