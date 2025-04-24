use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    status: i32,
    success: bool,
    message: String,
}

#[tokio::main]
pub async fn health_check() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://f3d-server.onrender.com/api/v1/health-check")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{response:#?}");
    Ok(())
}

#[tokio::main]
pub async fn another_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let body: String = reqwest::get("https://f3d-server.onrender.com/api/v1/health-check")
        .await?
        .text()
        .await?;
    println!("body = {body:?}");
    let parsed = json::parse(&body).unwrap();
    println!("parsed = {parsed:#?}");
    let json_res: Data = serde_json::from_str(&body)?;
    println!("json = {json_res:#?}");
    println!(
        "Please the status is {} and success is {} and the message from the server is {}",
        json_res.status, json_res.success, json_res.message
    );
    Ok(())
}

/// Downloads a file from the specified URL and saves it to the system root (C: drive)
/// 
/// # Arguments
/// 
/// * `url` - The URL of the file to download
/// * `filename` - The name to save the file as on the C: drive
/// 
/// # Returns
/// 
/// * `Result<(), Box<dyn std::error::Error>>` - Ok if successful, Err if the download or save operation fails
#[tokio::main]
pub async fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        return Err(format!("Failed to download file: HTTP {}", response.status()).into());
    }
    
    let mut file = std::fs::File::create(format!("C:\\{}", filename))?;
    let content = response.bytes().await?;
    std::io::copy(&mut content.as_ref(), &mut file)?;
    
    Ok(())
}
