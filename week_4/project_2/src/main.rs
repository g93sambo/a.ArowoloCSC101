use std::io;
fn main() {
   let mut age_inp = String::new();
   let mut exp_inp = String::new();

   io::stdin()
   .read_line(&mut age_inp)
   .expect("not an integer");
    let age:i32 = age_inp.trim().parse().expect("not a valid age");

    io::stdin()
   .read_line(&mut exp_inp)
   .expect("not an option");
    let binding = exp_inp.to_lowercase();
    let exp = binding.trim();

    if age >=40 && exp == "true" {
        let pay = 1_560_000;
        println!("your pay is N{}", pay);
    } else if age >= 30 && age < 40 && exp =="true"{
        let pay = 1_480_000;
        println!("your pay is N{}", pay);

    }else if age < 28 && exp == "true"{
        let pay = 1_300_000;
        println!("your pay is N{}", pay);
    } else if exp != "true" && exp != "false"{
        println!("please try again and enter true or false");

    }
    else{
        let pay = 100_000;
        println!("your pay is N{}", pay);
    }
}
