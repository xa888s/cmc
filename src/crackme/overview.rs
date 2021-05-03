use crate::{
    crackme::{CrackMe, Language, Platform, Stats},
    next_parse,
};
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::fmt::{self, Display, Formatter};

// Holds the contents of a crackme
#[derive(Debug, PartialEq)]
pub struct OverviewCrackMe<'a> {
    download_href: &'a str,
    description: &'a str,
}

impl<'a> OverviewCrackMe<'a> {
    pub fn new(download_href: &'a str, description: &'a str) -> OverviewCrackMe<'a> {
        OverviewCrackMe {
            download_href,
            description,
        }
    }
}

impl Display for OverviewCrackMe<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Download link: https://crackmes.one{}",
            self.download_href
        )?;
        write!(f, "Description: {}", self.description)
    }
}

impl<'a> OverviewCrackMe<'a> {
    // TODO: Clean up this whole function
    pub fn with_full_html(html: &'a Html) -> Result<CrackMe<'a, Self>> {
        let selector = Selector::parse("div").unwrap();

        // doing all our passes
        let mut info = html
            .select(&selector)
            .filter(|e| e.value().classes().any(|s| s == "col-3"))
            .flat_map(|e| e.text())
            .filter(|t| !t.chars().all(char::is_whitespace))
            .map(|t| t.trim())
            .skip(1)
            .step_by(2);

        // Order of items:
        // author
        // language
        // upload
        // platform
        // difficulty
        // quality

        let author = info.next().ok_or_else(|| anyhow!("No author!"))?;

        next_parse! {
            info,
            language: Language
        }

        let date = info.next().ok_or_else(|| anyhow!("No upload date!"))?;

        next_parse! {
            info,
            platform: Platform,
            difficulty: f32,
            quality: f32
        }

        // get rid of known download flags
        info.nth(1);

        // make sure there (probably) hasn't been a change in the format
        assert!(info.next().is_none());

        let name = OverviewCrackMe::parse_name(&html)?;
        let download_href = OverviewCrackMe::parse_download_link(&html)?;

        let description = OverviewCrackMe::get_description(&html)?;

        let stats = Stats {
            difficulty,
            quality,
        };

        let overview = OverviewCrackMe::new(download_href, description);

        // put together our crackme and return it
        let crackme = CrackMe {
            name,
            date,
            author,
            language,
            platform,
            stats,
            other: overview,
        };

        Ok(crackme)
    }

    pub fn download_href(&self) -> &str {
        &self.download_href
    }

    fn parse_name(html: &Html) -> Result<&str> {
        // the name is the only h3 element
        let selector = Selector::parse("h3").unwrap();

        let name = {
            // Input starts like "'s NAME_OF_CRACKME"
            // So we could just take the characters from 3..
            // but we check for "'s " to make sure our format is still correct (just a safeguard)
            let text = html
                .select(&selector)
                .next()
                .and_then(|t| t.text().nth(1))
                .and_then(|t| t.split("'s ").nth(1))
                .ok_or_else(|| anyhow!("No h3 element for name!"))?;

            // FIXME: use actual parsing
            text
        };

        Ok(name)
    }

    fn parse_download_link(html: &Html) -> Result<&str> {
        // guaranteed to parse
        let selector = Selector::parse("a").unwrap();

        // finding the download link
        let download_href = html
            .select(&selector)
            .find(|e| e.value().classes().any(|c| c == "btn-download"))
            .and_then(|a| a.value().attr("href"))
            .ok_or_else(|| anyhow!("No href value!"))?;

        Ok(download_href)
    }

    fn get_description(html: &Html) -> Result<&str> {
        let selector = Selector::parse("div").unwrap();

        let description = html
            .select(&selector)
            .filter(|div| div.value().classes().any(|class| class == "col-12"))
            .nth(1)
            .and_then(|div| div.select(&Selector::parse("span").unwrap()).next())
            .and_then(|span| span.text().next());

        description.ok_or_else(|| anyhow!("No description"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../../static/test.html");

    #[test]
    fn parse_full_crackme() {
        let html = Html::parse_document(TEST_FILE);
        let crackme = OverviewCrackMe::with_full_html(&html).unwrap();

        assert_eq!(
            crackme,
            CrackMe {
                name: "SAFE_01",
                author: "oles",
                date: "12:44 PM 04/22/2021",
                platform: Platform::Windows,
                language: Language::VisualBasic,
                stats: Stats {
                    quality: 4.5,
                    difficulty: 1.0,
                },
                other: OverviewCrackMe {
                    download_href: "/static/crackme/60816fca33c5d42f38520831.zip",
                    description: "easy crackme ..enjoy )",
                }
            }
        );
    }
}
