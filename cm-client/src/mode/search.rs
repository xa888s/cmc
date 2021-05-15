use crate::{
    cli::SearchArgs,
    mode::{self, get},
};
use crackmes::{
    list::{self, ListCrackMe},
    Html, Selector,
};

use anyhow::{anyhow, Result};
use reqwest::Client;

const SEARCH_URL: &str = "https://crackmes.one/search";

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

    let search = Html::parse_document(&search);

    let mut crackmes: Vec<ListCrackMe<'_>> = list::parse_list(&search)?;

    if let Some(crackme) = mode::get_choice(client, &mut crackmes).await? {
        get::handle_crackme(client, crackme.id()).await?;
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
