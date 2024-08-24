use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://www.karlancer.com";

    // تعداد درخواست‌هایی که می‌خواهیم ارسال کنیم
    let request_count = 100;

    // ایجاد Futureها برای ارسال درخواست‌ها
    let futures: Vec<_> = (0..request_count).map(|_| {
        let client = client.clone(); // کلون کردن client برای استفاده در Future
        let url = url.to_string(); // کلون کردن url برای استفاده در Future
        tokio::spawn(async move {
            match client.get(&url).send().await {
                Ok(response) => {
                    println!("Status: {}", response.status());
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        })
    }).collect();

    // منتظر می‌مانیم تا همه Futureها تکمیل شوند
    for future in futures {
        future.await?;
    }

    Ok(())
}
