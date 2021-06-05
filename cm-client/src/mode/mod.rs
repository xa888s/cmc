use crate::tui::{
    search::{SearchText, Searcher},
    term,
};

use anyhow::Result;
use crackmes::list::ListCrackme;
use crossterm::event::{Event, EventStream, KeyModifiers};
use futures_util::stream::StreamExt;
use reqwest::Client;

pub mod get;
pub mod latest;
pub mod search;

// TODO: Optimize this
pub async fn get_choice<'a>(
    client: &mut Client,
    input: &'a mut [ListCrackme<'a>],
) -> Result<Option<&'a ListCrackme<'a>>> {
    let mut term = term::get_term()?;

    let mut events = EventStream::new();

    let mut searcher = Searcher::new(input);
    let mut search_text = SearchText::default();

    searcher.fetch_descriptions(client).await?;

    term::draw(&mut term, &search_text, &mut searcher)?;

    while let Some(e) = events.next().await.transpose()? {
        if let Event::Key(k) = e {
            use crossterm::event::KeyCode::*;
            match k.code {
                Enter => {
                    break;
                }
                Esc => return Ok(None),
                Up => searcher.previous(),
                Char('k') if k.modifiers == KeyModifiers::CONTROL => searcher.previous(),
                Down => searcher.next(),
                Char('j') if k.modifiers == KeyModifiers::CONTROL => searcher.next(),
                Char(c) => {
                    search_text.push(c);
                    searcher.search(search_text.as_str());
                }
                Backspace => {
                    search_text.pop();
                    searcher.search(search_text.as_str())
                }
                _ => break,
            }
            searcher.fetch_descriptions(client).await?;
        }

        term::draw(&mut term, &search_text, &mut searcher)?;
    }

    term::close_term(term)?;

    Ok(searcher.into_selected())
}
