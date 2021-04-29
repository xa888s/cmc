use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "simple crackmes.one client")]
pub struct App {
    #[argh(subcommand)]
    pub nested: Commands,
}

#[non_exhaustive]
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Commands {
    Get(SubGet),
    Search(SubSearch),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(
    description = "used to search for crackmes",
    subcommand,
    name = "search"
)]
pub struct SubSearch {}

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

fn id_parser(v: &str) -> Result<String, String> {
    let v = v.to_string();
    if v.len() != 24 {
        Err("Invalid ID length".to_string())
    } else {
        Ok(v)
    }
}
