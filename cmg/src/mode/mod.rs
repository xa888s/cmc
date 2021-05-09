use crate::tui::state::StatefulList;
use crate::tui::term::{self, Search};
use anyhow::Result;
use crackmes::list::ListCrackMe;
use crossterm::event::{Event, EventStream, KeyModifiers};
use futures_util::stream::StreamExt;
use reqwest::Client;

pub mod get;
pub mod latest;
pub mod search;

// TODO: Optimize this
pub async fn get_choice<'a>(
    client: &mut Client,
    input: &'a [ListCrackMe<'a>],
) -> Result<Option<ListCrackMe<'a>>> {
    let mut term = term::get_term()?;

    let mut events = EventStream::new();

    let mut search = Search::new(input);
    let mut list = StatefulList::with_items(search.search());

    list.last();

    let mut description = get_description(client, &list).await?;

    term::draw(&mut term, &search, &mut list, &description)?;

    let mut selected = false;

    while let Some(e) = events.next().await.transpose()? {
        if let Event::Key(k) = e {
            use crossterm::event::KeyCode::*;
            match k.code {
                Enter => {
                    selected = true;
                    break;
                }
                Esc => break,
                Up => list.previous(),
                Char('k') if k.modifiers == KeyModifiers::CONTROL => list.previous(),
                Down => list.next(),
                Char('j') if k.modifiers == KeyModifiers::CONTROL => list.next(),
                Char(c) => {
                    search.push(c);
                    list.unselect();
                    list.items = search.search();
                    list.last();
                }
                Backspace => {
                    search.pop();
                    list.unselect();
                    list.items = search.search();
                    list.last();
                }
                _ => break,
            }
            description = get_description(client, &list).await?;
        }

        term::draw(&mut term, &search, &mut list, &description)?;
    }

    term::close_term(term)?;

    // TODO: fix the return type to not have double indirection
    Ok(if selected {
        list.selected().map(|c| (**c).clone())
    } else {
        None
    })
}

async fn get_description(
    client: &mut Client,
    list: &StatefulList<&'_ ListCrackMe<'_>>,
) -> Result<String> {
    Ok(if let Some(l) = list.selected() {
        format!(
            "{}\nDescription:\n{}",
            l,
            crate::get::get_description(client, l.id()).await?
        )
    } else {
        String::new()
    })
}
