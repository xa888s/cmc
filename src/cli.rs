use crate::crackme::{Language, Platform};
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(name = "cmg", about = "simple crackmes.one client")]
pub struct App {
    #[structopt(subcommand)]
    pub nested: Command,
}

#[derive(StructOpt, PartialEq, Debug)]
pub enum Command {
    Get {
        #[structopt(name = "get", help = "the ID of the crackme", parse(try_from_str = id_parser))]
        id: String,
    },
    Search(SearchArgs),
}

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(
    help = "used to search for crackmes (WARNING: Only gets first page of results)",
    name = "search"
)]
pub struct SearchArgs {
    #[structopt(
        help = "the range of difficulty (i.e. 1..6)",
        parse(try_from_str = range_parser),
        short,
        long,
        default_value = "1..6"
    )]
    pub difficulty: (u8, u8),

    #[structopt(
        help = "the range of quality (i.e. 1..6)",
        parse(try_from_str = range_parser),
        short,
        long,
        default_value = "1..6"
    )]
    pub quality: (u8, u8),

    #[structopt(help = "name of crackme", short, long, default_value = "")]
    pub name: String,

    #[structopt(help = "name of crackme's author", short, long, default_value = "")]
    pub author: String,

    #[structopt(help = "language of crackme", short, long)]
    pub language: Option<Language>,

    #[structopt(help = "platform of crackme", short, long)]
    pub platform: Option<Platform>,
}

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(help = "used to get crackmes", name = "get")]
pub struct SubGet {
    #[structopt(
        help = "the ID of the crackme",
        parse(try_from_str = id_parser)
    )]
    pub id: String,
}

fn range_parser(v: &str) -> Result<(u8, u8), &'static str> {
    let mut sides = v.split("..");
    let (first, second): (u8, u8) = (
        sides
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or("Invalid beginning bound")?,
        sides
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or("Invalid ending bound")?,
    );

    if sides.next().is_some() || first < 1 || second > 6 {
        return Err("Invalid range");
    }

    Ok((first, second))
}

fn id_parser(v: &str) -> Result<String, &'static str> {
    let v = v.to_string();
    if v.len() != 24 {
        Err("Invalid ID length")
    } else {
        Ok(v)
    }
}
