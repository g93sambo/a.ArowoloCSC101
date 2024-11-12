fn main() {
    // While true
    loop {
        // Initialize x
        let mut x = 0;

        loop {
            println!("x = {}", x);
            x += 1;

            // Break if x reaches 15
            if x == 15 {
                break;
            }
        }
    }
}