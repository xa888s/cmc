use anyhow::{anyhow, Result};
use crackmes::{overview::OverviewCrackme, Html};
use reqwest::Client;
use std::{fs, io::Cursor, path::Path};
use zip::read::ZipArchive;

const MAIN_URL: &str = "https://crackmes.one";
const GET_URL: &str = "https://crackmes.one/crackme/";

fn write_zip_to_disk(bytes: Vec<u8>, crackme: &OverviewCrackme<'_>) -> Result<()> {
    // wrap our bytes with a cursor for the seek implementation
    let mut zip = ZipArchive::new(Cursor::new(bytes))?;

    // writing the zip file's contents to disk, copied from the zip crates extract method on
    // ZipArchive
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
            .ok_or_else(|| anyhow!("Invalid file path"))?;

        let outpath = Path::new(crackme.name()).join(filepath);

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

pub async fn get_description(client: &mut Client, id: &str) -> Result<String> {
    // downloads crackme page
    let html = {
        let body = client
            .get(GET_URL.to_string() + id)
            .send()
            .await?
            .text()
            .await?;

        Html::parse_document(&body)
    };

    // this unwrap is safe because OverviewCrackMe is guaranteed to have a description
    Ok(OverviewCrackme::with_full_html(&html, id)?
        .description()
        .to_string())
}

pub async fn handle_crackme(client: &mut Client, id: &str) -> Result<()> {
    // downloads crackme page
    let html = {
        let body = client
            .get(GET_URL.to_string() + id)
            .send()
            .await?
            .text()
            .await?;

        Html::parse_document(&body)
    };

    let crackme = OverviewCrackme::with_full_html(&html, id)?;

    // getting the zip file
    let bytes = client
        .get(MAIN_URL.to_string() + "/static/crackme/" + crackme.id() + ".zip")
        .send()
        .await?
        .bytes()
        .await?
        .to_vec();

    // writing the files contained inside it to disk (in a new folder in the current directory with
    // its name being the id)
    write_zip_to_disk(bytes, &crackme)?;
    println!("{}", crackme);

    Ok(())
}
