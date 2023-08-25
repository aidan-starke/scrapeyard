use regex::Regex;

#[derive(Clone, Debug)]
pub struct Surf {
    pub model: String,
    pub count: Option<u32>,
    pub links: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Surfs {
    pub page_link: String,
    pub surf_link: String,
    pub surfs: Vec<Surf>,
}

impl Surf {
    fn new(model: String, count: Option<u32>) -> Self {
        let links = if count.is_some() {
            Vec::with_capacity(count.unwrap() as usize)
        } else {
            Vec::new()
        };

        Self {
            model,
            count,
            links,
        }
    }
}

pub trait SurfScraper {
    fn get_page_string(&self) -> String;

    fn format_link(&self, link: String) -> String;

    fn scrape_page(self, has_surf: Regex, includes_count: bool) -> Self;

    fn scrape_links(self, link_matcher: Box<dyn Fn(String) -> Vec<Regex>>) -> Self;
}

impl SurfScraper for Surfs {
    fn get_page_string(&self) -> String {
        let response = reqwest::blocking::get(self.page_link.clone());

        response.unwrap().text().unwrap()
    }
    fn format_link(&self, link_match: String) -> String {
        format!(
            "{}{}",
            self.surf_link.clone(),
            link_match.replace(" ", "%20")
        )
    }

    fn scrape_page(mut self, has_surf: Regex, includes_count: bool) -> Self {
        let html_content = self.get_page_string();

        let mut results = vec![];
        if includes_count {
            has_surf
                .captures_iter(&html_content)
                .map(|c| c.extract())
                .for_each(|(_, [model, count])| {
                    let surf = Surf::new(model.to_string(), Some(count.parse::<u32>().unwrap()));

                    results.push(surf);
                });
        } else {
            has_surf
                .captures_iter(&html_content)
                .map(|c| c.extract())
                .for_each(|(_, [model])| {
                    let surf = Surf::new(model.to_string(), None);

                    results.push(surf);
                });
        }

        self.surfs = results;

        self
    }

    fn scrape_links(mut self, link_matcher: Box<dyn Fn(String) -> Vec<Regex>>) -> Self {
        let html_content = self.get_page_string();

        let mut results = vec![];
        for surf in self.surfs.clone() {
            let mut matches = vec![];

            for matcher in link_matcher(surf.model.clone()) {
                matcher
                    .captures_iter(&html_content)
                    .map(|c| c.extract())
                    .for_each(|(_, [link])| {
                        matches.push(self.format_link(link.to_string()));
                    });
            }

            results.push(Surf {
                links: matches,
                ..surf
            })
        }

        self.surfs = results;

        self
    }
}
