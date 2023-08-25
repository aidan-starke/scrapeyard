use regex::Regex;

mod types;

use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let takanini = Surfs {
        page_link: "https://www.pickapart.co.nz/Takanini-Stock".to_string(),
        surf_link: "https://www.pickapart.co.nz/eziparts/".to_string(),
        surfs: vec![],
    };

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

    let takanini_surfs = takanini
        .clone()
        .scrape_page(Regex::new(r"Corolla (\w+).*? \((\d+)\)").unwrap(), true)
        .scrape_links(link_matcher.clone());

    println!("{:#?}", &takanini_surfs);

    let avondale = Surfs {
        page_link: "https://www.pickapart.co.nz/Avondale-Stock".to_string(),
        ..takanini
    };

    let avondale_surfs = avondale
        .clone()
        .scrape_page(Regex::new(r"Corolla (\w+).*? \((\d+)\)").unwrap(), true)
        .scrape_links(link_matcher);

    println!("{:#?}", &avondale_surfs);

    Ok(())
}
