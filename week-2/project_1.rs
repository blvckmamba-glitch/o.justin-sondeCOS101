fn  main() {
let principal:f64 = 520000000.00;  // initial amount
 let rate: f64 = 0.10;        // 10% annual interest
 let times: f64 = 1.0;
    let years: f64 = 5.0;       // for 5 years

     let amount = principal * (1.0 + rate / times).powf(times * years);
    let compound_interest = amount - principal;

 println!("Final amount after {} years = {:.2}", years, amount);
    println!("Compound interest earned = {:.2}", compound_interest);
}
