use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    // Step 1: Define the datasets
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Step 2: Create output file
    let mut file = File::create("merged_dataset.txt")?;

    // Step 3: Merge datasets and write to file
    writeln!(file, "{:<5} | {:<30} | {:<15} | {:<20}", "S/N", "Commissioner", "Zone", "Ministry")?;
    writeln!(file, "{}", "-".repeat(80))?;

    for i in 0..commissioners.len() {
        writeln!(
            file,
            "{:<5} | {:<30} | {:<15} | {:<20}",
            i + 1,
            commissioners[i],
            zones[i],
            ministries[i]
        )?;
    }

    println!("Merged dataset successfully written to merged_dataset.txt ");

    Ok(())
}