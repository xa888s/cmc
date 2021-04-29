use std::fmt;

use crate::{
    crackme::{Language, Platform, Stats},
    next_child, next_parse_inner,
};
use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};

const SEARCH_URL: &str = "https://crackmes.one/search";

#[derive(Debug, PartialEq)]
struct SearchCrackMe<'a> {
    name: &'a str,
    author: &'a str,
    language: Language,
    platform: Platform,
    date: &'a str,
    solutions: u64,
    comments: u64,
    stats: Stats,
}

impl<'a> fmt::Display for SearchCrackMe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Upload: {}", self.date)?;
        writeln!(f, "Platform: {}", self.platform)?;
        writeln!(f, "Quality: {}", self.stats.quality)?;
        write!(f, "Difficulty: {}", self.stats.difficulty)
    }
}

impl<'a> SearchCrackMe<'a> {
    pub fn with_element_iter(
        mut info: impl Iterator<Item = scraper::ElementRef<'a>>,
    ) -> Result<SearchCrackMe<'a>> {
        next_child! {
            info,
            name,
            author
        }

        next_parse_inner! {
            info,
            language: Language,
            difficulty: f32,
            quality: f32,
            platform: Platform
        }

        let date: &str = info
            .next()
            .and_then(|l| l.text().next().map(|s| s.trim()))
            .ok_or_else(|| anyhow!("No date"))?;

        next_parse_inner! {
            info,
            solutions: u64,
            comments: u64
        }

        let stats = Stats::new(quality, difficulty);

        let crackme = SearchCrackMe {
            name,
            author,
            language,
            platform,
            date,
            solutions,
            comments,
            stats,
        };

        Ok(crackme)
    }
}

// We have a seperate struct so that we can store the Html with it, saving some allocations
#[derive(Debug, PartialEq)]
pub struct SearchCrackMes<'a> {
    html: &'a Html,
    crackmes: Vec<SearchCrackMe<'a>>,
}

impl<'a> fmt::Display for SearchCrackMes<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.crackmes
            .iter()
            .take(self.crackmes.len() - 1)
            .try_for_each(|c| writeln!(f, "{}\n", c))?;

        // special case last
        if let Some(c) = self.crackmes.last() {
            write!(f, "{}", c)
        } else {
            Ok(())
        }
    }
}

impl<'a> SearchCrackMes<'a> {
    pub fn with_search_html(html: &'a Html) -> Result<SearchCrackMes<'a>> {
        let selector = Selector::parse("tr").unwrap();
        let inner_selector = Selector::parse("td").unwrap();

        // here we have each "crackme" in a list of "tr"s
        let mut table = html
            .select(&selector)
            .filter(|t| t.value().classes().any(|c| c == "text-center"))
            .map(|t| t.select(&inner_selector));

        // so we parse each td inside them, which gives the info
        let crackmes = {
            let crackmes: Vec<_> = table.try_fold::<_, _, Result<Vec<SearchCrackMe<'a>>>>(
                Vec::new(),
                |mut acc, info| {
                    acc.push(SearchCrackMe::with_element_iter(info)?);
                    Ok(acc)
                },
            )?;

            SearchCrackMes { html, crackmes }
        };

        Ok(crackmes)
    }
}

// returns all the search results
pub async fn handle_search_results<'a>(client: &mut Client) -> Result<()> {
    let html = {
        let body = client.get(SEARCH_URL).send().await?.text().await?;
        Html::parse_document(&body)
    };

    let token = get_token(&html)?;

    let params = [
        ("name", ""),
        ("author", ""),
        ("difficulty-min", "1"),
        ("difficulty-max", "6"),
        ("quality-min", "1"),
        ("quality-max", "6"),
        ("token", token),
    ];

    let search = client
        .post(SEARCH_URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let search = Html::parse_document(&search);

    let crackmes = SearchCrackMes::with_search_html(&search)?;

    println!("{}", crackmes);
    Ok(())
}

// returns the token to allow searching
fn get_token(html: &Html) -> Result<&str> {
    let selector = Selector::parse("input").unwrap();

    let element = html
        .select(&selector)
        .find(|e| match e.value().id() {
            Some(s) => s == "token",
            _ => false,
        })
        .ok_or_else(|| anyhow!("Couldn't parse token"))?;

    let token = element
        .value()
        .attr("value")
        .ok_or_else(|| anyhow!("Token doesn't have a value attribute"))?;

    Ok(token)
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_FILE: &str = include_str!("../static/search_test.html");

    #[test]
    fn parse_search_text() {
        let html = Html::parse_document(TEST_FILE);
        let crackme = SearchCrackMes::with_search_html(&html).unwrap();

        dbg!(crackme);

        //assert_eq!(
        //    crackme,
        //    CrackMe {
        //        html: &html,
        //        name: "SAFE_01",
        //        author: "oles",
        //        upload: "12:44 PM 04/22/2021",
        //        platform: Platform::Windows,
        //        language: Language::VisualBasic,
        //        download_href: "/static/crackme/60816fca33c5d42f38520831.zip",
        //        stats: Stats {
        //            quality: 4.5,
        //            difficulty: 1.0,
        //        }
        //    }
        //);
    }
}
