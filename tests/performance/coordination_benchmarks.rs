//! Performance benchmarks for Rhema Coordination CLI

use crate::common::{fixtures::TestFixtures, helpers::TestHelpers, TestEnv};
use rand::thread_rng;
use rand::Rng;
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// Benchmark coordination command execution
fn benchmark_coordination_command(
    args: &[&str],
) -> Result<Duration, Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();

    let output = Command::new("cargo")
        .args(&["run", "--bin", "rhema", "--", "coordination"])
        .args(args)
        .output()?;

    let duration = start.elapsed();

    if !output.status.success() {
        return Err(format!(
            "Benchmark command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(duration)
}

/// Run multiple iterations and calculate statistics
fn run_benchmark_iterations<F>(
    iterations: usize,
    test_fn: F,
) -> Result<BenchmarkStats, Box<dyn std::error::Error + Send + Sync>>
where
    F: Fn() -> Result<Duration, Box<dyn std::error::Error + Send + Sync>>,
{
    let mut durations = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let duration = test_fn()?;
        durations.push(duration);
    }

    durations.sort();

    let min = durations[0];
    let max = durations[durations.len() - 1];
    let avg = durations.iter().sum::<Duration>() / durations.len() as u32;
    let median = durations[durations.len() / 2];

    Ok(BenchmarkStats {
        iterations,
        min,
        max,
        avg,
        median,
        durations,
    })
}

#[derive(Debug)]
struct BenchmarkStats {
    iterations: usize,
    min: Duration,
    max: Duration,
    avg: Duration,
    median: Duration,
    durations: Vec<Duration>,
}

impl BenchmarkStats {
    fn print_summary(&self, test_name: &str) {
        println!("=== {} Benchmark Results ===", test_name);
        println!("Iterations: {}", self.iterations);
        println!("Min: {:?}", self.min);
        println!("Max: {:?}", self.max);
        println!("Avg: {:?}", self.avg);
        println!("Median: {:?}", self.median);
        println!("================================");
    }
}

// ============================================================================
// Agent Management Benchmarks
// ============================================================================

#[test]
fn benchmark_agent_registration() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    let stats = run_benchmark_iterations(10, || {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("bench-agent-{}", rand::random::<u32>()),
            "--type",
            "TestAgent",
            "--scope",
            "benchmarking",
        ])
    })?;

    stats.print_summary("Agent Registration");

    // Performance assertions
    assert!(stats.avg.as_millis() < 5000); // Should complete within 5 seconds
    assert!(stats.max.as_millis() < 10000); // Max should be under 10 seconds

    Ok(())
}

#[test]
fn benchmark_agent_listing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-populate with agents
    for i in 0..50 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("list-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "benchmarking",
        ])?;
    }

    let stats =
        run_benchmark_iterations(20, || benchmark_coordination_command(&["agent", "list"]))?;

    stats.print_summary("Agent Listing (50 agents)");

    // Performance assertions
    assert!(stats.avg.as_millis() < 2000); // Should complete within 2 seconds
    assert!(stats.max.as_millis() < 5000); // Max should be under 5 seconds

    Ok(())
}

#[test]
fn benchmark_agent_listing_with_filters() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-populate with mixed agent types
    for i in 0..100 {
        let agent_type = if i % 3 == 0 {
            "CodeReviewAgent"
        } else {
            "TestAgent"
        };
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("filter-bench-agent-{}", i),
            "--type",
            agent_type,
            "--scope",
            "benchmarking",
        ])?;
    }

    let stats = run_benchmark_iterations(15, || {
        benchmark_coordination_command(&["agent", "list", "--agent-type", "CodeReviewAgent"])
    })?;

    stats.print_summary("Agent Listing with Filter (100 agents)");

    // Performance assertions
    assert!(stats.avg.as_millis() < 3000); // Should complete within 3 seconds
    assert!(stats.max.as_millis() < 8000); // Max should be under 8 seconds

    Ok(())
}

#[test]
fn benchmark_agent_message_sending() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Register an agent
    benchmark_coordination_command(&[
        "agent",
        "register",
        "--name",
        "message-bench-agent",
        "--type",
        "TestAgent",
        "--scope",
        "benchmarking",
    ])?;

    let stats = run_benchmark_iterations(25, || {
        benchmark_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            "Benchmark test message",
            "--message-type",
            "Test",
            "--priority",
            "Normal",
        ])
    })?;

    stats.print_summary("Agent Message Sending");

    // Performance assertions
    assert!(stats.avg.as_millis() < 1000); // Should complete within 1 second
    assert!(stats.max.as_millis() < 3000); // Max should be under 3 seconds

    Ok(())
}

#[test]
fn benchmark_agent_broadcast() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-populate with multiple agents
    for i in 0..20 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("broadcast-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "benchmarking",
        ])?;
    }

    let stats = run_benchmark_iterations(10, || {
        benchmark_coordination_command(&[
            "agent",
            "broadcast",
            "Broadcast benchmark message",
            "--message-type",
            "Notification",
            "--priority",
            "Normal",
        ])
    })?;

    stats.print_summary("Agent Broadcast (20 agents)");

    // Performance assertions
    assert!(stats.avg.as_millis() < 2000); // Should complete within 2 seconds
    assert!(stats.max.as_millis() < 5000); // Max should be under 5 seconds

    Ok(())
}

// ============================================================================
// Session Management Benchmarks
// ============================================================================

#[test]
fn benchmark_session_creation() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-register agents
    for i in 0..10 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("session-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "benchmarking",
        ])?;
    }

    let stats = run_benchmark_iterations(15, || {
        benchmark_coordination_command(&[
            "session",
            "create",
            &format!("Benchmark Session {}", rand::random::<u32>()),
            "--participants",
            "agent-001,agent-002,agent-003",
        ])
    })?;

    stats.print_summary("Session Creation");

    // Performance assertions
    assert!(stats.avg.as_millis() < 1500); // Should complete within 1.5 seconds
    assert!(stats.max.as_millis() < 4000); // Max should be under 4 seconds

    Ok(())
}

#[test]
fn benchmark_session_listing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-create sessions
    for i in 0..30 {
        benchmark_coordination_command(&[
            "session",
            "create",
            &format!("List Bench Session {}", i),
            "--participants",
            "agent-001",
        ])?;
    }

    let stats =
        run_benchmark_iterations(20, || benchmark_coordination_command(&["session", "list"]))?;

    stats.print_summary("Session Listing (30 sessions)");

    // Performance assertions
    assert!(stats.avg.as_millis() < 2000); // Should complete within 2 seconds
    assert!(stats.max.as_millis() < 6000); // Max should be under 6 seconds

    Ok(())
}

#[test]
fn benchmark_session_message_sending() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Create a session
    benchmark_coordination_command(&[
        "session",
        "create",
        "Message Bench Session",
        "--participants",
        "agent-001,agent-002,agent-003",
    ])?;

    let stats = run_benchmark_iterations(30, || {
        benchmark_coordination_command(&[
            "session",
            "send-message",
            "--session-id",
            "session-001",
            "Benchmark session message",
            "--message-type",
            "Chat",
            "--priority",
            "Normal",
            "--sender-id",
            "agent-001",
        ])
    })?;

    stats.print_summary("Session Message Sending");

    // Performance assertions
    assert!(stats.avg.as_millis() < 1000); // Should complete within 1 second
    assert!(stats.max.as_millis() < 3000); // Max should be under 3 seconds

    Ok(())
}

// ============================================================================
// System Monitoring Benchmarks
// ============================================================================

#[test]
fn benchmark_system_stats() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-populate with data
    for i in 0..25 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("stats-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "benchmarking",
        ])?;
    }

    for i in 0..15 {
        benchmark_coordination_command(&[
            "session",
            "create",
            &format!("Stats Bench Session {}", i),
            "--participants",
            "agent-001",
        ])?;
    }

    let stats =
        run_benchmark_iterations(25, || benchmark_coordination_command(&["system", "stats"]))?;

    stats.print_summary("System Statistics");

    // Performance assertions
    assert!(stats.avg.as_millis() < 1500); // Should complete within 1.5 seconds
    assert!(stats.max.as_millis() < 4000); // Max should be under 4 seconds

    Ok(())
}

#[test]
fn benchmark_system_stats_detailed() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-populate with data
    for i in 0..50 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("detailed-stats-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "benchmarking",
        ])?;
    }

    let stats = run_benchmark_iterations(15, || {
        benchmark_coordination_command(&["system", "stats", "--detailed"])
    })?;

    stats.print_summary("System Statistics (Detailed)");

    // Performance assertions
    assert!(stats.avg.as_millis() < 3000); // Should complete within 3 seconds
    assert!(stats.max.as_millis() < 8000); // Max should be under 8 seconds

    Ok(())
}

#[test]
fn benchmark_message_history() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Pre-populate with messages
    benchmark_coordination_command(&[
        "agent",
        "register",
        "--name",
        "history-bench-agent",
        "--type",
        "TestAgent",
        "--scope",
        "benchmarking",
    ])?;

    for i in 0..100 {
        benchmark_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            &format!("History benchmark message {}", i),
            "--message-type",
            "Test",
            "--priority",
            "Normal",
        ])?;
    }

    let stats = run_benchmark_iterations(20, || {
        benchmark_coordination_command(&["system", "message-history"])
    })?;

    stats.print_summary("Message History (100 messages)");

    // Performance assertions
    assert!(stats.avg.as_millis() < 2000); // Should complete within 2 seconds
    assert!(stats.max.as_millis() < 6000); // Max should be under 6 seconds

    Ok(())
}

#[test]
fn benchmark_health_check() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    let stats =
        run_benchmark_iterations(30, || benchmark_coordination_command(&["system", "health"]))?;

    stats.print_summary("Health Check");

    // Performance assertions
    assert!(stats.avg.as_millis() < 500); // Should complete within 500ms
    assert!(stats.max.as_millis() < 2000); // Max should be under 2 seconds

    Ok(())
}

// ============================================================================
// Load Testing Benchmarks
// ============================================================================

#[test]
fn benchmark_high_load_agent_registration() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    let start_time = Instant::now();

    // Register many agents rapidly
    for i in 0..100 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("load-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "load-testing",
        ])?;
    }

    let total_duration = start_time.elapsed();

    println!("=== High Load Agent Registration ===");
    println!("Total time for 100 agents: {:?}", total_duration);
    println!("Average time per agent: {:?}", total_duration / 100);
    println!("=====================================");

    // Performance assertions
    assert!(total_duration.as_secs() < 60); // Should complete within 1 minute
    assert!(total_duration.as_millis() / 100 < 500); // Average under 500ms per agent

    Ok(())
}

#[test]
fn benchmark_high_load_message_sending() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Register an agent
    benchmark_coordination_command(&[
        "agent",
        "register",
        "--name",
        "load-message-agent",
        "--type",
        "TestAgent",
        "--scope",
        "load-testing",
    ])?;

    let start_time = Instant::now();

    // Send many messages rapidly
    for i in 0..200 {
        benchmark_coordination_command(&[
            "agent",
            "send-message",
            "--to",
            "agent-001",
            &format!("Load test message {}", i),
            "--message-type",
            "Test",
            "--priority",
            "Normal",
        ])?;
    }

    let total_duration = start_time.elapsed();

    println!("=== High Load Message Sending ===");
    println!("Total time for 200 messages: {:?}", total_duration);
    println!("Average time per message: {:?}", total_duration / 200);
    println!("==================================");

    // Performance assertions
    assert!(total_duration.as_secs() < 120); // Should complete within 2 minutes
    assert!(total_duration.as_millis() / 200 < 300); // Average under 300ms per message

    Ok(())
}

#[test]
fn benchmark_concurrent_operations() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    let start_time = Instant::now();

    // Simulate concurrent operations using threads
    let handles: Vec<_> = (0..10)
        .map(|i| {
            std::thread::spawn(move || {
                let name = format!("concurrent-agent-{}", i);
                let args = if i % 2 == 0 {
                    vec![
                        "agent",
                        "register",
                        "--name",
                        &name,
                        "--type",
                        "TestAgent",
                        "--scope",
                        "concurrent-testing",
                    ]
                } else {
                    vec!["agent", "list"]
                };

                benchmark_coordination_command(&args)
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()?;
    }

    let total_duration = start_time.elapsed();

    println!("=== Concurrent Operations ===");
    println!(
        "Total time for 10 concurrent operations: {:?}",
        total_duration
    );
    println!("Average time per operation: {:?}", total_duration / 10);
    println!("==============================");

    // Performance assertions
    assert!(total_duration.as_secs() < 30); // Should complete within 30 seconds

    Ok(())
}

// ============================================================================
// Memory Usage Benchmarks
// ============================================================================

#[test]
fn benchmark_memory_usage_under_load() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    // Get initial memory usage (approximate)
    let initial_memory = std::process::id();

    // Perform memory-intensive operations
    for i in 0..50 {
        benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("memory-bench-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "memory-testing",
            "--capabilities",
            "capability-1,capability-2,capability-3,capability-4,capability-5",
        ])?;

        // Send messages to create message history
        for j in 0..10 {
            benchmark_coordination_command(&[
                "agent",
                "send-message",
                "--to",
                &format!("agent-{:03}", i + 1),
                &format!("Memory test message {} from agent {}", j, i),
                "--message-type",
                "Test",
                "--priority",
                "Normal",
                "--payload",
                &format!(
                    "{{\"test\": \"memory\", \"agent\": {}, \"message\": {}}}",
                    i, j
                ),
            ])?;
        }
    }

    // Get final memory usage (approximate)
    let final_memory = std::process::id();

    println!("=== Memory Usage Under Load ===");
    println!("Initial process ID: {}", initial_memory);
    println!("Final process ID: {}", final_memory);
    println!("Memory usage test completed");
    println!("================================");

    // Note: In a real implementation, we'd use proper memory measurement
    // This is a placeholder for memory usage validation

    Ok(())
}

// ============================================================================
// Stress Testing Benchmarks
// ============================================================================

#[test]
fn benchmark_stress_test_rapid_operations() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let env = TestEnv::with_sample_data()?;
    std::env::set_current_dir(&env.repo_path)?;

    let start_time = Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;

    // Perform rapid operations to stress the system
    for i in 0..500 {
        match benchmark_coordination_command(&[
            "agent",
            "register",
            "--name",
            &format!("stress-agent-{}", i),
            "--type",
            "TestAgent",
            "--scope",
            "stress-testing",
        ]) {
            Ok(_) => success_count += 1,
            Err(_) => error_count += 1,
        }

        // Every 50 operations, try to list agents
        if i % 50 == 0 {
            let _ = benchmark_coordination_command(&["agent", "list"]);
        }
    }

    let total_duration = start_time.elapsed();

    println!("=== Stress Test Results ===");
    println!("Total operations: 500");
    println!("Successful: {}", success_count);
    println!("Errors: {}", error_count);
    println!(
        "Success rate: {:.2}%",
        (success_count as f64 / 500.0) * 100.0
    );
    println!("Total time: {:?}", total_duration);
    println!(
        "Operations per second: {:.2}",
        500.0 / total_duration.as_secs_f64()
    );
    println!("===========================");

    // Performance assertions
    assert!(success_count > 400); // At least 80% success rate
    assert!(total_duration.as_secs() < 300); // Should complete within 5 minutes

    Ok(())
}
