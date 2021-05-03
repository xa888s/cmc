use anyhow::Result;
use reqwest::Client;
use structopt::StructOpt;

mod cli;
mod crackme;
mod macros;
mod mode;

use cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let args: App = App::from_args();
    let mut client = Client::builder().cookie_store(true).build()?;

    match args.nested {
        Command::Get { id } => {
            mode::handle_crackme(&mut client, &id).await?;
        }
        Command::Search(args) => {
            mode::handle_search_results(&mut client, args).await?;
        }
    }
    Ok(())
}
