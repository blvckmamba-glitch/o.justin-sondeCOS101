use std::fs::File;
use std::io::Write;

fn main() {
    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let mut content = String::from("Nigerian Breweries Plc - Drink Categories\n\n");

    content.push_str("Lager:\n");
    for drink in &lager {
        content.push_str(&format!("- {}\n", drink));
    }

    content.push_str("\nStout:\n");
    for drink in &stout {
        content.push_str(&format!("- {}\n", drink));
    }

    content.push_str("\nNon-Alcoholic:\n");
    for drink in &non_alcoholic {
        content.push_str(&format!("- {}\n", drink));
    }

    let mut file = File::create("nigerian_breweries.txt")
        .expect("Unable to create file");
    file.write_all(content.as_bytes())
        .expect("Unable to write file");

    println!("Drink categories saved to nigerian_breweries.txt");
    
}