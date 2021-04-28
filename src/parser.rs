use anyhow::Result;
use reqwest::Client;
use scraper::{Html, Selector};

const SEARCH_URL: &str = "https://crackmes.one/search";

pub struct CrackMe {
    name: String,
    author: String,
    rating: f32,
    difficulty: f32,
    platform: &'static str,
    solutions: u64,
    comments: u64,
}

pub async fn get_search_results<'a>(token: &'a str) -> Result<Vec<CrackMe>> {}

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
        .ok_or("Couldn't parse token")
        .unwrap();

    let token = element.value().attr("value").unwrap().to_owned();

    Ok(token)
}
