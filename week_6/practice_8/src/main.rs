fn main() {
    let city_arr: [&str; 5] = ["Abuja", "Port Harcourt", "Maiduguri", "Kano", "Lagos"];

    println!("array is {:?}", city_arr);
    println!("array size is {}", city_arr.len());

    for index in 0..5 {
        println!("City {} is located in: {}", index, city_arr[index]);
    }
}