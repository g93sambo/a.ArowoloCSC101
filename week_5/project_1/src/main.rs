use std::io;
fn main() {
    let mut total_cost = 0;
    println!(" What do you want to order (Note we have a 5% dicount on orders above N10,000)      
          P = Poundo Yam/Edinkaiko Soup  - N3,200
          F = Fried Rice & Chicken       - N3,000
          A = Amala & Ewedu Soup         - N2,500
          E = Eba and Egusi Soup         - N2,000
          W = White Rice & Stew          - N2,500 ");
  loop{  

          let mut food_code = String::new();
          println!("type P,F,A,E,w and type done when done with ordering");
          io::stdin()
          .read_line(&mut food_code)
          .expect("not a valid input");
        let choice = food_code.trim().to_lowercase();

        if choice == "done"{
            break
        }
        let price_per_item:u32;

        if choice == "p" {
            price_per_item = 3200;
        } else if choice == "f" {
            price_per_item = 3000;
        } else if choice == "a" {
            price_per_item = 2500;
        } else if choice == "e" {
            price_per_item = 2000;
        } else if choice == "w"  {
            price_per_item = 2500;
        } else {
            println!("Invalid code! Please select from the menu.");
            continue;
        }
        let mut quantity_input = String::new();
        println!("How many do you want");
          io::stdin()
          .read_line(&mut quantity_input)
          .expect("not a valid input");
        let quantity:u32 = quantity_input.trim().parse().expect("not an integer");
        total_cost += price_per_item * quantity;
        println!("This item has bee added to your cart 
         your Current total is : N{}", total_cost);
        
  };
  
  if total_cost > 10_000 {
    println!("\nYour total is ₦{} ", total_cost);
    let discount = (total_cost as f32 * 0.05) as u32;
    total_cost -= discount;
    println!("A discount of 5% (₦{}) has been applied.", discount);
    println!("Final Total: ₦{}", total_cost);
    println!("Thank you for your order!"); 
} else{
    println!("\nYour total is ₦{} ", total_cost);
    println!("Thank you for your order!"); 
}

}
