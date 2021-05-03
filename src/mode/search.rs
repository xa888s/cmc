use crate::crackme::search::{SearchCrackMe, SearchItem, SEARCH_URL};
use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};

// returns all the search results
pub async fn handle_search_results<'a>(
    client: &mut Client,
    args: crate::cli::SearchArgs,
) -> Result<()> {
    let html = {
        let body = client.get(SEARCH_URL).send().await?.text().await?;
        Html::parse_document(&body)
    };

    let token = get_token(&html)?;

    let mut params: Vec<(&str, String)> = vec![
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

    let crackmes = SearchCrackMe::with_search_html(&search)?;

    if let Some(id) = get_choice(&crackmes) {
        crate::mode::handle_crackme(client, &id).await?;
    }

    Ok(())
}

// TODO: Optimize this
fn get_choice(input: &[SearchCrackMe<'_>]) -> Option<String> {
    use skim::prelude::*;
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for item in input.iter().map(SearchItem::with_search) {
        tx.send(Arc::new(item)).ok()?;
    }
    drop(tx);

    let selected_items = Skim::run_with(&options, Some(rx))
        .and_then(|out| (!out.is_abort).then(|| out.selected_items))?;

    selected_items
        .get(0)
        .and_then(|i| (**i).as_any().downcast_ref::<SearchItem>())
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

#[cfg(test)]
mod test {
    use super::*;
    const TEST_FILE: &str = include_str!("../static/search_test.html");

    #[test]
    fn parse_search_text() {
        let html = Html::parse_document(TEST_FILE);
        let crackmes = SearchCrackMe::with_search_html(&html).unwrap();

        assert_eq!(
            crackmes.first(),
            Some(&SearchCrackMe {
                id: "60816fca33c5d42f38520831",
                name: "SAFE_01",
                author: "oles",
                language: Language::VisualBasic,
                platform: Platform::Windows,
                date: "12:44 PM 04/22/2021",
                solutions: 0,
                comments: 0,
                stats: Stats {
                    quality: 4.5,
                    difficulty: 1.0
                }
            })
        );
    }
}
