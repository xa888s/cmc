use super::CrackMe;
use crate::{
    error::{CrackMeError, CrackMeResult},
    next_parse, Language, Platform, Stats,
};
use scraper::{Html, Selector};

pub type ListCrackMe<'a> = CrackMe<'a>;

impl<'a> ListCrackMe<'a> {
    pub fn to_search_string(&self) -> String {
        format!(
            "{}{}{}{}{}{:.1}{:.1}{}{}{}{}",
            self.name,
            self.author,
            self.language,
            self.date,
            self.platform,
            self.stats.quality,
            self.stats.difficulty,
            self.id,
            self.solutions,
            self.comments,
            self.description.as_deref().unwrap_or("")
        )
    }
}

pub fn parse_list(html: &Html) -> CrackMeResult<Vec<ListCrackMe<'_>>> {
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
                .ok_or(CrackMeError::NotFound("ID"))?;

            Ok((id, rest))
        })
        .map(|info| info.and_then(parse_row))
        .collect();

    crackmes
}

pub fn parse_row<'a>(
    (id, mut tr): (&'a str, impl Iterator<Item = &'a str>),
) -> CrackMeResult<ListCrackMe<'a>> {
    let (name, author) = (
        tr.next().ok_or(CrackMeError::NotFound("name"))?,
        tr.next().ok_or(CrackMeError::NotFound("author"))?,
    );

    next_parse! {
        tr,
        language: Language,
        difficulty: f32,
        quality: f32,
        platform: Platform
    }

    let date: &str = tr.next().ok_or(CrackMeError::NotFound("date"))?;

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
        solutions,
        comments,
        description: None,
    };

    Ok(crackme)
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
        let crackmes: Vec<ListCrackMe<'_>> = parse_list(&latest).unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&CrackMe {
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
                description: None,
            })
        );
    }

    #[test]
    fn parse_search_text() {
        let html = Html::parse_document(TEST_SEARCH_FILE);
        let search = html;
        let crackmes: Vec<ListCrackMe<'_>> = parse_list(&search).unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&CrackMe {
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
                description: None,
            })
        );
    }
}
