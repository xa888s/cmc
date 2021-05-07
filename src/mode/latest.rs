use crate::{
    crackme::{latest::LatestPage, list::ListCrackMe, CrackMe},
    mode::{self, get},
};

use anyhow::Result;
use reqwest::Client;
use scraper::Html;
use std::convert::TryInto;

const LATEST_URL: &str = "https://crackmes.one/lasts/";

pub async fn handle_latest_results<'a>(client: &mut Client, number: u64) -> Result<()> {
    let html = {
        let body = client
            .get(format!("{}{}", LATEST_URL, number))
            .send()
            .await?
            .text()
            .await?;
        Html::parse_document(&body)
    };

    let latest = LatestPage(html);

    let crackmes: Vec<CrackMe<'_, ListCrackMe>> = (&latest).try_into()?;

    if let Some(id) = mode::get_choice(&crackmes) {
        get::handle_crackme(client, &id).await?;
    }

    Ok(())
}
