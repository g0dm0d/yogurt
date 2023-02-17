use reqwest::{Error, StatusCode};
use std::{io, time::Duration};

pub async fn get(url: &String) -> Result<io::Cursor<bytes::Bytes>, Error> {
    let mut response = reqwest::get(url).await.expect("request failed");
    loop {
        match response.status() {
            StatusCode::OK => {
                let content = io::Cursor::new(response.bytes().await.expect("body invalid"));
                return Ok(content);
            }
            status if status.is_server_error() => {
                response = reqwest::get(url).await.expect("request failed");
            }
            _ => {
                tokio::time::sleep(Duration::from_secs(5)).await;
                response = reqwest::get(url).await.expect("request failed");
            }
        }
    }
}
