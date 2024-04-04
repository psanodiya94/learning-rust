fn main() {
    // Loop from 1 to 5 printing the index using loop
    let mut idx = 3;
    
    loop {
        println!("Index is: {}", idx);

        idx += 1;

        if idx > 5 {
            break;
        }
    }

    // Loop from 1 to 5 printing the index using for
    println!("Even Number between 1 to 10 are ");
    for i in 1..10 {
        let num = if i % 2 == 0 {
            i
        } else {
            continue 
        };
        println!("-> {}", num);
    }

    // Loop from 1 to 5 printing the index using while
    idx = 2;

    while idx < 5 {
        println!("Index is: {}", idx);

        idx += 1;
    }

    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Enter a word(type 'stop' to exit): ");
        std::io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {}", input);
    }
    println!("Goodbye!");
}
