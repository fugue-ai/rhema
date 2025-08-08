use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Testing MCP Daemon functionality...");

    // Start the daemon in background
    let mut daemon_process = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "rhema",
            "--",
            "daemon",
            "start",
            "--foreground",
            "--port",
            "8081",
        ])
        .spawn()
        .expect("Failed to start daemon");

    println!("Daemon started with PID: {}", daemon_process.id());

    // Wait for daemon to start
    thread::sleep(Duration::from_secs(5));

    // Test health endpoint using curl
    let output = Command::new("curl")
        .args(&["-s", "http://127.0.0.1:8081/health"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Health endpoint responded successfully");
                println!("Response: {}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("❌ Health endpoint failed with status: {}", output.status);
                println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Failed to test health endpoint: {}", e);
        }
    }

    // Test stats endpoint
    let output = Command::new("curl")
        .args(&["-s", "http://127.0.0.1:8081/stats"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Stats endpoint responded successfully");
                println!("Response: {}", String::from_utf8_lossy(&output.stdout));
            } else {
                println!("❌ Stats endpoint failed with status: {}", output.status);
                println!("Error: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Failed to test stats endpoint: {}", e);
        }
    }

    // Stop the daemon
    println!("Stopping daemon...");
    let _ = daemon_process.kill();
    let _ = daemon_process.wait();
    println!("Daemon stopped");
}
