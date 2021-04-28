use anyhow::{format_err, Result};
use reqwest::Client;
use scraper::{Html, Selector};

const SEARCH_URL: &str = "https://crackmes.one/search";

// Holds the contents of a search result
#[derive(Debug, Default)]
pub struct CrackMe {
    name: String,
    author: String,
    rating: f32,
    difficulty: f32,
    platform: &'static str,
    solutions: u64,
    comments: u64,
}

// returns all the search results
pub async fn get_search_results<'a>(client: &mut Client, token: &'a str) -> Result<Vec<CrackMe>> {
    todo!()
}

// returns the token to allow searching
pub async fn get_token(client: &mut Client) -> Result<String> {
    let html = {
        let body = client.get(SEARCH_URL).send().await?.text().await?;
        Html::parse_document(&body)
    };

    let selector = Selector::parse("input").unwrap();

    let element = html
        .select(&selector)
        .find(|e| match e.value().id() {
            Some(s) => s == "token",
            _ => false,
        })
        .ok_or_else(|| format_err!("Couldn't parse token"))?;

    let token = element
        .value()
        .attr("value")
        .ok_or_else(|| format_err!("Token doesn't have a value attribute"))?
        .to_owned();

    Ok(token)
}
