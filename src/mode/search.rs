use crate::{
    cli::SearchArgs,
    crackme::{
        list::ListCrackMe,
        search::{SearchPage, SEARCH_URL},
        CrackMe,
    },
    mode::{get, self},
};

use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::convert::TryInto;

// returns all the search results
pub async fn handle_search_results<'a>(client: &mut Client, args: SearchArgs) -> Result<()> {
    let html = {
        let body = client.get(SEARCH_URL).send().await?.text().await?;
        Html::parse_document(&body)
    };

    let token = get_token(&html)?;

    let mut params = vec![
        ("name", args.name.unwrap_or_default()),
        ("author", args.author.unwrap_or_default()),
        ("difficulty-min", args.difficulty.0.to_string()),
        ("difficulty-max", args.difficulty.1.to_string()),
        ("quality-min", args.quality.0.to_string()),
        ("quality-max", args.quality.1.to_string()),
        ("token", token.to_string()),
    ];

    if let Some(l) = args.language {
        params.push(("lang", l.to_string()));
    }

    if let Some(p) = args.platform {
        params.push(("platform", p.to_string()));
    }

    let search = client
        .post(SEARCH_URL)
        .form(&params)
        .send()
        .await?
        .text()
        .await?;

    let search = SearchPage(Html::parse_document(&search));

    let crackmes: Vec<CrackMe<'_, ListCrackMe>> = (&search).try_into()?;

    if let Some(id) = mode::get_choice(&crackmes) {
        get::handle_crackme(client, &id).await?;
    }

    Ok(())
}

// returns the token to allow searching
fn get_token(html: &Html) -> Result<&str> {
    let selector = Selector::parse("#token").unwrap();

    let token = html
        .select(&selector)
        .next()
        .and_then(|t| t.value().attr("value"))
        .ok_or_else(|| anyhow!("Couldn't parse token"))?;

    Ok(token)
}
