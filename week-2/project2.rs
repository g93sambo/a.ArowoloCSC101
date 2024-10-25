fn main(){
    let quantity = [2,1,3,3,1];
    let amount = [450000,1500000,750000,2850000,250000];
    let mut sum = [0;5];
    
    for i in 0..sum.len(){
        sum[i] = amount[i] * quantity[i];
    }
    let total: i32 = sum.iter().sum(); 
    let  sum_of_q: i32 = quantity.iter().sum();
    let average = total/sum_of_q;
    println!("{average}");

}