fn main() {
   let fullname = " Chidubem John Umeh";
   let department = "Computer Science";
   let uni  = "Pan-Atlantic Univeristy".to_string();
   let mut school = "School of Science";
   school.push_str(" and technology");

   println!("My name is: {}", fullname);
   println!("The length my fullname is: {}", fullname.len());
   println!("I am a student of {} Department", department);
   println!(" {}", school);
   println!(" {}", uni);
}
