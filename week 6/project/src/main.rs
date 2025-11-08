use std::collections::HashMap;
use std::io;

fn main() {
    // Define the menu using a HashMap
    let menu: HashMap<char, (&str, u32)> = [
        ('P', ("Poundo Yam/Edinkaiko Soup", 3200)),
        ('F', ("Fried Rice & Chicken", 3000)),
        ('A', ("Amala & Ewedu Soup", 2500)),
        ('E', ("Eba & Egusi Soup", 2000)),
        ('W', ("White Rice & Stew", 2500)),
    ]
    .iter()
    .cloned()
    .collect();

    println!("Welcome to the Nigerian Food Ordering System!");
    println!("Here is our menu:");

    for (code, (name, price)) in &menu {
        println!("{} = {} - ₦{}", code, name, price);
    }

    // Get food type input
    println!("\nEnter the food code (P, F, A, E, W):");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect("Failed to read input");
    let food_code = food_code.trim().to_uppercase().chars().next().unwrap_or(' ');

    // Validate food code
    if let Some((food_name, unit_price)) = menu.get(&food_code) {
        // Get quantity input
        println!("Enter the quantity:");
        let mut quantity_input = String::new();
        io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
        let quantity: u32 = quantity_input.trim().parse().expect("Invalid quantity");

        let mut total = unit_price * quantity;

        // Apply discount if applicable
        if total > 10_000 {
            let discount = total as f32 * 0.05;
            total = (total as f32 - discount) as u32;
            println!("A 5% discount has been applied!");
        }

        println!(
            "\nOrder Summary:\nItem: {}\nQuantity: {}\nTotal Amount: ₦{}",
            food_name, quantity, total
        );
    } else {
        println!("Invalid food code entered. Please try again.");
    }
}
