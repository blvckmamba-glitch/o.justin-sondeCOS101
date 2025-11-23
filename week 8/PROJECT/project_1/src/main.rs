use std::io::{self, Write};

#[derive(Debug)]
struct Band {
    label: &'static str,
    min_exp: u32,
    max_exp: Option<u32>, // None means "no upper bound"
    titles_by_sector: Vec<&'static str>, // aligned with `sectors` index
}

fn normalize(s: &str) -> String {
    s.trim().to_lowercase()
}

fn find_aps_level<'a>(
    sectors: &'a Vec<&'static str>,
    bands: &'a Vec<Band>,
    sector_input: &str,
    title_input: &str,
    years: u32,
) -> Option<&'a str> {
    let sector = normalize(sector_input);
    let title = normalize(title_input);

    // Find sector index
    let sector_idx = sectors
        .iter()
        .position(|s| normalize(s) == sector)?;

    // Find band by experience
    let band = bands.iter().find(|b| {
        let within_min = years >= b.min_exp;
        let within_max = match b.max_exp {
            Some(max) => years <= max,
            None => true,
        };
        within_min && within_max
    })?;

    // Match title in the sector column
    let expected_title = band.titles_by_sector.get(sector_idx)?;
    if normalize(expected_title) == title {
        Some(band.label)
    } else {
        None
    }
}

fn main() {
    // Define sectors (columns)
    let sectors = vec![
        "Public Servant",
        "Office Administrator",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    // Define APS bands with titles aligned to sectors
    let bands = vec![
        Band {
            label: "APS 1-2",
            min_exp: 1,
            max_exp: Some(2),
            titles_by_sector: vec![
                "APS 1-2",
                "Intern",
                "–",
                "Paralegal",
                "Placement",
            ],
        },
        Band {
            label: "APS 3-5",
            min_exp: 3,
            max_exp: Some(5),
            titles_by_sector: vec![
                "APS 3-5",
                "Administrator",
                "Research Assistant",
                "Junior Associate",
                "Classroom Teacher",
            ],
        },
        Band {
            label: "APS 5-8",
            min_exp: 5,
            max_exp: Some(8),
            titles_by_sector: vec![
                "APS 5-8",
                "Senior Administrator",
                "PhD Candidate",
                "Associate",
                "Snr Teacher",
            ],
        },
        Band {
            label: "EL1 8-10",
            min_exp: 8,
            max_exp: Some(10),
            titles_by_sector: vec![
                "EL1 8-10",
                "Office Manager",
                "Post-Doc Researcher",
                "Senior Associate 1-2",
                "Leading Teacher",
            ],
        },
        Band {
            label: "EL2 10-13",
            min_exp: 10,
            max_exp: Some(13),
            titles_by_sector: vec![
                "EL2 10-13",
                "Director",
                "Senior Lecturer",
                "Senior Associate 3-4",
                "Deputy Principal",
            ],
        },
        Band {
            label: "SES",
            min_exp: 13,
            max_exp: None,
            titles_by_sector: vec![
                "SES",
                "CEO",
                "Dean",
                "Partner",
                "Principal",
            ],
        },
    ];

    // Simple CLI
    let mut sector = String::new();
    let mut title = String::new();
    let mut years_str = String::new();

    print!("Enter sector (e.g., Lawyer): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut sector).unwrap();

    print!("Enter role title (e.g., Associate): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut title).unwrap();

    print!("Enter years of experience (e.g., 6): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut years_str).unwrap();

    let years: u32 = match years_str.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid years of experience.");
            return;
        }
    };

    match find_aps_level(&sectors, &bands, &sector, &title, years) {
        Some(level) => println!("Staff holds position: {}", level),
        None => println!("No matching APS level found for the provided inputs."),
    }
}