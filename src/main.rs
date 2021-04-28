use anyhow::Result;
use argh::FromArgs;
use reqwest::Client;
mod get;
mod search;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "simple crackmes.one client")]
struct App {
    #[argh(subcommand)]
    nested: Commands,
}

#[non_exhaustive]
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Commands {
    Get(SubGet),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "used to get crackmes", subcommand, name = "get")]
struct SubGet {
    #[argh(
        description = "the ID of the crackme",
        from_str_fn(id_parser),
        positional
    )]
    id: String,
}

fn id_parser(v: &str) -> Result<String, String> {
    let v = v.to_string();
    if v.len() != 24 {
        Err(v)
    } else {
        Ok(v)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: App = argh::from_env();
    match args.nested {
        Commands::Get(SubGet { id }) => {
            let mut client = Client::builder().cookie_store(true).build()?;
            get::get_crackme(&mut client, &id).await?;
        }
        _ => unreachable!("Command not implemented yet!"),
    }
    Ok(())
}
