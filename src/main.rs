use anyhow::Result;
use reqwest::Client;
mod parser;

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = Client::builder().cookie_store(true).build()?;

    let token = parser::get_token(&mut client).await?;

    dbg!(token);
    Ok(())
}
