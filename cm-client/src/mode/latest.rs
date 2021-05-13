use crate::mode::{self, get};
use crackmes::{
    list::{self, ListCrackMe},
    Html,
};

use anyhow::Result;
use reqwest::Client;

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

    let latest = html;

    let crackmes: Vec<ListCrackMe<'_>> = list::parse_list(&latest)?;

    if let Some(crackme) = mode::get_choice(client, &crackmes).await? {
        get::handle_crackme(client, crackme.id()).await?;
    }

    Ok(())
}
