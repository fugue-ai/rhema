use std::time::Duration;
use tokio::time::sleep;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing MCP Daemon functionality...");
    
    // Test health endpoint
    let client = reqwest::Client::new();
    
    // Wait a bit for daemon to start
    sleep(Duration::from_secs(2)).await;
    
    // Test health endpoint
    match client.get("http://127.0.0.1:8081/health").timeout(Duration::from_secs(5)).send().await {
        Ok(response) => {
            println!("✅ Health endpoint responded with status: {}", response.status());
            if let Ok(body) = response.text().await {
                println!("Response body: {}", body);
            }
        }
        Err(e) => {
            println!("❌ Health endpoint failed: {}", e);
        }
    }
    
    // Test stats endpoint
    match client.get("http://127.0.0.1:8081/stats").timeout(Duration::from_secs(5)).send().await {
        Ok(response) => {
            println!("✅ Stats endpoint responded with status: {}", response.status());
            if let Ok(body) = response.text().await {
                println!("Response body: {}", body);
            }
        }
        Err(e) => {
            println!("❌ Stats endpoint failed: {}", e);
        }
    }
    
    Ok(())
} 