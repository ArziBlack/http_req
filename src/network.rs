use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub status: i32,
    pub success: bool,
    pub message: String,
}

#[tokio::main]
pub async fn health_check() -> Result<Data, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://f3d-server.onrender.com/api/v1/health-check")
        .await?
        .text()
        .await?;
    println!("{response}");
    let json_res: Data = serde_json::from_str(&response)?;
    
    std::fs::write("C:\\Users\\bunak\\Development\\health_check_response.json", &response)?;
    Ok(json_res)
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

#[tokio::main]
pub async fn get_location() -> Result<geolocation::Locator, Box<dyn std::error::Error>> {
    let location = geolocation::find("105.113.64.77").unwrap();
    println!("Location Information:");
    println!("  IP: {}", location.ip);
    println!("  Latitude: {}", location.latitude);
    println!("  Longitude: {}", location.longitude);
    println!("  City: {}", location.city);
    println!("  Region: {}", location.region);
    println!("  Country: {}", location.country);
    println!("  Timezone: {}", location.timezone);
    println!("  Location: {}", location.location);
    Ok(location)
}
