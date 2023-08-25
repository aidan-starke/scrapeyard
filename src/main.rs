mod libs;
pub mod types;

use std::{collections::HashMap, error::Error};

use libs::*;
use tokio::time;
pub use types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut interval = time::interval(time::Duration::from_secs(60 * 60 * 24));

    let pages = ["Avondale", "Takanini"].map(|location| {
        (
            location,
            format!("https://www.pickapart.co.nz/{}-Stock", location),
        )
    });

    loop {
        interval.tick().await;

        let mut page_strings = HashMap::new();
        for (location, page) in pages.clone() {
            let response = reqwest::get(page);
            let page_string = response.await?.text().await?;

            page_strings.insert(location.to_string(), page_string);
        }

        if let Some(new_surfs) = scrape_surfs(page_strings) {
            let _ = send_email(new_surfs);
        } else {
            println!("No new surfs found");
        }
    }
}
