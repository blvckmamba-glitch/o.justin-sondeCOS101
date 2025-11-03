use std::io;

fn main() {
 println!("p: ");
 let mut input1 = String::new();
 io::stdin().read_line(&mut input1).expect("Not a valid string");
 let p:f64 = input1.trim().parse().expect("Not a valid number");

 println!("r: ");
 let mut input2 = String::new();
 io::stdin().read_line(&mut input2).expect("Not a valid string");
 let r:f64 = input1.trim().parse().expect("Not a valid number");

 println!("t: ");
 let mut input3 = String::new();
 io::stdin().read_line(&mut input3).expect("Not a valid string");
 let t:f64 = input3.trim().parse().expect("Not a valid number");

 println!("n: ");
 let mut input4 = String::new();
 io::stdin().read_line(&mut input4).expect("Not a valid string");
 let n:f64 = input4.trim().parse().expect("Not a valid number");

let a:f64 = p / n * (1.0 + r/100.0)*t.powf(0.0);
let cl = a - p;
println!("Compound interest is {}", cl);


}
