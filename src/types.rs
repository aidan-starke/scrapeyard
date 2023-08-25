use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Surf {
    pub model: String,
    pub count: u32,
    pub links: Vec<String>,
}

impl Surf {
    fn new(model: String, count: u32) -> Self {
        Self {
            model,
            count,
            links: Vec::with_capacity(count as usize),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Surfs {
    pub page_string: String,
    pub surf_link: String,
    pub surfs: Vec<Surf>,
}

impl Surfs {
    pub fn new(page_link: String, surf_link: String) -> Self {
        let response = reqwest::blocking::get(page_link);
        let page_string = response.unwrap().text().unwrap();

        Self {
            surf_link,
            page_string,
            surfs: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("{:#?}", self.surfs);
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.surfs).unwrap()
    }
}

pub trait SurfScraper {
    fn format_link(&self, link: String) -> String;

    fn scrape_page(self, has_surf: Regex) -> Self;

    fn scrape_links(self, link_matcher: Box<dyn Fn(String) -> Vec<Regex>>) -> Self;

    fn compare_and_write_surfs(self, location: String) -> Option<Vec<Surf>>;
}

impl SurfScraper for Surfs {
    fn format_link(&self, link_match: String) -> String {
        format!(
            "{}{}",
            self.surf_link.clone(),
            link_match.replace(" ", "%20")
        )
    }

    fn scrape_page(mut self, has_surf: Regex) -> Self {
        self.surfs = has_surf
            .captures_iter(&self.page_string)
            .map(|c| c.extract().1)
            .map(|[model, count]| Surf::new(model.to_string(), count.parse::<u32>().unwrap()))
            .collect();

        self
    }

    fn scrape_links(mut self, link_matcher: Box<dyn Fn(String) -> Vec<Regex>>) -> Self {
        self.surfs = self
            .surfs
            .clone()
            .into_iter()
            .map(|mut surf| {
                surf.links = link_matcher(surf.model.clone()).into_iter().fold(
                    vec![],
                    |mut acc, matcher| {
                        let mut links = matcher
                            .captures_iter(&self.page_string)
                            .map(|c| c.extract().1)
                            .map(|[link]| self.format_link(link.to_string()))
                            .collect::<Vec<_>>();

                        acc.append(&mut links);

                        acc
                    },
                );

                surf
            })
            .collect();

        self
    }

    fn compare_and_write_surfs(self, location: String) -> Option<Vec<Surf>> {
        let previous =
            fs::read_to_string(format!("{}.json", "data/".to_string() + location.as_str()))
                .expect(format!("Unable to read file for {}", location).as_str());

        let previous: Vec<Surf> = serde_json::from_str(&previous)
            .expect(format!("Unable to parse json for {}", location).as_str());

        let new_surfs = self
            .surfs
            .clone()
            .into_iter()
            .filter(|surf| !previous.contains(&surf))
            .map(|surf| {
                println!("New surf at {}, {:#?}", location, surf);

                fs::write(
                    format!("{}.json", "data/".to_string() + &location),
                    self.to_json(),
                )
                .expect(format!("Unable to write file for {}", location).as_str());

                surf
            })
            .collect::<Vec<_>>();

        if !new_surfs.is_empty() {
            Some(new_surfs)
        } else {
            None
        }
    }
}
