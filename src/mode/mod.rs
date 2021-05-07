use crate::crackme::{
    list::{ListCrackMe, ListItem},
    CrackMe,
};

pub mod get;
pub mod latest;
pub mod search;

// TODO: Optimize this
pub fn get_choice(input: &[CrackMe<'_, ListCrackMe>]) -> Option<String> {
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
