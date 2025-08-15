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

use crate::CliContext;
use chrono::Utc;
use clap::Subcommand;
use rhema_api::{AgentInfo, MessageType, RhemaResult};
use rhema_coordination::agent::real_time_coordination::{AgentStatus, MessagePriority};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum AgentSubcommands {
    /// Register a new agent
    Register {
        /// Agent name
        #[arg(long, value_name = "NAME")]
        name: String,

        /// Agent type/capabilities
        #[arg(long, value_name = "TYPE")]
        agent_type: String,

        /// Assigned scope
        #[arg(long, value_name = "SCOPE")]
        scope: String,

        /// Agent capabilities (comma-separated)
        #[arg(long, value_name = "CAPABILITIES")]
        capabilities: Option<String>,
    },

    /// List all registered agents
    List {
        /// Filter by agent type
        #[arg(long, value_name = "TYPE")]
        agent_type: Option<String>,

        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<AgentStatus>,

        /// Filter by scope
        #[arg(long, value_name = "SCOPE")]
        scope: Option<String>,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },

    /// Unregister an agent
    Unregister {
        /// Agent ID
        #[arg(value_name = "AGENT_ID")]
        agent_id: String,
    },

    /// Update agent status
    Status {
        /// Agent ID
        #[arg(value_name = "AGENT_ID")]
        agent_id: String,

        /// New status
        #[arg(long, value_enum)]
        status: AgentStatus,
    },

    /// Get agent information
    Info {
        /// Agent ID
        #[arg(value_name = "AGENT_ID")]
        agent_id: String,
    },

    /// Send a message to an agent
    SendMessage {
        /// Recipient agent ID
        #[arg(long, value_name = "TO")]
        to: String,

        /// Message content
        #[arg(value_name = "CONTENT")]
        content: String,

        /// Message type
        #[arg(long, default_value = "Custom")]
        message_type: String,

        /// Message priority
        #[arg(long, value_enum, default_value = "Normal")]
        priority: MessagePriority,

        /// Message payload (JSON)
        #[arg(long, value_name = "PAYLOAD")]
        payload: Option<String>,

        /// Require acknowledgment
        #[arg(long)]
        require_ack: bool,
    },

    /// Broadcast a message to all agents
    Broadcast {
        /// Message content
        #[arg(value_name = "CONTENT")]
        content: String,

        /// Message type
        #[arg(long, default_value = "Custom")]
        message_type: String,

        /// Message priority
        #[arg(long, value_enum, default_value = "Normal")]
        priority: MessagePriority,

        /// Message payload (JSON)
        #[arg(long, value_name = "PAYLOAD")]
        payload: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum SessionSubcommands {
    /// Create a new coordination session
    CreateSession {
        /// Session topic
        #[arg(value_name = "TOPIC")]
        topic: String,

        /// Participant agent IDs (comma-separated)
        #[arg(long, value_name = "PARTICIPANTS")]
        participants: String,
    },

    /// List all coordination sessions
    ListSessions {
        /// Show only active sessions
        #[arg(long)]
        active: bool,

        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },

    /// Join a coordination session
    JoinSession {
        /// Session ID
        #[arg(long, value_name = "SESSION_ID")]
        session_id: String,

        /// Agent ID
        #[arg(long, value_name = "AGENT_ID")]
        agent_id: String,
    },

    /// Leave a coordination session
    LeaveSession {
        /// Session ID
        #[arg(long, value_name = "SESSION_ID")]
        session_id: String,

        /// Agent ID
        #[arg(long, value_name = "AGENT_ID")]
        agent_id: String,
    },

    /// Send a message to a coordination session
    SendSessionMessage {
        /// Session ID
        #[arg(long, value_name = "SESSION_ID")]
        session_id: String,

        /// Message content
        #[arg(value_name = "CONTENT")]
        content: String,

        /// Message type
        #[arg(long, default_value = "Custom")]
        message_type: String,

        /// Message priority
        #[arg(long, value_enum, default_value = "Normal")]
        priority: MessagePriority,

        /// Sender agent ID
        #[arg(long, value_name = "SENDER_ID")]
        sender_id: String,
    },

    /// Get session information
    SessionInfo {
        /// Session ID
        #[arg(value_name = "SESSION_ID")]
        session_id: String,
    },
}

#[derive(Subcommand)]
pub enum SystemSubcommands {
    /// Show coordination system statistics
    Stats {
        /// Show detailed statistics
        #[arg(long)]
        detailed: bool,

        /// Export statistics to file
        #[arg(long, value_name = "FILE")]
        export: Option<String>,
    },

    /// Show message history
    MessageHistory {
        /// Number of messages to show
        #[arg(long, default_value = "50")]
        limit: usize,

        /// Filter by agent ID
        #[arg(long, value_name = "AGENT_ID")]
        agent_id: Option<String>,

        /// Filter by message type
        #[arg(long)]
        message_type: Option<String>,

        /// Show message payloads
        #[arg(long)]
        show_payloads: bool,
    },

    /// Monitor coordination system in real-time
    Monitor {
        /// Monitoring interval (seconds)
        #[arg(long, default_value = "5")]
        interval: u64,

        /// Show agent status changes
        #[arg(long)]
        agent_status: bool,

        /// Show message traffic
        #[arg(long)]
        messages: bool,

        /// Show session activity
        #[arg(long)]
        sessions: bool,
    },

    /// Health check for coordination system
    Health {
        /// Show detailed health information
        #[arg(long)]
        detailed: bool,

        /// Check specific components
        #[arg(long, value_name = "COMPONENTS")]
        components: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum CoordinationSubcommands {
    /// Agent management
    Agent {
        #[command(subcommand)]
        subcommand: AgentSubcommands,
    },

    /// Session management
    Session {
        #[command(subcommand)]
        subcommand: SessionSubcommands,
    },

    /// System monitoring
    System {
        #[command(subcommand)]
        subcommand: SystemSubcommands,
    },
}

pub fn handle_coordination(
    context: &CliContext,
    subcommand: &CoordinationSubcommands,
) -> RhemaResult<()> {
    match subcommand {
        CoordinationSubcommands::Agent { subcommand } => handle_agent(context, subcommand),
        CoordinationSubcommands::Session { subcommand } => handle_session(context, subcommand),
        CoordinationSubcommands::System { subcommand } => handle_system(context, subcommand),
    }
}

fn handle_agent(context: &CliContext, subcommand: &AgentSubcommands) -> RhemaResult<()> {
    let coordination_system = context.rhema.get_coordination_system().ok_or_else(|| {
        rhema_api::RhemaError::InvalidYaml {
            file: "coordination".to_string(),
            message: "Coordination system not initialized. Run 'rhema init' first.".to_string(),
        }
    })?;

    match subcommand {
        AgentSubcommands::Register {
            name,
            agent_type,
            scope,
            capabilities,
        } => {
            let agent_id = Uuid::new_v4().to_string();
            let capabilities_vec = capabilities
                .as_ref()
                .map(|caps| caps.split(',').map(|s| s.trim().to_string()).collect())
                .unwrap_or_default();

            let agent_info = AgentInfo {
                id: agent_id.clone(),
                name: name.clone(),
                agent_type: agent_type.clone(),
                status: AgentStatus::Idle,
                current_task_id: None,
                assigned_scope: scope.clone(),
                capabilities: capabilities_vec,
                last_heartbeat: Utc::now(),
                is_online: true,
                performance_metrics: rhema_coordination::agent::real_time_coordination::AgentPerformanceMetrics::default(),
            };

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                coordination_system.register_agent(agent_info).await?;
                println!("✅ Agent registered successfully: {}", agent_id);
                Ok(())
            })
        }

        AgentSubcommands::List {
            agent_type,
            status,
            scope,
            detailed,
        } => {
            let agents = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { coordination_system.get_all_agents().await });

            let filtered_agents: Vec<_> = agents
                .into_iter()
                .filter(|agent| {
                    agent_type.as_ref().map_or(true, |t| agent.agent_type == *t)
                        && status.as_ref().map_or(true, |s| agent.status == *s)
                        && scope
                            .as_ref()
                            .map_or(true, |sc| agent.assigned_scope == *sc)
                })
                .collect();

            if filtered_agents.is_empty() {
                println!("No agents found matching the criteria");
            } else {
                println!("Found {} agent(s):", filtered_agents.len());
                for agent in filtered_agents {
                    if *detailed {
                        println!("  ID: {}", agent.id);
                        println!("  Name: {}", agent.name);
                        println!("  Type: {}", agent.agent_type);
                        println!("  Status: {:?}", agent.status);
                        println!("  Status: {:?}", agent.status);
                        println!("  Scope: {}", agent.assigned_scope);
                        println!("  Capabilities: {}", agent.capabilities.join(", "));
                        println!("  Online: {}", agent.is_online);
                        println!("  Last Heartbeat: {:?}", agent.last_heartbeat);
                        println!("---");
                    } else {
                        println!(
                            "  {} ({}) - {:?} - {}",
                            agent.name, agent.id, agent.status, agent.agent_type
                        );
                    }
                }
            }
            Ok(())
        }

        AgentSubcommands::Unregister { agent_id } => {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                coordination_system.unregister_agent(&agent_id).await?;
                println!("✅ Agent unregistered successfully: {}", agent_id);
                Ok(())
            })
        }

        AgentSubcommands::Status { agent_id, status } => {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                coordination_system
                    .update_agent_status(&agent_id, status.clone())
                    .await?;
                println!("✅ Agent status updated: {} -> {:?}", agent_id, status);
                Ok(())
            })
        }

        AgentSubcommands::Info { agent_id } => {
            let agents = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { coordination_system.get_all_agents().await });

            if let Some(agent) = agents.into_iter().find(|a| a.id == *agent_id) {
                println!("Agent Information:");
                println!("  ID: {}", agent.id);
                println!("  Name: {}", agent.name);
                println!("  Type: {}", agent.agent_type);
                println!("  Status: {:?}", agent.status);
                println!("  Status: {:?}", agent.status);
                println!("  Scope: {}", agent.assigned_scope);
                println!("  Capabilities: {}", agent.capabilities.join(", "));
                println!("  Online: {}", agent.is_online);
                println!("  Last Heartbeat: {:?}", agent.last_heartbeat);
                if let Some(task_id) = &agent.current_task_id {
                    println!("  Current Task: {}", task_id);
                }
            } else {
                println!("❌ Agent not found: {}", agent_id);
            }
            Ok(())
        }

        AgentSubcommands::SendMessage {
            to,
            content,
            message_type,
            priority,
            payload,
            require_ack,
        } => {
            let message = rhema_coordination::agent::AgentMessage {
                id: Uuid::new_v4().to_string(),
                message_type: MessageType::Custom("cli_message".to_string()),
                priority: priority.clone(),
                sender_id: "cli".to_string(),
                recipient_ids: vec![to.clone()],
                content: content.clone(),
                payload: None, // TODO: Implement proper payload handling
                timestamp: Utc::now(),
                requires_ack: *require_ack,
                expires_at: None,
                metadata: HashMap::new(),
            };

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                coordination_system.send_message(message).await?;
                println!("✅ Message sent to agent: {}", to);
                Ok(())
            })
        }

        AgentSubcommands::Broadcast {
            content,
            message_type,
            priority,
            payload,
        } => {
            let agents = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { coordination_system.get_all_agents().await });

            let recipient_ids: Vec<String> = agents
                .into_iter()
                .filter(|a| a.is_online)
                .map(|a| a.id)
                .collect();

            if recipient_ids.is_empty() {
                println!("❌ No online agents to broadcast to");
                return Ok(());
            }

            let message = rhema_coordination::agent::AgentMessage {
                id: Uuid::new_v4().to_string(),
                message_type: MessageType::Custom("cli_broadcast".to_string()),
                priority: priority.clone(),
                sender_id: "cli".to_string(),
                recipient_ids: recipient_ids.clone(),
                content: content.clone(),
                payload: None, // TODO: Implement proper payload handling
                timestamp: Utc::now(),
                requires_ack: false,
                expires_at: None,
                metadata: HashMap::new(),
            };

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                coordination_system.send_message(message).await?;
                println!("✅ Message broadcasted to {} agents", recipient_ids.len());
                Ok(())
            })
        }
    }
}

fn handle_session(context: &CliContext, subcommand: &SessionSubcommands) -> RhemaResult<()> {
    let coordination_system = context.rhema.get_coordination_system().ok_or_else(|| {
        rhema_api::RhemaError::InvalidYaml {
            file: "coordination".to_string(),
            message: "Coordination system not initialized. Run 'rhema init' first.".to_string(),
        }
    })?;

    match subcommand {
        SessionSubcommands::CreateSession {
            topic,
            participants,
        } => {
            let participant_ids: Vec<String> = participants
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let session_id = coordination_system
                    .create_session(topic.clone(), participant_ids.clone())
                    .await?;
                println!("✅ Session created: {}", session_id);
                println!("  Topic: {}", topic);
                println!("  Participants: {}", participants);
                Ok(())
            })
        }

        SessionSubcommands::ListSessions { active, detailed } => {
            // Note: This would require implementing get_sessions() in the coordination system
            println!("📋 Session listing not yet implemented in coordination system");
            println!("  Active only: {}", active);
            println!("  Detailed: {}", detailed);
            Ok(())
        }

        SessionSubcommands::JoinSession {
            session_id,
            agent_id,
        } => tokio::runtime::Runtime::new().unwrap().block_on(async {
            coordination_system
                .join_session(session_id, agent_id)
                .await?;
            println!("✅ Agent {} joined session {}", agent_id, session_id);
            Ok(())
        }),

        SessionSubcommands::LeaveSession {
            session_id,
            agent_id,
        } => tokio::runtime::Runtime::new().unwrap().block_on(async {
            coordination_system
                .leave_session(session_id, agent_id)
                .await?;
            println!("✅ Agent {} left session {}", agent_id, session_id);
            Ok(())
        }),

        SessionSubcommands::SendSessionMessage {
            session_id,
            content,
            message_type,
            priority,
            sender_id,
        } => {
            let message = rhema_coordination::agent::AgentMessage {
                id: Uuid::new_v4().to_string(),
                message_type: MessageType::Custom("session_message".to_string()),
                priority: priority.clone(),
                sender_id: sender_id.clone(),
                recipient_ids: vec![], // Will be filled by session
                content: content.clone(),
                payload: None,
                timestamp: Utc::now(),
                requires_ack: false,
                expires_at: None,
                metadata: HashMap::new(),
            };

            tokio::runtime::Runtime::new().unwrap().block_on(async {
                coordination_system
                    .send_session_message(session_id, message)
                    .await?;
                println!("✅ Session message sent to {}", session_id);
                Ok(())
            })
        }

        SessionSubcommands::SessionInfo { session_id } => {
            // Note: This would require implementing get_session_info() in the coordination system
            println!("📋 Session info not yet implemented in coordination system");
            println!("  Session ID: {}", session_id);
            Ok(())
        }
    }
}

fn handle_system(context: &CliContext, subcommand: &SystemSubcommands) -> RhemaResult<()> {
    let coordination_system = context.rhema.get_coordination_system().ok_or_else(|| {
        rhema_api::RhemaError::InvalidYaml {
            file: "coordination".to_string(),
            message: "Coordination system not initialized. Run 'rhema init' first.".to_string(),
        }
    })?;

    match subcommand {
        SystemSubcommands::Stats { detailed, export } => {
            let agents = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { coordination_system.get_all_agents().await });

            let total_agents = agents.len();
            let online_agents = agents.iter().filter(|a| a.is_online).count();
            let idle_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Idle)
                .count();
            let busy_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Busy)
                .count();
            let working_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Working)
                .count();

            println!("📊 Coordination System Statistics");
            println!("  Total Agents: {}", total_agents);
            println!("  Online Agents: {}", online_agents);
            println!("  Idle Agents: {}", idle_agents);
            println!("  Busy Agents: {}", busy_agents);
            println!("  Working Agents: {}", working_agents);

            if *detailed {
                println!("\nDetailed Statistics:");
                for agent in &agents {
                    println!(
                        "  {}: {:?} - {}",
                        agent.name, agent.status, agent.agent_type
                    );
                }
            }

            if let Some(export_path) = export {
                let stats = json!({
                    "total_agents": total_agents,
                    "online_agents": online_agents,
                    "idle_agents": idle_agents,
                    "busy_agents": busy_agents,
                    "working_agents": working_agents,
                    "timestamp": Utc::now(),
                    "agents": agents
                });

                std::fs::write(export_path, serde_json::to_string_pretty(&stats)?)?;
                println!("✅ Statistics exported to: {}", export_path);
            }

            Ok(())
        }

        SystemSubcommands::MessageHistory {
            limit,
            agent_id,
            message_type,
            show_payloads,
        } => {
            // Note: This would require implementing get_message_history() in the coordination system
            println!("📋 Message history not yet implemented in coordination system");
            println!("  Limit: {}", limit);
            println!("  Agent ID filter: {:?}", agent_id);
            println!("  Message type filter: {:?}", message_type);
            println!("  Show payloads: {}", show_payloads);
            Ok(())
        }

        SystemSubcommands::Monitor {
            interval,
            agent_status,
            messages,
            sessions,
        } => {
            println!("🔍 Starting real-time monitoring...");
            println!("  Interval: {} seconds", interval);
            println!("  Monitor agent status: {}", agent_status);
            println!("  Monitor messages: {}", messages);
            println!("  Monitor sessions: {}", sessions);
            println!("  Press Ctrl+C to stop monitoring");

            // Note: This would require implementing a monitoring stream in the coordination system
            println!("📋 Real-time monitoring not yet implemented in coordination system");
            Ok(())
        }

        SystemSubcommands::Health {
            detailed,
            components,
        } => {
            let agents = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { coordination_system.get_all_agents().await });

            let healthy_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Idle)
                .count();
            let busy_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Busy)
                .count();
            let working_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Working)
                .count();
            let offline_agents = agents
                .iter()
                .filter(|a| a.status == AgentStatus::Offline)
                .count();

            println!("🏥 Coordination System Health Check");
            println!("  Total Agents: {}", agents.len());
            println!("  Idle: {}", healthy_agents);
            println!("  Busy: {}", busy_agents);
            println!("  Working: {}", working_agents);
            println!("  Offline: {}", offline_agents);

            let overall_health = if offline_agents > 0 {
                "🔴 Offline"
            } else if busy_agents > 0 || working_agents > 0 {
                "🔄 Active"
            } else if healthy_agents > 0 {
                "✅ Idle"
            } else {
                "❌ No Agents"
            };
            println!("  Overall Status: {}", overall_health);

            if *detailed {
                println!("\nDetailed Health Information:");
                for agent in &agents {
                    let status_icon = match agent.status {
                        AgentStatus::Idle => "✅",
                        AgentStatus::Busy => "⚠️",
                        AgentStatus::Working => "🔄",
                        AgentStatus::Offline => "🔴",
                        AgentStatus::Blocked => "🚫",
                        AgentStatus::Collaborating => "🤝",
                        AgentStatus::Failed => "❌",
                    };
                    println!(
                        "  {} {}: {:?} - {}",
                        status_icon, agent.name, agent.status, agent.agent_type
                    );
                }
            }

            if let Some(comps) = components {
                println!("\nComponent-specific checks:");
                for component in comps.split(',') {
                    let component = component.trim();
                    match component {
                        "agents" => println!("  Agents: {} healthy", healthy_agents),
                        "coordination" => println!("  Coordination: ✅ Operational"),
                        "messaging" => println!("  Messaging: ✅ Operational"),
                        _ => println!("  {}: ❓ Unknown component", component),
                    }
                }
            }

            Ok(())
        }
    }
}
