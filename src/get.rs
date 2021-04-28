use anyhow::{format_err, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::path::Path;
use zip::read::ZipArchive;

const MAIN_URL: &str = "https://crackmes.one";
const GET_URL: &str = "https://crackmes.one/crackme/";

async fn get_download_link(client: &mut Client, id: &str) -> Result<String> {
    // getting download url
    let html = {
        let body = client
            .get(GET_URL.to_string() + id)
            .send()
            .await?
            .text()
            .await?;

        Html::parse_document(&body)
    };

    // guaranteed to parse
    let selector = Selector::parse("a").unwrap();

    // finding the download link
    let element = html
        .select(&selector)
        .find(|e| e.value().classes().any(|c| c == "btn-download"))
        .ok_or_else(|| format_err!("No element with btn-download"))?;

    Ok(element
        .value()
        .attr("href")
        .ok_or_else(|| format_err!("No href value"))?
        .to_string())
}

async fn write_zip_to_disk(bytes: Vec<u8>, id: &str) -> Result<()> {
    // wrap our bytes with a cursor for the seek implementation
    let mut zip = ZipArchive::new(std::io::Cursor::new(bytes))?;

    // writing the zip file's contents to disk, copied from the zip crates extract method on
    // ZipArchive
    use std::fs;
    for i in 0..zip.len() {
        let first = zip.by_index_decrypt(i, b"crackmes.one")?;
        let mut file = match first {
            Ok(f) => f,
            Err(_) => {
                drop(first);
                match zip.by_index_decrypt(i, b"crackmes.de")? {
                    Ok(f) => f,
                    Err(_) => continue,
                }
            }
        };

        let filepath = file
            .enclosed_name()
            .ok_or_else(|| format_err!("Invalid file path"))?;

        let outpath = Path::new(id).join(filepath);

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

pub async fn get_crackme(client: &mut Client, id: &str) -> Result<()> {
    // downloads and parses the crackme page to get the download link
    let href = get_download_link(client, id).await?;

    // getting the zip file
    let bytes = client
        .get(MAIN_URL.to_string() + &href)
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();

    // writing the files contained inside it to disk (in a new folder in the current directory with
    // its name being the id)
    write_zip_to_disk(bytes, id).await?;

    Ok(())
}
