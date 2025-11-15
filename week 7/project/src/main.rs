use std::io::{self, Write};
use std::f64::consts::PI;

fn main() {
    println!("Calculator - choose an option:");
    println!("1) Area of Trapezium ");
    println!("2) Area of Rhombus ");
    println!("3) Area of Parallelogram ");
    println!("4) Surface Area of Cube");
    println!("5) Volume of Cylinder");
    print!("Enter choice (1-5): ");
    io::stdout().flush().unwrap();

    let choice = read_usize().unwrap_or_else(|err| {
        eprintln!("Input error: {}. Exiting.", err);
        std::process::exit(1);
    });

    match choice {
        1 => {
            println!("\nCompute area of a trapezium.");
            let height = prompt_f64("Enter height: ");
            let base1 = prompt_f64("Enter base1: ");
            let base2 = prompt_f64("Enter base2: ");
            let area = area_trapezium(height, base1, base2);
            println!("Area of trapezium = {:.6}", area);
        }
        2 => {
            println!("\nCompute area of a rhombus.");
            let d1 = prompt_f64("Enter diagonal1: ");
            let d2 = prompt_f64("Enter diagonal2: ");
            let area = area_rhombus(d1, d2);
            println!("Area of rhombus = {:.6}", area);
        }
        3 => {
            println!("\nCompute area of a parallelogram.");
            let base = prompt_f64("Enter base: ");
            let altitude = prompt_f64("Enter altitude: ");
            let area = area_parallelogram(base, altitude);
            println!("Area of parallelogram = {:.6}", area);
        }
        4 => {
            println!("\nCompute surface area of a cube.");
            let side = prompt_f64("Enter side length: ");
            let area = surface_area_cube(side);
            println!("Surface area of cube = {:.6}", area);
        }
        5 => {
            println!("\nCompute volume of a cylinder.");
            let radius = prompt_f64("Enter radius: ");
            let height = prompt_f64("Enter height: ");
            let vol = volume_cylinder(radius, height);
            println!("Volume of cylinder = {:.6}", vol);
        }
        _ => {
            eprintln!("Invalid choice. Please run the program again and choose 1-5.");
            std::process::exit(1);
        }
    }
}

fn read_line_trimmed() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
}

fn read_usize() -> Result<usize, &'static str> {
    let s = read_line_trimmed();
    s.parse::<usize>().map_err(|_| "expected an integer choice")
}

fn prompt_f64(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let input = read_line_trimmed();
        match input.parse::<f64>() {
            Ok(v) => return v,
            Err(_) => {
                eprintln!("Invalid number. Please enter a valid numeric value.");
            }
        }
    }
}

fn area_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    (height / 2.0) * (base1 + base2)
}

fn area_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn surface_area_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius.powi(2) * height
}