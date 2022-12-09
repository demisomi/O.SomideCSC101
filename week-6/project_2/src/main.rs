// A program that asks for sibling details are then displays them
use std::io;

fn main() {
   let mut user_input1 = String::new();
   println!("How many siblings do you have")

   io::stdin().read_line(& mut user_input1).expect("Not a valid String");
   let sibling_num = user_input1.trim().parse().expect("Not a valid Number");

   for i in 0..sibling_num {
      println!("What is the first name of sibling {}", i);

      let mut user_input2 = String::new();
      io::stdin().read_line(& mut user_input2).expect("Not a valid String");
      let first_name = user_input2.trim();

      println!("What is the age of sibling {}", i);

      let mut user_input3 = String::new();
      io::stdin().read_line(& mut user_input3).expect("Not a valid String");
      let age = user_input3.trim().parse().expect("Not a valid Number");

      if age > 18{
         
      }else{

      }

   }
}
