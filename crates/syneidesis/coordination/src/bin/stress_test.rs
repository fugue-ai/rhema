/*
 * Copyright 2025 Cory Parent
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use clap::Parser;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Barrier;
use tokio::sync::Mutex;
use tokio::time::sleep;
use tracing::{error, info, warn};

use syneidesis_grpc::CoordinationClient;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of concurrent agents to simulate
    #[arg(short, long, default_value_t = 100)]
    agents: usize,

    /// Duration of the stress test
    #[arg(short, long, default_value = "600s")]
    duration: String,

    /// Message rate per second per agent
    #[arg(short, long, default_value_t = 10)]
    message_rate: u32,

    /// Ramp-up time in seconds
    #[arg(short, long, default_value_t = 60)]
    ramp_up: u64,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    info!("Starting stress test with {} agents", args.agents);

    // Parse duration
    let duration = parse_duration(&args.duration)?;
    let ramp_up_duration = Duration::from_secs(args.ramp_up);

    // Create gRPC client
    let client = Arc::new(Mutex::new(CoordinationClient::new_default().await?));

    // Create barrier for synchronized start
    let barrier = Arc::new(Barrier::new(args.agents + 1)); // +1 for main thread

    // Spawn agents
    let mut handles = Vec::new();
    let start_time = Instant::now();

    for i in 0..args.agents {
        let client = client.clone();
        let barrier = barrier.clone();
        let agent_id = format!("stress_agent_{i}");
        let message_rate = args.message_rate;

        let handle = tokio::spawn(async move {
            // Wait for all agents to be ready
            barrier.wait().await;

            // Ramp-up period
            sleep(ramp_up_duration).await;

            // Register agent via gRPC
            let agent_info = syneidesis_grpc::AgentInfo {
                id: agent_id.clone(),
                name: format!("Stress Agent {i}"),
                agent_type: "stress_test".to_string(),
                status: syneidesis_grpc::AgentStatus::Idle as i32,
                health: syneidesis_grpc::AgentHealth::Healthy as i32,
                current_task_id: None,
                assigned_scope: "stress_test".to_string(),
                capabilities: vec!["stress_test".to_string()],
                last_heartbeat: None,
                is_online: true,
                performance_metrics: None,
                priority: 1,
                version: "1.0.0".to_string(),
                endpoint: None,
                metadata: std::collections::HashMap::new(),
                created_at: None,
                last_updated: None,
            };

            match client.lock().await.register_agent(agent_info).await {
                Ok(_) => info!("Agent {} registered", agent_id),
                Err(e) => {
                    warn!("Agent {} failed to register: {}", agent_id, e);
                    return Ok::<(), Box<dyn std::error::Error + Send + Sync>>(());
                }
            }

            let mut interval =
                tokio::time::interval(Duration::from_secs_f32(1.0 / message_rate as f32));
            let mut message_count = 0;

            loop {
                interval.tick().await;

                // Send a test message via gRPC
                let message = syneidesis_grpc::AgentMessage {
                    id: format!("msg_{agent_id}_{message_count}"),
                    message_type: syneidesis_grpc::MessageType::Custom as i32,
                    priority: syneidesis_grpc::MessagePriority::Normal as i32,
                    sender_id: agent_id.clone(),
                    recipient_ids: vec![format!("stress_agent_{}", (i + 1) % args.agents)],
                    content: format!("Stress test message #{message_count}"),
                    payload: None,
                    timestamp: Some(prost_types::Timestamp::from(std::time::SystemTime::now())),
                    requires_ack: false,
                    expires_at: None,
                    metadata: std::collections::HashMap::new(),
                };

                match client.lock().await.send_message(message).await {
                    Ok(_) => {
                        message_count += 1;
                        if message_count % 100 == 0 {
                            info!("Agent {} sent {} messages", agent_id, message_count);
                        }
                    }
                    Err(e) => {
                        warn!("Agent {} failed to send message: {}", agent_id, e);
                    }
                }

                // Check if we should stop
                if start_time.elapsed() >= duration {
                    break;
                }
            }

            info!(
                "Agent {} completed, sent {} messages",
                agent_id, message_count
            );
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        });

        handles.push(handle);
    }

    // Wait for all agents to be ready
    barrier.wait().await;
    info!("All agents ready, starting stress test...");

    // Wait for all agents to complete
    let _total_messages = 0;
    let mut failed_agents = 0;

    for handle in handles {
        match handle.await {
            Ok(Ok(())) => {
                info!("Agent completed successfully");
            }
            Ok(Err(e)) => {
                error!("Agent failed: {}", e);
                failed_agents += 1;
            }
            Err(e) => {
                error!("Agent task failed: {}", e);
                failed_agents += 1;
            }
        }
    }

    let elapsed = start_time.elapsed();
    let _expected_messages = args.agents as u32 * args.message_rate * elapsed.as_secs() as u32;

    info!("Stress test completed!");
    info!("Duration: {:?}", elapsed);
    info!("Agents: {}", args.agents);
    info!("Failed agents: {}", failed_agents);
    info!(
        "Success rate: {:.2}%",
        ((args.agents - failed_agents) as f64 / args.agents as f64) * 100.0
    );

    if failed_agents > 0 {
        warn!("Some agents failed during stress test");
        std::process::exit(1);
    }

    Ok(())
}

fn parse_duration(duration_str: &str) -> Result<Duration, Box<dyn std::error::Error>> {
    let duration_str = duration_str.to_lowercase();

    if duration_str.ends_with('s') {
        let seconds: u64 = duration_str[..duration_str.len() - 1].parse()?;
        Ok(Duration::from_secs(seconds))
    } else if duration_str.ends_with('m') {
        let minutes: u64 = duration_str[..duration_str.len() - 1].parse()?;
        Ok(Duration::from_secs(minutes * 60))
    } else if duration_str.ends_with('h') {
        let hours: u64 = duration_str[..duration_str.len() - 1].parse()?;
        Ok(Duration::from_secs(hours * 3600))
    } else {
        // Assume seconds if no unit specified
        let seconds: u64 = duration_str.parse()?;
        Ok(Duration::from_secs(seconds))
    }
}
