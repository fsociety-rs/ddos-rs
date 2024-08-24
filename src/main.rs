use reqwest::Client;
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;

    let url = "https://example.com";
    let request_count = 5000;
    let concurrency_limit = 100;

    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    let mut handles = vec![];

    for _ in 0..request_count {
        let semaphore = semaphore.clone();
        let client = client.clone();
        let url = url.to_string();

        let handle = tokio::spawn(async move {
            // Acquire a permit from the semaphore
            let _permit = match semaphore.acquire().await {
                Ok(permit) => permit,
                Err(_) => {
                    eprintln!("Failed to acquire semaphore permit");
                    return;
                }
            };

            // Use the permit to limit concurrent requests
            match client.get(&url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        println!("Request successful, Status: {}", response.status());
                    } else {
                        println!("Request failed, Status: {}", response.status());
                    }
                }
                Err(err) => {
                    eprintln!("Request error: {}", err);
                }
            }

            // `_permit` is automatically released when it goes out of scope
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
