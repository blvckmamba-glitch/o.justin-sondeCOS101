use std::fs::File;
use std::io::{Write, Read, Result};

struct Student {
    id: u32,
    name: String,
    age: u8,
    department: String,
}

fn main() -> Result<()> {
    let students = vec![
        Student {
            id: 1,
            name: String::from("John Doe"),
            age: 20,
            department: String::from("Computer Science"),
        },
        Student {
            id: 2,
            name: String::from("Mary Jane"),
            age: 22,
            department: String::from("Business Administration"),
        },
        Student {
            id: 3,
            name: String::from("Ahmed Musa"),
            age: 19,
            department: String::from("Economics"),
        },
    ];

    println!("{:<5} | {:<20} | {:<5} | {:<25}", "ID", "Name", "Age", "Department");
    println!("{}", "-".repeat(60));
    for student in &students {
        println!(
            "{:<5} | {:<20} | {:<5} | {:<25}",
            student.id, student.name, student.age, student.department
        );
    }

    let mut file = File::create("students.txt")?;
    writeln!(file, "{:<5} | {:<20} | {:<5} | {:<25}", "ID", "Name", "Age", "Department")?;
    writeln!(file, "{}", "-".repeat(60))?;
    for student in &students {
        writeln!(
            file,
            "{:<5} | {:<20} | {:<5} | {:<25}",
            student.id, student.name, student.age, student.department
        )?;
    }

    println!("Student details successfully saved to students.txt");

    let mut file = File::open("students.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("\nContents of students.txt:");
    println!("{}", contents);

    Ok(())
}