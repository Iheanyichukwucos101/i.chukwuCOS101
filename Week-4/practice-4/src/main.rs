fn main() {
   
   let fullname = " Ibekwe Iheanyichukwu";
   let department = " Software Engineering";
   let uni = " Pan Atlantic University";

   let mut school = "School of Science".to_string();
   //puch string
   school.push_str("  and Technology");

   println!("My name is: {}",fullname ); 
   //check length
   println!("The lenght my fullname is:{}",fullname.len());
   println!("I am a Student of {}Department",department);
   println!("{}",school );
   println!("{}",uni);
}
