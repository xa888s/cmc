use anyhow::Result;
use reqwest::Client;

mod cli;
mod crackme;
mod get;
mod search;

use cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args: App = argh::from_env();
    match args.nested {
        Commands::Get(SubGet { id }) => {
            let mut client = Client::builder().cookie_store(true).build()?;
            get::handle_crackme(&mut client, &id).await?;
        }
        _ => unreachable!("Command not implemented yet!"),
    }
    Ok(())
}
