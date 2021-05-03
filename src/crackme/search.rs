use super::{
    list::{self, ListCrackMe},
    CrackMe,
};
use std::convert::TryFrom;

use anyhow::Result;
use scraper::Html;

pub const SEARCH_URL: &str = "https://crackmes.one/search";

// Some typesafety stuff
pub struct SearchPage(pub Html);

impl<'a> TryFrom<&'a SearchPage> for Vec<CrackMe<'a, ListCrackMe>> {
    type Error = anyhow::Error;

    fn try_from(SearchPage(html): &'a SearchPage) -> Result<Self, Self::Error> {
        list::parse_list(&html)
    }
}
