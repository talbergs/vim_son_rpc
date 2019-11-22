use parse_wiki_text::{Configuration, Node};
use reqwest;
use reqwest::Url;
use std::error::Error;
use select::document::Document;
use select::predicate::{Attr};
use std::fs::{self, OpenOptions};
use std::io::Write;

#[derive(Debug)]
struct Err {
    pub msg: String,
}

impl Err {
    pub fn new(msg: &str) -> Self {
        Self {msg: msg.into()}
    }
}

impl <T>From<T> for Err where T: Error {
    fn from(e: T) -> Self {
        Err { msg: e.description().into() }
    }
}

struct Crawler {
    version: f32,
    url: Url,
    targets_visited: Vec<String>,
}

impl Crawler {
    pub fn new(url: &str, version: &str) -> Result<Self, Err> {
        let url = Url::parse(url)?;
        let version = version.parse::<f32>()?;
        Ok(Self {url, version, targets_visited: Vec::new()})
    }

    fn get_links<'a>(nodes: &'a Vec<Node>, links: &mut Vec<String>) {
        for node in nodes {
            match node {
                Node::ExternalLink { nodes, .. } |
                Node::Heading { nodes, .. }      |
                Node::Preformatted { nodes, .. } |
                Node::Tag { nodes, .. }          => Self::get_links(&nodes, links),
                Node::Link { target, .. }        => links.push(target.clone().into()),
                _                                => ()
            }
        }
    }

    fn get_wikitext(url: Url) -> Result<String, Err> {
        let body = reqwest::get(url)?.text()?;

        Ok(Document::from(body.as_str())
            .find(Attr("name", "wikitext"))
            .nth(0)
            .ok_or(Err::new("No textarea element found."))?
            .text())
    }

    fn extract_targets(wikitext: &str) -> Vec<String> {
        let result = Configuration::default().parse(wikitext);
        let mut links: Vec<String> = Vec::new();
        Self::get_links(&result.nodes, &mut links);
        links
    }

    pub fn begin(&mut self, target: &str) -> Result<(), Err> {
        self.targets_visited.push(target.into());
        println!("{}", target);

        let ds_target = target.replace(":", "/");
        let mut url = self.url
            .join(&format!("{}/", self.version))?
            .join(&ds_target)?;

        url.set_query(Some("do=edit"));

        let wikitext = Self::get_wikitext(url)?;
        for next_target in Self::extract_targets(&wikitext) {
            // if next_target.contains("#") {
            //     println!("  >{}", next_target);
            //     continue;
            // }

            if self.targets_visited.contains(&next_target) {
                // println!("  >>>{}", next_target);
                continue;
            }

            let common = next_target.trim_start_matches(target).len();

            if common != next_target.len() && common != 0 {
                self.begin(&next_target)?;
            }
        }

        let dirname = format!("{}/{}", self.version, ds_target);
        let filename = format!("{}/{}", &dirname, target);
        fs::create_dir_all(&dirname)?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&filename)?;

        file.write_all(wikitext.as_bytes())?;

        Ok(())
    }
}

fn main() -> Result<(), Err> {
    let mut crawler = Crawler::new(
        "https://www.zabbix.com/documentation/",
        "4.4",
    )?;

    crawler.begin("manual:api:reference")
}
