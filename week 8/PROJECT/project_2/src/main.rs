use std::cmp::Ordering;

#[derive(Debug)]
struct Applicant {
    name: String,
    years_of_experience: u32,
}

fn find_most_experienced(applicants: &[Applicant]) -> Option<&Applicant> {
    applicants.iter().max_by(|a, b| {
        a.years_of_experience
            .cmp(&b.years_of_experience)
    })
}

fn main() {
    // Sample applicants (these could come from user input)
    let applicants = vec![
        Applicant {
            name: "Osinachi".to_string(),
            years_of_experience: 5,
        },
        Applicant {
            name: "Mamba".to_string(),
            years_of_experience: 12,
        },
        Applicant {
            name: "Chioma".to_string(),
            years_of_experience: 8,
        },
    ];

    match find_most_experienced(&applicants) {
        Some(applicant) => {
            println!(
                "The applicant with the highest experience is {} with {} years.",
                applicant.name, applicant.years_of_experience
            );
        }
        None => println!("No applicants found."),
    }
}
