use anyhow::{format_err, Result};
use scraper::{Html, Selector};
use std::fmt::{self, Display, Formatter};

// Holds the contents of a crackme
#[derive(Debug, PartialEq, Default)]
pub struct CrackMe {
    name: String,
    author: String,
    language: String,
    upload: String,
    download_href: String,
    platform: String,
    stats: Stats,
}

#[derive(Debug, PartialEq, Default)]
struct Stats {
    quality: f32,
    difficulty: f32,
}

impl Display for CrackMe {
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
        writeln!(f, "Quality: {}", self.stats.quality)?;
        write!(f, "Difficulty: {}", self.stats.difficulty)
    }
}

impl CrackMe {
    pub fn with_full_html(html: &Html) -> Result<CrackMe> {
        CrackMe::parse_stats(&html)
    }

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
    fn parse_stats(html: &Html) -> Result<CrackMe> {
        let selector = Selector::parse("div").unwrap();

        let mut info = html
            .select(&selector)
            .filter(|e| e.value().classes().any(|s| s == "col-3"))
            .flat_map(|e| e.text())
            .filter(|t| !t.chars().all(char::is_whitespace))
            .map(|t| t.trim());

        // FIXME: Store the html and only hold &str instead of making a ton of new allocations
        let author = info
            .nth(1)
            .ok_or_else(|| format_err!("No author!"))?
            .to_string();

        let language = info
            .nth(1)
            .ok_or_else(|| format_err!("No language!"))?
            .to_string();

        let upload = info
            .nth(1)
            .ok_or_else(|| format_err!("No upload date!"))?
            .to_string();

        let platform = info
            .nth(1)
            .ok_or_else(|| format_err!("No platform!"))?
            .to_string();

        let difficulty: f32 = info
            .nth(1)
            .ok_or_else(|| format_err!("No difficulty!"))?
            .parse()?;

        let quality: f32 = info
            .nth(1)
            .ok_or_else(|| format_err!("No quality!"))?
            .parse()?;

        // make sure there (probably) hasn't been a change in the format
        assert!(info.next().is_none());

        let selector = Selector::parse("h3").unwrap();

        let name = {
            let text = html
                .select(&selector)
                .next()
                .ok_or_else(|| format_err!("No h3 element for name!"))?
                .inner_html();

            // FIXME: use actual parsing
            text.split("'s ")
                .nth(1)
                .ok_or_else(|| format_err!("No name!"))?
                .to_string()
        };

        Ok(CrackMe {
            name,
            upload,
            author,
            language,
            download_href: CrackMe::parse_download_link(&html)?,
            platform,
            stats: Stats {
                difficulty,
                quality,
            },
        })
    }

    pub fn download_href(&self) -> &str {
        &self.download_href
    }

    fn parse_download_link(html: &Html) -> Result<String> {
        // guaranteed to parse
        let selector = Selector::parse("a").unwrap();

        // finding the download link
        let element = html
            .select(&selector)
            .find(|e| e.value().classes().any(|c| c == "btn-download"))
            .ok_or_else(|| format_err!("No element with btn-download"))?;

        Ok(element
            .value()
            .attr("href")
            .ok_or_else(|| format_err!("No href value"))?
            .to_string())
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
                name: "SAFE_01".to_string(),
                author: "oles".to_string(),
                upload: "12:44 PM 04/22/2021".to_string(),
                platform: "Windows".to_string(),
                language: "(Visual) Basic".to_string(),
                download_href: "/static/crackme/60816fca33c5d42f38520831.zip".to_string(),
                stats: Stats {
                    quality: 4.5,
                    difficulty: 1.0,
                }
            }
        );
    }
}
