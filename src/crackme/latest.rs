use crate::crackme::{
    list::{self, ListCrackMe},
    CrackMe,
};
use anyhow::Result;
use scraper::Html;
use std::convert::TryFrom;

// more typesafety
pub struct LatestPage(pub Html);

impl<'a> TryFrom<&'a LatestPage> for Vec<CrackMe<'a, ListCrackMe<'a>>> {
    type Error = anyhow::Error;

    fn try_from(LatestPage(html): &'a LatestPage) -> Result<Self, Self::Error> {
        list::parse_list(&html)
    }
}
