use std::fs::File;
use std::io::{self, BufRead, Read};

fn loop_lines(filename: &str, start_line: usize, end_line: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if (start_line..=end_line).contains(&(i + 1)) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn display_database_structure() -> io::Result<()> {
    let filename = "globacom_dbase.sql"; 
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    println!("{}", content);
    Ok(())
}

fn display_project_manager() -> io::Result<()> {
    loop_lines("globacom_dbase.sql", 160, 164)
}

fn display_employee() -> io::Result<()> {
    loop_lines("globacom_dbase.sql", 173, 182)
}

fn display_customer() -> io::Result<()> {
    loop_lines("globacom_dbase.sql", 105, 117)
}

fn display_vendor() -> io::Result<()> {
    loop_lines("globacom_dbase.sql", 126, 136)
}

fn main() {
    println!("Welcome to the Globacom Database!");
    println!("What type of user are you:");
    println!("1. Administrator");
    println!("2. Project Manager");
    println!("3. Employee");
    println!("4. Customer");
    println!("5. Vendor");

    let mut input = String::new();
    println!("Enter your user type:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let input: i32 = input.trim().parse().unwrap();

    if input == 1 {
        let mut password = String::new();
        println!("Enter your admin password:");
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read input");
        
        let password = password.trim().to_lowercase();
        if password == "sambo" {
            display_database_structure();
        } else {
            println!("Incorrect password!");
        }
    } else {
        let result = match input {
            2 => display_project_manager(),
            3 => display_employee(),
            4 => display_customer(),
            5 => display_vendor(),
            _ => {
                println!("Invalid user type!");
                return;
            }
        };

        if let Err(e) = result {
            println!("Error: {}", e);
        }
    }
}
