use anyhow::Result;
use std::io;

use super::search::{SearchText, Searcher};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, List, Paragraph, Wrap};
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
    search_text: &SearchText,
    searcher: &mut Searcher<'a>,
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

        let items: List = searcher.list();

        f.render_stateful_widget(items, chunks[0], searcher.state());

        let width = chunks[1].width;
        let text = search_text.get(width as usize);

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

        let description = Paragraph::new(
            searcher
                .selected()
                .map(|crackme| crackme.to_string())
                .unwrap_or_default(),
        )
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: false });

        f.render_widget(description, chunks[2]);
    })?;

    Ok(())
}
