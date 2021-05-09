use anyhow::Result;
use crackmes::list::ListCrackMe;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::io;

use crate::tui::state::StatefulList;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use tui::{backend::CrosstermBackend, Terminal};

pub type Term = Terminal<CrosstermBackend<io::Stdout>>;

pub fn get_term() -> Result<Term> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    enable_raw_mode()?;
    term.backend_mut()
        .execute(EnterAlternateScreen)?
        .execute(EnableMouseCapture)?;
    term.clear()?;
    Ok(term)
}

pub fn close_term(mut term: Term) -> Result<()> {
    term.show_cursor()?;
    disable_raw_mode()?;
    term.backend_mut()
        .execute(LeaveAlternateScreen)?
        .execute(DisableMouseCapture)?;
    Ok(())
}

pub fn draw<'a>(
    term: &mut Term,
    search: &Search<'a>,
    list: &mut StatefulList<&ListCrackMe<'a>>,
    description: &str,
) -> Result<()> {
    term.draw(|f| {
        let whole = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());

        let mut chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
            .split(whole[0]);

        chunks.push(whole[1]);

        let items: Vec<ListItem> = list
            .items
            .iter()
            .map(|l| ListItem::new(format!("{} by {}", l.name(), l.author())))
            .collect();

        let items = List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol(">> ");

        f.render_stateful_widget(items, chunks[0], &mut list.state);

        let width = chunks[1].width;
        let text = search.get(width as usize);

        let search_block = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
        f.render_widget(search_block, chunks[1]);

        f.set_cursor(
            chunks[1].x
                + 1
                + if (width as usize - 1) < text.len() {
                    width
                } else {
                    text.len() as u16
                },
            chunks[1].y + 1,
        );

        let description = Paragraph::new(description)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false });

        f.render_widget(description, chunks[2]);
    })?;

    Ok(())
}

#[derive(Default)]
pub struct Search<'a> {
    current: String,
    store: &'a [ListCrackMe<'a>],
    matcher: SkimMatcherV2,
}

impl<'a> Search<'a> {
    pub fn new(store: &'a [ListCrackMe<'a>]) -> Search<'a> {
        Search {
            store,
            ..Default::default()
        }
    }

    pub fn search(&mut self) -> Vec<&'a ListCrackMe<'a>> {
        self.store
            .iter()
            .filter(|crackme| {
                self.matcher
                    .fuzzy_match(&crackme.to_search_string(), &self.current.trim())
                    .is_some()
            })
            .collect()
    }

    pub fn push(&mut self, c: char) {
        self.current.push(c);
    }

    pub fn pop(&mut self) {
        self.current.pop();
    }

    pub fn get(&self, length: usize) -> &str {
        // accounting for pipe characters at beginning and end, and cursor
        let length = length.checked_sub(3).unwrap_or(length);

        let start = if self.current.len() > length {
            self.current.len() - length
        } else {
            0
        };
        &self.current[start..]
    }
}
