use regex::Regex;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

    pub fn print(&self) {
        println!("{:#?}", self.surfs);
    }
}

pub trait SurfScraper {
    fn format_link(&self, link: String) -> String;

    fn scrape_page(self, has_surf: Regex) -> Self;

    fn scrape_links(self, link_matcher: Box<dyn Fn(String) -> Vec<Regex>>) -> Self;
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
                            .collect::<Vec<String>>();

                        acc.append(&mut links);

                        acc
                    },
                );

                surf
            })
            .collect();

        self
    }
}
