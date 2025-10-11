fn main() {
    // Sales amounts from the table
    let amount = [450_000.0, 1_500_000.0, 1_800_000.0, 2_850_000.0, 250_000.0];
    let quantity = [2.0, 1.0, 3.0, 3.0, 1.0];
    let quantity_length = quantity.len();
    let mut sum = 0.0;
    let mut total_quantity = 0.0;
    for i in 1..quantity_length{
        sum = sum + (amount[i] * quantity[i]);
    };

    for i in 1..quantity_length{
        total_quantity = total_quantity + quantity[i];
    };

    let average_sales = sum / total_quantity;

    // Calculate sum
    /*let sum: f64 = sales.iter().sum();

    // Calculate average
    let average = sum / sales.len() as f64;*/

    println!("Total Sales = {:.2}", sum);
    println!("Average Sales = {:.2}", average_sales); 
}