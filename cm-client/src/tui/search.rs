use anyhow::Result;
use crackmes::list::ListCrackme;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tui::widgets::{Block, Borders, List, ListItem, ListState};

#[derive(Default, Debug)]
pub struct SearchText(String);

impl SearchText {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn push(&mut self, c: char) {
        self.0.push(c);
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn get(&self, length: usize) -> &str {
        // accounting for pipe characters at beginning and end, and cursor
        let length = length.checked_sub(3).unwrap_or(length);

        let start = if self.0.len() > length {
            self.0.len() - length
        } else {
            0
        };
        &self.0[start..]
    }
}

#[derive(Default)]
pub struct Searcher<'crackme> {
    store: &'crackme mut [ListCrackme<'crackme>],
    found: Vec<usize>,
    state: ListState,
    matcher: SkimMatcherV2,
}

use reqwest::Client;
impl<'a> Searcher<'a> {
    pub fn new(store: &'a mut [ListCrackme<'a>]) -> Searcher<'a> {
        let mut searcher = Searcher {
            found: (0..store.len()).collect(),
            store,
            ..Default::default()
        };
        searcher.last();

        searcher
    }

    pub async fn fetch_descriptions(&mut self, client: &mut Client) -> Result<()> {
        // we start at the end of the vector, so we only have to download the current and next
        // crackme's description (technically only current but we fetch the next one to lighten the
        // load later), unless we skip to the top (through the gg bind), then we also need to
        // download below us
        if let Some(nearby) = self.found.len().checked_sub(1).and_then(|last| {
            self.state.selected().map(|i| {
                let start = i.saturating_sub(1);

                // "saturating" add on the len
                let end = if i + 1 > last { last } else { i + 1 };

                start..=end
            })
        }) {
            for i in nearby {
                let crackme = &mut self.store[i];
                if crackme.description().is_none() {
                    crackme
                        .try_set_description(
                            crate::get::get_description(client, crackme.id()).await?,
                        )
                        .unwrap();
                }
            }
        }
        Ok(())
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn search(&mut self, query: &str) {
        let items = self
            .store
            .iter()
            .enumerate()
            .filter(|(_, crackme)| {
                self.matcher
                    .fuzzy_match(&crackme.to_search_string(), query.trim())
                    .is_some()
            })
            .map(|(index, _)| index)
            .collect();

        self.found = items;
        self.last();
    }

    pub fn list(&self) -> List<'static> {
        let items: Vec<ListItem> = self
            .found
            .iter()
            .flat_map(|&i| self.store.get(i))
            .map(|l| ListItem::new(format!("{} by {}", l.name(), l.author())))
            .collect();

        List::new(items)
            .block(Block::default().borders(Borders::ALL))
            .highlight_symbol(">> ")
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.found.len() - 1 {
                    self.found.len() - 1
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<&ListCrackme<'_>> {
        self.state
            .selected()
            .and_then(|i| self.found.get(i).and_then(|&i| self.store.get(i)))
    }

    pub fn last(&mut self) {
        if !self.found.is_empty() {
            self.state.select(None);
            self.state.select(Some(self.found.len() - 1));
        }
    }

    pub fn into_selected(self) -> Option<&'a ListCrackme<'a>> {
        self.state
            .selected()
            .and_then(move |i| self.store.get(self.found[i]))
    }
}
