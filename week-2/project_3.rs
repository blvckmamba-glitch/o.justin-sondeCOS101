fn main() {
let principal:f64 = 510_000.00; // initial amount
let rate:f64 = 0.05; // 5% annual interest
let time:f64 = 1.00;
let years:f64 = 3.00; //  for 3 years  	();

 let amount = principal * (1.0 - rate / 100.00).powf(time * years);

 println!("Final amount after {} years = {:.2}", years, amount);
}