use regex::Regex;

mod types;

use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    vec!["Avondale", "Takanini"]
        .into_iter()
        .for_each(|location| {
            println!("{}", location);

            let scraper = Surfs::new(
                format!("https://www.pickapart.co.nz/{}-Stock", location),
                "https://www.pickapart.co.nz/eziparts/".to_string(),
            );

            let surfs = scraper
                .scrape_page(surf_matcher.clone())
                .scrape_links(link_matcher.clone());

            surfs.compare_and_write_surfs(location.to_string());
        });

    Ok(())
}
