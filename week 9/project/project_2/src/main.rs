use std::fs::File;
use std::io::Write;

struct Student {
    id: u32,
    name: String,
    age: u32,
    department: String,
}

fn main() {
    let students = vec![
        Student {
            id: 1,
            name: String::from("John Doe"),
            age: 20,
            department: String::from("Computer Science"),
        },
        Student {
            id: 2,
            name: String::from("Jane Smith"),
            age: 22,
            department: String::from("Business Administration"),
        },
        Student {
            id: 3,
            name: String::from("Michael Johnson"),
            age: 19,
            department: String::from("Economics"),
        },
    ];

    println!("Student Details:");
    for student in &students {
        println!(
            "ID: {}, Name: {}, Age: {}, Department: {}",
            student.id, student.name, student.age, student.department
        );
    }

    let mut file = File::create("students.txt")
        .expect("Unable to create file");

    for student in &students {
        let line = format!(
            "ID: {}, Name: {}, Age: {}, Department: {}\n",
            student.id, student.name, student.age, student.department
        );
        file.write_all(line.as_bytes())
            .expect("Unable to write to file");
    }

    println!("Student details saved to students.txt");
}