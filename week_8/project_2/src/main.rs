fn main() {
    let years = vec![2, 3, 6, 7, 1, 9, 10, 40];
    
    let mut max_year = years[0]; 
    
   
    for &i in years.iter() {
        if i > max_year {
            max_year = i; 
        }
    }

    println!("The highest value is: {}", max_year);
}