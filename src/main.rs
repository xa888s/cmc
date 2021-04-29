use anyhow::Result;
use reqwest::Client;

mod cli;
mod crackme;
mod get;
mod macros;
mod search;

use cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args: App = argh::from_env();
    let mut client = Client::builder().cookie_store(true).build()?;

    match args.nested {
        Commands::Get(SubGet { id }) => {
            get::handle_crackme(&mut client, &id).await?;
        }
        Commands::Search(args) => {
            search::handle_search_results(&mut client, args).await?;
        }
    }
    Ok(())
}
