use std::io;

fn main() {
    let mut experience = String::new();
    let mut age = String::new();

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut experience).unwrap();
    let experienced = experience.trim().eq_ignore_ascii_case("yes");

    println!("Enter the employee's age: ");
    io::stdin().read_line(&mut age).unwrap();
    let age: u32 = age.trim().parse().unwrap();

    let incentive = if experienced {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000 * 12 // Monthly incentive converted to annual
        } else {
            0 // No rule defined for 28–29
        }
    } else {
        100_000
    };

    println!("Annual incentive: ₦{}", incentive);
}