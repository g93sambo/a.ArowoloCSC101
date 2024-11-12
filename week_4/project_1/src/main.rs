use std::io;
fn main() {
  let mut a  = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("not a valid integer");
    let a:f32 = a.trim().parse().expect("not a valid integer");
    let mut b  = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("not a valid integer");
  let b:f32 = b.trim().parse().expect("not a valid integer");
  let mut c  = String::new();
    io::stdin()
    .read_line(&mut c)
    .expect("not a valid integer");
    let c:f32 = c.trim().parse().expect("not a valid integer");

    let discriminant:f32 = b.powf(2.0) - 4.0* a *c;

    if discriminant > 0.0 {
     let   root_1 =  (-b + discriminant.sqrt()) / 2.0*a;
     let   root_2 =  (-b - discriminant.sqrt()) / 2.0*a;
     println!("root 1 is {} and root 2 is {:.}",root_1, root_2 );
    } else if discriminant == 0.0{
     let   root = -b / 2.0*a;
     println!("the root is {}", root);
    } else {
        println!("there are no real rootss");
    }

}
