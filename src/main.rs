use regex::Regex;

mod types;

use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pickapart = Surfs {
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

    let pickapart_surfs = pickapart
        .scrape_page(Regex::new(r"Corolla (\w+).*? \((\d+)\)").unwrap(), true)
        .scrape_links(link_matcher);

    println!("{:#?}", dbg_pls::pretty(&pickapart_surfs));

    Ok(())
}
