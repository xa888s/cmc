use crate::{
    error::{CrackmeError, CrackmeResult},
    next_parse, BaseCrackme, Language, Platform, Stats,
};
use scraper::{Html, Selector};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct OverviewCrackme<'html> {
    base: BaseCrackme<'html>,
    description: &'html str,
}

impl<'html> fmt::Display for OverviewCrackme<'html> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.base)?;
        writeln!(f, "Description: {}", self.description)
    }
}

impl<'a> OverviewCrackme<'a> {
    // TODO: Clean up this whole function
    pub fn with_full_html(html: &'a Html, id: &'a str) -> CrackmeResult<OverviewCrackme<'a>> {
        let selector = Selector::parse("div.columns.panel-background div.column.col-3").unwrap();

        // doing all our passes
        let mut info = html
            .select(&selector)
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

        let author = info.next().ok_or(CrackmeError::NotFound("author"))?;

        next_parse! {
            info,
            language: Language
        }

        let date = info.next().ok_or(CrackmeError::NotFound("upload"))?;

        next_parse! {
            info,
            platform: Platform,
            difficulty: f32,
            quality: f32
        }

        // get rid of known download flags
        info.nth(1);

        // TODO: add this back when we know for sure that it doesn't fail on valid pages
        // make sure there (probably) hasn't been a change in the format
        // assert!(info.next().is_none());

        let name = OverviewCrackme::parse_name(&html)?;

        let description = OverviewCrackme::fetch_description(&html)?;

        let stats = Stats {
            quality,
            difficulty,
        };

        let solutions = OverviewCrackme::fetch_solutions(&html);
        let comments = OverviewCrackme::fetch_comments(&html);

        // put together our crackme and return it
        let crackme = BaseCrackme {
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

        let overview = OverviewCrackme {
            base: crackme,
            description,
        };

        Ok(overview)
    }

    pub fn description(&self) -> &str {
        self.description
    }

    pub fn id(&self) -> &str {
        self.base.id
    }

    pub fn name(&self) -> &str {
        self.base.name
    }

    fn fetch_comments(html: &Html) -> u64 {
        let selector = Selector::parse("div#comments p").unwrap();

        html.select(&selector).count() as u64
    }

    fn fetch_solutions(html: &Html) -> u64 {
        let selector = Selector::parse("div#solutions div.col-9").unwrap();

        html.select(&selector).count() as u64
    }

    fn parse_name(html: &Html) -> CrackmeResult<&str> {
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
                .ok_or(CrackmeError::NotFound("h3"))?;

            // FIXME: use actual parsing
            text
        };

        Ok(name)
    }

    fn fetch_description(html: &Html) -> CrackmeResult<&str> {
        let selector = Selector::parse("div.columns div.col-12 span").unwrap();

        let description = html
            .select(&selector)
            .next()
            .and_then(|span| span.text().next());

        description.ok_or(CrackmeError::NotFound("description"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../static/test.html");

    #[test]
    fn parse_full_crackme() {
        let html = Html::parse_document(TEST_FILE);
        let id = "60816fca33c5d42f38520831";
        let crackme = OverviewCrackme::with_full_html(&html, id).unwrap();

        assert_eq!(
            crackme,
            OverviewCrackme {
                base: BaseCrackme {
                    name: "SAFE_01",
                    author: "oles",
                    date: "12:44 PM 04/22/2021",
                    platform: Platform::Windows,
                    language: Language::VisualBasic,
                    stats: Stats {
                        quality: 3.7,
                        difficulty: 1.0,
                    },
                    id,
                    solutions: 0,
                    comments: 2,
                },
                description: "easy crackme ..enjoy )",
            }
        );
    }
}
