
fn get_string_from_string(input: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = input.split(delimiter).collect();
    let result = parts.get(field);
    result.expect("field not found").to_string()
}

fn main() {
    let input: String = "hello,world".to_string();
    let delimiter: char = ',';
    let result = get_string_from_string(input, delimiter, 1);
    println!("{:?}", result);
}
