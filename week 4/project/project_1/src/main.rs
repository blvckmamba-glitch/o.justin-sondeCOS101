use std::io;

fn main() {
    println!("Enter coefficients a, b, and c:");

    let a = read_input("a");
    let b = read_input("b");
    let c = read_input("c");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);
    } else {
        println!("No real roots.");
    }
}

fn read_input(name: &str) -> f64 {
    loop {
        println!("Enter {}:", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid input. Please enter a number."),
        }
    }
}