use regex::Regex;

mod types;

use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let takanini = Surfs::new(
        "https://www.pickapart.co.nz/Takanini-Stock".to_string(),
        "https://www.pickapart.co.nz/eziparts/".to_string(),
    );

    let surf_matcher = Regex::new(r"Corolla (\w+).*? \((\d+)\)").unwrap();

    let link_matcher = Box::new(|model: String| {
        vec![
            Regex::new(format!(r"href='.*\/eziparts\/(Display_Vehicle.*{})'", model).as_str())
                .unwrap(),
            Regex::new(
                format!(r"href='.*\/eziparts\/(Display_Vehicle.*{} \d+-\d+)'", model).as_str(),
            )
            .unwrap(),
            Regex::new(
                format!(
                    r"href='.*\/eziparts\/(Display_Vehicle.*{} \d{{2}}\/\d{{2}}-\d{{2}}\/\d{{2}})'",
                    model
                )
                .as_str(),
            )
            .unwrap(),
        ]
    });

    println!("Takanini");
    takanini
        .scrape_page(surf_matcher.clone())
        .scrape_links(link_matcher.clone())
        .print();

    let avondale = Surfs::new(
        "https://www.pickapart.co.nz/Avondale-Stock".to_string(),
        "https://www.pickapart.co.nz/eziparts/".to_string(),
    );

    println!("Avondale");
    avondale
        .scrape_page(surf_matcher)
        .scrape_links(link_matcher)
        .print();

    Ok(())
}
