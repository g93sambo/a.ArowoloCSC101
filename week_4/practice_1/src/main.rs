use std::io;

fn main() {
    println!("\nStudent Information management System");
    println!("\nPlease Enter your name");   
    let mut name = String::new();
    println!("{}", name);
    io::stdin()
     .read_line(&mut name)
     .expect("failes to read input");
    println!("Your name is: {}", name);

    println("\nEnter your age");
    let mut age = String::new();
        io::stdin().read_line(&mut age).expect("failed to read input");
    let age:i32 = age.trim().parse().expect("input not an integer");
    println("Your age is {}", age)
        
}
