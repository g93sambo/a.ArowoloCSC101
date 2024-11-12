fn main() {
    let mut input_str = String::new();

    println!("Enter Your height (in centimeters):");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Not a valid string");

    let height: f32 = input_str
        .trim()
        .parse()
        .expect("Not a valid number");

    if height >= 130.8 && height <= 165.0 {
        println!("You are of average height person.");
    } else if height > 165.0 {
        if height > 180.0 {
            println!("You are tall.");
        } else {
            println!("You are of above average height.");
        }
    } else if height < 130.8 && height > 100.0 {
        println!("You are short.");
    } else {
        println!("Abnormal height.");
    }
}