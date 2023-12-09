use std::io;
use std::time::Duration;
use tokio::time::sleep;

pub async fn get(url: &str) -> Result<io::Cursor<bytes::Bytes>, String> {
    const MAX_ATTEMPTS: u32 = 5;
    const RETRY_INTERVAL: u64 = 1;

    for attempt in 1..=MAX_ATTEMPTS {
        match reqwest::get(url).await {
            Ok(resp) => {
                let content = io::Cursor::new(resp.bytes().await.map_err(|err| err.to_string())?);
                return Ok(content);
            }
            Err(err) => {
                eprintln!("Attempt {}/{} - Request error: {}", attempt, MAX_ATTEMPTS, err);

                if attempt < MAX_ATTEMPTS {
                    sleep(Duration::from_secs(RETRY_INTERVAL)).await;
                }
            }
        }
    }

    Err("Download max attempts reached".to_owned())
}
