use crate::next_parse;
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::fmt::{self, Display, Formatter};
use strum::{Display, EnumString};

// we allow this so the mapping is more one to one
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Platform {
    DOS,
    #[strum(serialize = "Mac OS X", serialize = "macos")]
    MacOSX,
    #[strum(serialize = "Multiplatform", serialize = "multiplatform")]
    Multiplatform,
    #[strum(serialize = "Unix/linux etc.", serialize = "linux", serialize = "unix")]
    UnixLinux,
    #[strum(serialize = "Windows", serialize = "windows")]
    Windows,
    #[strum(serialize = "Windows 2000/XP only")]
    Windows2000XP,
    #[strum(serialize = "Windows 7 Only")]
    Windows7,
    #[strum(serialize = "Windows Vista Only")]
    WindowsVista,
    #[strum(serialize = "Unspecified/other", serialize = "other")]
    Other,
}

#[derive(Debug, PartialEq, EnumString, Display)]
pub enum Language {
    #[strum(serialize = "C/C++")]
    COrCPlusPlus,
    Assembler,
    Java,
    #[strum(serialize = "(Visual) Basic")]
    VisualBasic,
    #[strum(serialize = "Borland Delphi")]
    BorlandDelphi,
    #[strum(serialize = "Turbo Pascal")]
    TurboPascal,
    #[strum(serialize = ".NET")]
    DotNet,
    #[strum(serialize = "Unspecified/other")]
    Other,
}

// Holds the contents of a crackme
#[derive(Debug, PartialEq)]
pub struct CrackMe<'a> {
    html: &'a Html,
    name: &'a str,
    author: &'a str,
    language: Language,
    upload: &'a str,
    download_href: &'a str,
    platform: Platform,
    stats: Stats,
}

#[derive(Debug, PartialEq, Default)]
pub struct Stats {
    pub quality: f32,
    pub difficulty: f32,
}

impl Stats {
    pub fn new(quality: f32, difficulty: f32) -> Stats {
        Stats {
            quality,
            difficulty,
        }
    }
}

impl Display for CrackMe<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Upload: {}", self.upload)?;
        writeln!(
            f,
            "Download link: https://crackmes.one{}",
            self.download_href
        )?;
        writeln!(f, "Platform: {}", self.platform)?;
        writeln!(f, "Quality: {:.1}", self.stats.quality)?;
        write!(f, "Difficulty: {:.1}", self.stats.difficulty)
    }
}

impl CrackMe<'_> {
    /// ## Order of items:
    /// author
    /// language
    /// upload
    /// platform
    /// difficulty
    /// quality
    ///
    /// ## Iteration example:
    /// "Author:"
    /// "oles"
    /// "Language:"
    /// "(Visual) Basic"
    /// "Upload:"
    /// "12:44 PM 04/22/2021"
    /// "Platform"
    /// "Windows"
    /// "Difficulty:"
    /// "1.0"
    /// "Rate!"
    /// "Quality:"
    /// "4.5"
    /// "Rate!"
    ///
    // TODO: Clean up this whole function
    pub fn with_full_html(html: &Html) -> Result<CrackMe<'_>> {
        let selector = Selector::parse("div").unwrap();

        // doing all our passes
        let mut info = html
            .select(&selector)
            .filter(|e| e.value().classes().any(|s| s == "col-3"))
            .flat_map(|e| e.text())
            .filter(|t| !t.chars().all(char::is_whitespace))
            .map(|t| t.trim());

        // FIXME: Store the html and only hold &str instead of making a ton of new allocations
        let author = info.nth(1).ok_or_else(|| anyhow!("No author!"))?;

        next_parse! {
            info,
            language: Language
        }

        let upload = info.nth(1).ok_or_else(|| anyhow!("No upload date!"))?;

        next_parse! {
            info,
            platform: Platform,
            difficulty: f32,
            quality: f32
        }

        // make sure there (probably) hasn't been a change in the format
        assert!(info.next().is_none());

        let name = CrackMe::parse_name(&html)?;
        let download_href = CrackMe::parse_download_link(&html)?;

        let stats = Stats {
            difficulty,
            quality,
        };

        // put together our crackme and return it
        let crackme = CrackMe {
            html,
            name,
            upload,
            author,
            language,
            download_href,
            platform,
            stats,
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
            let text = html
                .select(&selector)
                .next()
                .ok_or_else(|| anyhow!("No h3 element for name!"))?
                .text()
                .nth(1)
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
        let element = html
            .select(&selector)
            .find(|e| e.value().classes().any(|c| c == "btn-download"))
            .ok_or_else(|| anyhow!("No element with btn-download"))?;

        let download_href = element
            .value()
            .attr("href")
            .ok_or_else(|| anyhow!("No href value"))?;

        Ok(download_href)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = include_str!("../static/test.html");

    #[test]
    fn parse_full_crackme() {
        let html = Html::parse_document(TEST_FILE);
        let crackme = CrackMe::with_full_html(&html).unwrap();

        assert_eq!(
            crackme,
            CrackMe {
                html: &html,
                name: "SAFE_01",
                author: "oles",
                upload: "12:44 PM 04/22/2021",
                platform: Platform::Windows,
                language: Language::VisualBasic,
                download_href: "/static/crackme/60816fca33c5d42f38520831.zip",
                stats: Stats {
                    quality: 4.5,
                    difficulty: 1.0,
                }
            }
        );
    }
}
