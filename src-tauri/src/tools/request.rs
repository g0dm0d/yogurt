use reqwest::Error;
use std::io;

pub async fn get(url: &str) -> Result<io::Cursor<bytes::Bytes>, Error> {
    loop {
        match reqwest::get(url).await {
            Ok(resp) => {
                let content = io::Cursor::new(resp.bytes().await.expect("body invalid"));
                return Ok(content);
            }
            Err(err) => {
                eprintln!("Request error: {}", err);
                continue;
            }
        }
    }
}
