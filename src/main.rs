pub mod network;

use network::{another_health_check, health_check};
fn main() {
    match health_check() {
        Ok(_) => println!("Health check successful!"),
        Err(e) => eprintln!("Health check failed: {}", e),
    }

    match another_health_check() {
        Ok(_) => println!("Another Health check successful!"),
        Err(e) => eprintln!("Another Health check failed: {}", e),
    }
}
