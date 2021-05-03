use std::{borrow::Cow, fmt};

use super::overview::{Language, Platform, Stats};
use crate::next_parse;

use anyhow::{anyhow, Result};
use scraper::{Html, Selector};

// For the CLI
use skim::{ItemPreview, PreviewContext, SkimItem};

pub const SEARCH_URL: &str = "https://crackmes.one/search";

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
pub struct SearchItem {
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

    // TODO: Find a way to show the description of each search result
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
