use regex::Regex;

use crate::types::*;
use html_node::text;
use html_node::typed::{
    elements::{a, b, body, div, h1, h2, html, li, p, ul},
    html,
};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::collections::HashMap;
use std::error::Error;

pub fn scrape_surfs() -> Option<HashMap<String, Vec<Surf>>> {
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

    let mut new_surfs = HashMap::new();

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

            if let Some(surfs) = surfs.compare_and_write_surfs(location.to_string()) {
                new_surfs.insert(location.to_string(), surfs);
            }
        });

    if new_surfs.is_empty() {
        None
    } else {
        Some(new_surfs)
    }
}

pub fn send_email(surfs: HashMap<String, Vec<Surf>>) -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;

    let email_body = html!(
            <html>
                <body>
                    <div>
                        <h1>"New Surfs at pickapart"</h1>
                        {surfs.into_iter().map(|(location, surfs)| html!(
                                <h2>{text!{"{}", location}}</h2>

                                {surfs.into_iter().map(|surf| html!(
                                    <p>
                                        <b>{text!("{}", surf.model)}</b>
                                        {text!(" ({})", surf.count)}
                                    </p>

                                    <ul>
                                        {surf.links.into_iter().zip(0..).map(|(link, i)| html!(
                                            <li>
                                                <a href={link.clone()}>{text!("Link {}", i + 1)}</a>
                                            </li>
                                        ))}
                                    </ul>
                                ))}
                        ))}
                    </div>
                </body>
            </html>
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(Credentials::new(
            "starke.aidan".to_string(),
            dotenv::var("GMAIL_PASSWORD").unwrap(),
        ))
        .build();

    let email = Message::builder()
        .from("Scrapeyard <starke.aidan@gmail.com>".parse().unwrap())
        .to("Me <starke.aidan@gmail.com>".parse().unwrap())
        .subject("New Surfs")
        .header(ContentType::TEXT_HTML)
        .body(email_body.to_string())
        .unwrap();

    mailer.send(&email)?;

    Ok(())
}
