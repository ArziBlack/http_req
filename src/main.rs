pub mod network;

use network::{another_health_check, download_batch_export_forms_zip, download_file, health_check};

fn main() {
    match health_check() {
        Ok(data) => {
            println!("Health check successful!");
            println!("Status: {}, Success: {}, Message: {}", data.status, data.success, data.message);
        },
        Err(e) => eprintln!("Health check failed: {}", e),
    }

    match another_health_check() {
        Ok(_) => println!("Another Health check successful!"),
        Err(e) => eprintln!("Another Health check failed: {}", e),
    }

    match download_file("https://f3d-server.onrender.com/api/v1/spills/export?client_id=680624347a2e786232986db6", "spills.xlsx") {
        Ok(_) => println!("File downloaded successfully!"),
        Err(e) => eprintln!("Failed to download file: {}", e),
    }

    match download_batch_export_forms_zip() {
        Ok(_) => println!("File downloaded successfully!"),
        Err(e) => eprintln!("Failed to download file: {}", e),
    }

    match network::get_location() {
        Ok(location) => {
            println!("Location retrieved successfully!");
            println!("Location: {:#?}", location.timezone);
        },
        Err(e) => eprintln!("Failed to retrieve location: {}", e),
    }
}
