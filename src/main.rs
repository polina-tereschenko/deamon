use std::{time::Duration, io, process::Command};
use thiserror::Error;
use tokio::time::sleep;
use reqwest;

#[derive(Error, Debug)]
enum DownloadError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
}

async fn download_file(url: &str, client: &reqwest::Client) -> Result<(), DownloadError> {
    println!("Downloading: {}", url);

    let mut response = client.get(url).send().await?;
    let content_length = response.content_length().unwrap_or(0);

    let file_name = format!("downloaded_file_{}", url.rsplit('/').next().unwrap_or("unknown"));

    let mut body_bytes = Vec::new();
    let mut downloaded_bytes = 0;

    while let Some(chunk) = response.chunk().await? {
        downloaded_bytes += chunk.len() as u64;
        body_bytes.extend_from_slice(&chunk);

        let progress = (downloaded_bytes as f64 / content_length as f64) * 100.0;
        println!("Progress: {:.2}%", progress);
    }

    tokio::fs::write(&file_name, body_bytes)
        .await
        .map_err(DownloadError::from)?;

    println!("Downloaded: {}", url);

    Ok(())
}

async fn restart_system() {
    let output = Command::new("sudo")
        .arg("shutdown")
        .arg("-r")
        .arg("now")
        .output();

    match output {
        Ok(_) => println!("System restart initiated."),
        Err(e) => eprintln!("Error restarting system: {:?}", e),
    }
}

async fn download_files(urls: Vec<&str>) {
    let client = reqwest::Client::new();
    
    for url in urls {
        match download_file(url, &client).await {
            Ok(_) => (),
            Err(e) => println! ("Error downloading {}: {:?}", url, e),
        }
    }
}

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://dl.zxteam.net/stratum/images/0678bc6b72501d55278c519ff5b648853052ea152c53fd1a9c0344ed9e91394c/DATA"
    ];

    let interval = Duration::from_secs(10);
    let max_downloads = 1;

    let mut download_count = 0;

    loop {
        println!("Checking for updates...");

        download_files(urls.clone()).await;

        download_count += 1;

        if download_count >= max_downloads {
            println!("Initiating system restart...");
            restart_system().await;
            break;
        }

        println!("Waiting for the next iteration...");
        sleep(interval).await;
    }
}