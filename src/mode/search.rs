use crate::{
    cli::SearchArgs,
    crackme::{
        list::{ListCrackMe, ListItem},
        search::{SearchPage, SEARCH_URL},
        CrackMe,
    },
    mode::get,
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

    let crackmes: Vec<CrackMe<'_, ListCrackMe<'_>>> = (&search).try_into()?;

    if let Some(id) = get_choice(&crackmes) {
        get::handle_crackme(client, &id).await?;
    }

    Ok(())
}

// TODO: Optimize this
fn get_choice(input: &[CrackMe<'_, ListCrackMe<'_>>]) -> Option<String> {
    use skim::prelude::*;
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for item in input.iter().map(ListItem::with_search) {
        tx.send(Arc::new(item)).ok()?;
    }
    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .and_then(|out| (!out.is_abort).then(|| out.selected_items))?;

    selected_items
        .get(0)
        .and_then(|i| (**i).as_any().downcast_ref::<ListItem>())
        .map(|s| s.id.clone())
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
