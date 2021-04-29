use crate::crackme::{Language, Platform};
use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "simple crackmes.one client")]
pub struct App {
    #[argh(subcommand)]
    pub nested: Commands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Commands {
    Get(SubGet),
    Search(SearchArgs),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    description = "used to search for crackmes (WARNING: Only gets first page of results)",
    subcommand,
    name = "search"
)]
pub struct SearchArgs {
    #[argh(
        description = "the range of difficulty (i.e. 1..6)",
        from_str_fn(range_parser),
        option,
        default = "(1, 6)"
    )]
    pub difficulty: (u8, u8),

    #[argh(
        description = "the range of quality (i.e. 1..6)",
        from_str_fn(range_parser),
        option,
        default = "(1, 6)"
    )]
    pub quality: (u8, u8),

    #[argh(description = "name of crackme", option, default = "")]
    pub name: String,

    #[argh(description = "name of crackme's author", option, default = "")]
    pub author: String,

    #[argh(description = "language of crackme", option)]
    pub language: Option<Language>,

    #[argh(description = "platform of crackme", option)]
    pub platform: Option<Platform>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "used to get crackmes", subcommand, name = "get")]
pub struct SubGet {
    #[argh(
        description = "the ID of the crackme",
        from_str_fn(id_parser),
        positional
    )]
    pub id: String,
}

fn range_parser(v: &str) -> Result<(u8, u8), String> {
    let mut sides = v.split("..");
    let (first, second): (u8, u8) = (
        sides
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| "Invalid beginning bound".to_string())?,
        sides
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| "Invalid ending bound".to_string())?,
    );

    if sides.next().is_some() || first < 1 || second > 6 {
        return Err("Invalid range".to_string());
    }

    Ok((first, second))
}

fn id_parser(v: &str) -> Result<String, String> {
    let v = v.to_string();
    if v.len() != 24 {
        Err("Invalid ID length".to_string())
    } else {
        Ok(v)
    }
}
