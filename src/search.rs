use std::fmt;

use crate::{
    crackme::{Language, Platform, Stats},
    next_parse,
};
use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use skim::prelude::*;

const SEARCH_URL: &str = "https://crackmes.one/search";

#[derive(Debug, PartialEq)]
pub struct SearchCrackMe<'a> {
    name: &'a str,
    author: &'a str,
    language: Language,
    platform: Platform,
    date: &'a str,
    solutions: u64,
    comments: u64,
    stats: Stats,
    id: &'a str,
}

impl<'a> fmt::Display for SearchCrackMe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Upload: {}", self.date)?;
        writeln!(f, "Platform: {}", self.platform)?;
        writeln!(f, "Quality: {}", self.stats.quality)?;
        writeln!(f, "ID: {}", self.id)?;
        write!(f, "Difficulty: {}", self.stats.difficulty)
    }
}

#[derive(Debug)]
struct SearchItem {
    text: String,
    preview: String,
    pub id: String,
}

impl SearchItem {
    pub fn with_search(crackme: &SearchCrackMe<'_>) -> SearchItem {
        SearchItem {
            text: format!("{} by {}", crackme.name, crackme.author),
            preview: crackme.to_string(),
            id: crackme.id.to_string(),
        }
    }
}

impl SkimItem for SearchItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.preview.clone())
    }
}

impl<'a> SearchCrackMe<'a> {
    pub fn with_search_html(html: &'a Html) -> Result<Vec<SearchCrackMe<'a>>> {
        let selector = Selector::parse("tr").unwrap();
        let inner_selector = Selector::parse("td").unwrap();

        // here we have each "crackme" in a list of "tr"s
        let mut table = html
            .select(&selector)
            .filter(|t| t.value().classes().any(|c| c == "text-center"))
            .map(|t| t.select(&inner_selector));

        // so we parse each td inside them, which gives the info
        let crackmes = table.try_fold::<_, _, Result<Vec<SearchCrackMe<'a>>>>(
            Vec::new(),
            |mut acc, info| {
                acc.push(SearchCrackMe::with_element_iter(info)?);
                Ok(acc)
            },
        )?;

        Ok(crackmes)
    }

    pub fn with_element_iter(
        info: impl Iterator<Item = scraper::ElementRef<'a>> + Clone,
    ) -> Result<SearchCrackMe<'a>> {
        // cloning the iterator, so it isn't that expensive
        let id: &str = info
            .clone()
            .next()
            .and_then(|td| td.children().nth(1))
            .and_then(|a| a.value().as_element())
            .and_then(|a| a.attr("href"))
            .and_then(|link| link.rsplit('/').next())
            .ok_or_else(|| anyhow!("No ID found"))?;

        let mut info = info
            .flat_map(|t| t.text())
            .filter(|t| !t.chars().all(char::is_whitespace))
            .map(|t| t.trim());

        let (name, author) = (
            info.next().ok_or_else(|| anyhow!("No name found!"))?,
            info.next().ok_or_else(|| anyhow!("No author found!"))?,
        );

        next_parse! {
            info,
            language: Language,
            difficulty: f32,
            quality: f32,
            platform: Platform
        }

        let date: &str = info.next().ok_or_else(|| anyhow!("No date"))?;

        next_parse! {
            info,
            solutions: u64,
            comments: u64
        }

        assert!(info.next().is_none());

        let stats = Stats::new(quality, difficulty);

        let crackme = SearchCrackMe {
            id,
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

// returns all the search results
pub async fn handle_search_results<'a>(
    client: &mut Client,
    args: crate::cli::SearchArgs,
) -> Result<()> {
    let html = {
        let body = client.get(SEARCH_URL).send().await?.text().await?;
        Html::parse_document(&body)
    };

    let token = get_token(&html)?;

    let mut params: Vec<(&str, String)> = vec![
        ("name", args.name.unwrap_or_default()),
        ("author", args.author.unwrap_or_default()),
        ("difficulty-min", args.difficulty.0.to_string()),
        ("difficulty-max", args.difficulty.1.to_string()),
        ("quality-min", args.quality.0.to_string()),
        ("quality-max", args.quality.1.to_string()),
        ("token", token.to_string()),
    ];

    if let Some(l) = args.language {
        params.push(("lang", l.to_string()));
    }

    if let Some(p) = args.platform {
        params.push(("platform", p.to_string()));
    }

    let search = client
        .post(SEARCH_URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let search = Html::parse_document(&search);

    let crackmes = SearchCrackMe::with_search_html(&search)?;

    if let Some(id) = get_choice(&crackmes) {
        crate::get::handle_crackme(client, &id).await?;
    }

    Ok(())
}

// TODO: Optimize this
fn get_choice(input: &[SearchCrackMe<'_>]) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for item in input.iter().map(SearchItem::with_search) {
        tx.send(Arc::new(item)).ok()?;
    }
    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .and_then(|out| (!out.is_abort).then(|| out.selected_items))?;

    selected_items
        .get(0)
        .and_then(|i| (**i).as_any().downcast_ref::<SearchItem>())
        .map(|s| s.id.clone())
}

// returns the token to allow searching
fn get_token(html: &Html) -> Result<&str> {
    let selector = Selector::parse("#token").unwrap();

    let token = html
        .select(&selector)
        .next()
        .and_then(|t| t.value().attr("value"))
        .ok_or_else(|| anyhow!("Couldn't parse token"))?;

    Ok(token)
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_FILE: &str = include_str!("../static/search_test.html");

    #[test]
    fn parse_search_text() {
        let html = Html::parse_document(TEST_FILE);
        let crackmes = SearchCrackMe::with_search_html(&html).unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&SearchCrackMe {
                id: "60816fca33c5d42f38520831",
                name: "SAFE_01",
                author: "oles",
                language: Language::VisualBasic,
                platform: Platform::Windows,
                date: "12:44 PM 04/22/2021",
                solutions: 0,
                comments: 0,
                stats: Stats {
                    quality: 4.5,
                    difficulty: 1.0
                }
            })
        );
    }
}
