use std::{borrow::Cow, fmt};

use super::CrackMe;
use crate::{
    crackme::{Language, Platform, Stats},
    next_parse,
};
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};

// For the CLI
use skim::{ItemPreview, PreviewContext, SkimItem};

#[derive(Debug, PartialEq)]
pub struct ListCrackMe {
    solutions: u64,
    comments: u64,
}

impl fmt::Display for ListCrackMe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Solutions: {}", self.solutions)?;
        writeln!(f, "Comments: {}", self.comments)
    }
}

#[derive(Debug)]
pub struct ListItem {
    text: String,
    preview: String,
    pub id: String,
}

impl ListItem {
    pub fn with_search(crackme: &CrackMe<'_, ListCrackMe>) -> ListItem {
        ListItem {
            text: format!("{} by {}", crackme.name, crackme.author),
            preview: crackme.to_string(),
            id: crackme.id().to_string(),
        }
    }
}

pub fn parse_list(html: &Html) -> Result<Vec<CrackMe<'_, ListCrackMe>>> {
    let selector = Selector::parse("#content-list .text-center").unwrap();

    let crackmes = html
        .select(&selector)
        .map(|tr| {
            let rest = tr
                .text()
                .filter(|t| !t.chars().all(char::is_whitespace))
                .map(|t| t.trim());

            let id = tr
                .select(&Selector::parse("td a[href^=\"/crackme/\"]").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .and_then(|link| link.rsplit('/').next())
                .ok_or_else(|| anyhow!("No ID"))?;

            Ok((id, rest))
        })
        .map(|info| info.and_then(parse_row))
        .collect();

    crackmes
}

pub fn parse_row<'a>(
    (id, mut tr): (&'a str, impl Iterator<Item = &'a str>),
) -> Result<CrackMe<'a, ListCrackMe>> {
    let (name, author) = (
        tr.next().ok_or_else(|| anyhow!("No name found!"))?,
        tr.next().ok_or_else(|| anyhow!("No author found!"))?,
    );

    next_parse! {
        tr,
        language: Language,
        difficulty: f32,
        quality: f32,
        platform: Platform
    }

    let date: &str = tr.next().ok_or_else(|| anyhow!("No date"))?;

    next_parse! {
        tr,
        solutions: u64,
        comments: u64
    }

    assert!(tr.next().is_none());

    let stats = Stats::new(quality, difficulty);

    let crackme = CrackMe {
        name,
        author,
        language,
        platform,
        date,
        stats,
        id,
        other: ListCrackMe {
            solutions,
            comments,
        },
    };

    Ok(crackme)
}

impl SkimItem for ListItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.text)
    }

    // TODO: Find a way to show the description of each search result
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.preview.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::crackme::{latest::LatestPage, search::SearchPage};
    use std::convert::TryInto;
    const TEST_SEARCH_FILE: &str = include_str!("../../static/search_test.html");
    const TEST_LATEST_FILE: &str = include_str!("../../static/latest_test.html");

    #[test]
    fn parse_latest_text() {
        let html = Html::parse_document(TEST_LATEST_FILE);
        let search = LatestPage(html);
        let crackmes: Vec<CrackMe<'_, _>> = (&search).try_into().unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&CrackMe {
                name: "Crack Me",
                author: "Segfault21",
                language: Language::DotNet,
                platform: Platform::Windows,
                date: "3:07 AM 04/30/2021",
                stats: Stats {
                    quality: 4.0,
                    difficulty: 4.0
                },
                id: "608b747633c5d458ce0ec753",
                other: ListCrackMe {
                    comments: 1,
                    solutions: 0,
                }
            })
        );
    }

    #[test]
    fn parse_search_text() {
        let html = Html::parse_document(TEST_SEARCH_FILE);
        let search = SearchPage(html);
        let crackmes: Vec<CrackMe<'_, _>> = (&search).try_into().unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&CrackMe {
                name: "SAFE_01",
                author: "oles",
                language: Language::VisualBasic,
                platform: Platform::Windows,
                date: "12:44 PM 04/22/2021",
                stats: Stats {
                    quality: 4.5,
                    difficulty: 1.0
                },
                id: "60816fca33c5d42f38520831",
                other: ListCrackMe {
                    solutions: 0,
                    comments: 0,
                }
            })
        );
    }
}
