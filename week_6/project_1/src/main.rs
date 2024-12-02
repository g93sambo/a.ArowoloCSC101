use std::io;
fn trap_area(){
    let mut h = String::new();
            println!("enther your height");
            io::stdin()
            .read_line(&mut h)
            .expect("not a valide input");
        let h:f64 = h.trim().parse().expect("not an integer");

        let mut b1 = String::new();
        println!("enther your b1");
            io::stdin()
            .read_line(&mut b1)
            .expect("not a valide input");
        let b1:f64 = b1.trim().parse().expect("not an integer");

        let mut b2 = String::new();
        println!("enther your b2");
            io::stdin()
            .read_line(&mut b2)
            .expect("not a valide input");
        let b2:f64 = b2.trim().parse().expect("not an integer");
   let area:f64 = h/2.0 * (b1 + b2);
   println!("the area of the trapezium is {area}");
}

fn rhomb_area( ){
    let mut d1 = String::new();
    println!("enther your d1");
    io::stdin()
    .read_line(&mut d1)
    .expect("not a valide input");
let d1:f64 = d1.trim().parse().expect("not an integer");

let mut d2 = String::new();
println!("enther your d2");
    io::stdin()
    .read_line(&mut d2)
    .expect("not a valide input");
let d2:f64 = d2.trim().parse().expect("not an integer");
    let area:f64 = 0.5 * d1 * d2 ;
    println!("the area of the rhombus is {area}");
 }

 fn par_area( ){
    let mut base = String::new();
    println!("enther your base");
            io::stdin()
            .read_line(&mut base)
            .expect("not a valide input");
        let base:f64 = base.trim().parse().expect("not an integer");

        let mut altitude = String::new();
        println!("enther your altitude");
            io::stdin()
            .read_line(&mut altitude)
            .expect("not a valide input");
        let altitude:f64 = altitude.trim().parse().expect("not an integer");
        let area:f64 = 0.5 * altitude * base;
    println!("the area of the parellelogram is {area}");
 }

 fn cube_area( ){
    let mut length = String::new();
    println!("enther your length");
            io::stdin()
            .read_line(&mut length)
            .expect("not a valide input");
        let length:f64 = length.trim().parse().expect("not an integer");
    let area:f64 = 6.0 * length * length;
    println!("the area of the cube is {area}");
 }

 fn cyl_vol(){
    let mut l = String::new();
    println!("enther your length");
            io::stdin()
            .read_line(&mut l)
            .expect("not a valide input");
        let l:f64 = l.trim().parse().expect("not an integer");
        let mut r = String::new();
        println!("enther your radius");
            io::stdin()
            .read_line(&mut r)
            .expect("not a valide input");
        let r:f64 = r.trim().parse().expect("not an integer");
        let pi:f64 = 22.0/7.0;
    let volume:f64 = pi * r * l ;
    println!("the area of the parellelogram is {volume}");
 }


fn main() {
    println!("Type t for the area of a trapezium
    Type r for the area of a rhombus
    Type p for the area of a Parallelogram
    Type c for the area of a cube
    Type cy for the volume of a Cylinder

    type done if you are done
    ");
    loop{
        println!("Choose a letter from t,r,p,c,cy 1-5 or type done if you are done
        ");
        let mut input1 = String::new();
        io::stdin()
        .read_line(&mut input1)
        .expect("not a valide input");
       let input = input1.trim().to_lowercase();
          
         if input == "done"{
            break;
          } 

        if input == "t"{
        trap_area();
        }
        else if input == "r"{
        rhomb_area();
        } else if input == "p"{
        par_area();
        }
        else if input == "c"{
        cube_area();
        }
        else if input == "cy"{    
        cyl_vol();
        }
      
    }
}
