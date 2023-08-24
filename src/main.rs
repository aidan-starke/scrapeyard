#![allow(dead_code)]
use regex::Regex;

#[derive(Debug, dbg_pls::DebugPls)]
struct Surf {
    model: String,
    count: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://www.pickapart.co.nz/Takanini-Stock");

    let html_content = response.unwrap().text().unwrap();

    // TODO: actually find surf
    let has_surf = Regex::new(r"Voltz (\w+) \((\d+)\)").unwrap();

    let mut results = vec![];
    for (_, [model, count]) in has_surf.captures_iter(&html_content).map(|c| c.extract()) {
        results.push(Surf {
            model: model.to_string(),
            count: count.parse().unwrap(),
        });
    }

    println!("{:#?}", dbg_pls::pretty(&results));

    // TODO: Get link to surf

    Ok(())
}
