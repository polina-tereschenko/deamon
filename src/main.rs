use std::{thread, time::Duration};
use tokio::time::sleep;
use reqwest::Client;

async fn download_file(url: &str, client: &Client) -> Result<(), reqwest::Error> {
    let response = client.get(url).send().await?;
    
    let file_name = format!("downloaded_file_{}", url.rsplit('/').next().unwrap_or("unknown"));
    
    tokio::fs::write(&file_name, response.bytes().await?).await?;
    
    println!("Downloaded: {}", url);
    
    Ok(())
}

async fn download_files(urls: Vec<&str>) {
    let client = reqwest::Client::new();
    
    for url in urls {
        match download_file(url, &client).await {
            Ok(_) => (),
            Err(e) => eprintln!("Error downloading {}: {:?}", url, e),
        }
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://dl.zxteam.net/stratum/images/0678bc6b72501d55278c519ff5b648853052ea152c53fd1a9c0344ed9e91394c/DATA"
    ];

    let interval = Duration::from_secs(10);

    loop {
        download_files(urls.clone()).await;
        sleep(interval).await;
    }
}