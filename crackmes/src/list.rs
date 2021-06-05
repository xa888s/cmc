use super::BaseCrackme;
use crate::{
    error::{CrackmeError, CrackmeResult},
    next_parse, Language, Platform, Stats,
};
use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct ListCrackme<'html> {
    base: BaseCrackme<'html>,
    description: Option<String>,
}

impl<'html> fmt::Display for ListCrackme<'html> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.base)?;
        writeln!(
            f,
            "Description: {}",
            self.description
                .as_deref()
                .unwrap_or("No description found")
        )
    }
}

impl<'a> ListCrackme<'a> {
    pub fn to_search_string(&self) -> String {
        format!(
            "{}{}{}{}{}{:.1}{:.1}{}{}{}{}",
            self.base.name,
            self.base.author,
            self.base.language,
            self.base.date,
            self.base.platform,
            self.base.stats.quality,
            self.base.stats.difficulty,
            self.base.id,
            self.base.solutions,
            self.base.comments,
            self.description.as_deref().unwrap_or_default()
        )
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn name(&self) -> &str {
        self.base.name
    }

    pub fn author(&self) -> &str {
        self.base.author
    }

    pub fn id(&self) -> &str {
        self.base.id
    }

    pub fn try_set_description(&mut self, s: String) -> Result<(), String> {
        if self.description.is_none() {
            self.description = Some(s);
            Ok(())
        } else {
            Err(s)
        }
    }
}

pub fn parse_list(html: &Html) -> CrackmeResult<Vec<ListCrackme<'_>>> {
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
                .ok_or(CrackmeError::NotFound("ID"))?;

            Ok((id, rest))
        })
        .map(|info| info.and_then(parse_row))
        .collect();

    crackmes
}

pub fn parse_row<'a>(
    (id, mut tr): (&'a str, impl Iterator<Item = &'a str>),
) -> CrackmeResult<ListCrackme<'a>> {
    let (name, author) = (
        tr.next().ok_or(CrackmeError::NotFound("name"))?,
        tr.next().ok_or(CrackmeError::NotFound("author"))?,
    );

    next_parse! {
        tr,
        language: Language,
        difficulty: f32,
        quality: f32,
        platform: Platform
    }

    let date: &str = tr.next().ok_or(CrackmeError::NotFound("date"))?;

    next_parse! {
        tr,
        solutions: u64,
        comments: u64
    }

    assert!(tr.next().is_none());

    let stats = Stats::new(quality, difficulty);

    let base = BaseCrackme {
        name,
        author,
        language,
        date,
        platform,
        stats,
        id,
        solutions,
        comments,
    };

    let list = ListCrackme {
        base,
        description: None,
    };

    Ok(list)
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_SEARCH_FILE: &str = include_str!("../static/search_test.html");
    const TEST_LATEST_FILE: &str = include_str!("../static/latest_test.html");

    #[test]
    fn parse_latest_text() {
        let html = Html::parse_document(TEST_LATEST_FILE);
        let latest = html;
        let crackmes: Vec<ListCrackme<'_>> = parse_list(&latest).unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&ListCrackme {
                base: BaseCrackme {
                    name: "EZwan",
                    author: "DirkD",
                    language: Language::COrCPlusPlus,
                    platform: Platform::UnixLinux,
                    date: "5:40 PM 05/07/2021",
                    stats: Stats {
                        quality: 4.0,
                        difficulty: 1.0
                    },
                    id: "60957b9a33c5d458ce0ec88e",
                    solutions: 0,
                    comments: 1,
                },
                description: None,
            })
        );
    }

    #[test]
    fn parse_search_text() {
        let html = Html::parse_document(TEST_SEARCH_FILE);
        let search = html;
        let crackmes: Vec<ListCrackme<'_>> = parse_list(&search).unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&ListCrackme {
                base: BaseCrackme {
                    name: "EZwan",
                    author: "DirkD",
                    language: Language::COrCPlusPlus,
                    platform: Platform::UnixLinux,
                    date: "5:40 PM 05/07/2021",
                    stats: Stats {
                        quality: 4.0,
                        difficulty: 1.0
                    },
                    id: "60957b9a33c5d458ce0ec88e",
                    solutions: 0,
                    comments: 0,
                },
                description: None,
            })
        );
    }
}
