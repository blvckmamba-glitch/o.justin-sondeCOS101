use std::io;

fn main() {
   println!("Enter your name: ");
   let mut name = String::new();
   io::stdin().read_line(&mut name).expect("Failed to read input");

   println!("Enter score1: ");
   let mut input1 = String::new();
   io::stdin().read_line(&mut input1).expect("Not a valid string");
   let score1:f32 = input1.trim().parse().expect("Not a valid number");

   println!("Enter score2: ");
   let mut input2 = String::new();
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let score2:f32 = input2.trim().parse().expect("Not a valid number");

   println!("Enter score3: ");
   let mut input3 = String::new();
   io::stdin().read_line(&mut input3).expect("Not a valid string");
   let score3:f32 = input3.trim().parse().expect("Not a valid number");

   let average = (score1 + score2 + score3)/3.0;
    println!("you average is {}", average);
     number");
  
   if average >= 100.0 && average <= 70.0
   {
      println!("your grade is A {}", name);
   }
   else if average > 70.0 && average <= 60.0
   {
      println!("Your grade is B {}", name);
   }
   else if  average > 60.0 && average <= 50.0
   {
      println!("Your grade is C {}", name);
   }
   else if average > 50.0 && average <= 45.0
   {
      println!("Your grade is D {}", name);
   }
   else if average > 45.0 && average <= 0.0
   {
      println!("Your grade is F {}", name);
   }
}

