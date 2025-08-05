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

use clap::Subcommand;
use rhema_ai::agent::real_time_coordination::{
    AgentInfo, AgentMessage, AgentStatus, MessagePriority, MessageType,
    RealTimeCoordinationSystem,
};
use rhema_core::RhemaResult;
use std::collections::HashMap;
use std::sync::Arc;

/// Agent management commands
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

/// Coordination session management commands
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

/// System monitoring and statistics commands
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

/// Main coordination command enum
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

/// Coordination manager for CLI operations
pub struct CoordinationManager {
    coordination_system: Arc<RealTimeCoordinationSystem>,
}

impl CoordinationManager {
    /// Create a new coordination manager
    pub fn new() -> Self {
        Self {
            coordination_system: Arc::new(RealTimeCoordinationSystem::new()),
        }
    }

    /// Create a new coordination manager with custom configuration
    pub fn with_config(config: rhema_ai::agent::real_time_coordination::CoordinationConfig) -> Self {
        Self {
            coordination_system: Arc::new(RealTimeCoordinationSystem::with_config(config)),
        }
    }

    /// Execute agent subcommands
    pub async fn execute_agent_command(&self, cmd: &AgentSubcommands) -> RhemaResult<()> {
        match cmd {
            AgentSubcommands::Register { name, agent_type, scope, capabilities } => {
                self.register_agent(name, agent_type, scope, capabilities.as_deref()).await
            }
            AgentSubcommands::List { agent_type, status, scope, detailed } => {
                self.list_agents(agent_type.as_deref(), status.clone(), scope.as_deref(), *detailed).await
            }
            AgentSubcommands::Unregister { agent_id } => {
                self.unregister_agent(agent_id).await
            }
            AgentSubcommands::Status { agent_id, status } => {
                self.update_agent_status(agent_id, status.clone()).await
            }
            AgentSubcommands::Info { agent_id } => {
                self.get_agent_info(agent_id).await
            }
            AgentSubcommands::SendMessage { to, content, message_type, priority, payload, require_ack } => {
                self.send_message(to, content, &message_type, priority.clone(), payload.as_deref(), *require_ack).await
            }
            AgentSubcommands::Broadcast { content, message_type, priority, payload } => {
                self.broadcast_message(content, &message_type, priority.clone(), payload.as_deref()).await
            }
        }
    }

    /// Execute session subcommands
    pub async fn execute_session_command(&self, cmd: &SessionSubcommands) -> RhemaResult<()> {
        match cmd {
            SessionSubcommands::CreateSession { topic, participants } => {
                self.create_session(topic, participants).await
            }
            SessionSubcommands::ListSessions { active, detailed } => {
                self.list_sessions(*active, *detailed).await
            }
            SessionSubcommands::JoinSession { session_id, agent_id } => {
                self.join_session(session_id, agent_id).await
            }
            SessionSubcommands::LeaveSession { session_id, agent_id } => {
                self.leave_session(session_id, agent_id).await
            }
            SessionSubcommands::SendSessionMessage { session_id, content, message_type, priority, sender_id } => {
                self.send_session_message(session_id, content, &message_type, priority.clone(), sender_id).await
            }
            SessionSubcommands::SessionInfo { session_id } => {
                self.get_session_info(session_id).await
            }
        }
    }

    /// Execute system subcommands
    pub async fn execute_system_command(&self, cmd: &SystemSubcommands) -> RhemaResult<()> {
        match cmd {
            SystemSubcommands::Stats { detailed, export } => {
                self.show_stats(*detailed, export.as_deref()).await
            }
            SystemSubcommands::MessageHistory { limit, agent_id, message_type, show_payloads } => {
                self.show_message_history(*limit, agent_id.as_deref(), message_type.as_deref().map(|s| s.to_string()), *show_payloads).await
            }
            SystemSubcommands::Monitor { interval, agent_status, messages, sessions } => {
                self.monitor_system(*interval, *agent_status, *messages, *sessions).await
            }
            SystemSubcommands::Health { detailed, components } => {
                self.health_check(*detailed, components.as_deref()).await
            }
        }
    }

    // Agent management methods
    async fn register_agent(
        &self,
        name: &str,
        agent_type: &str,
        scope: &str,
        capabilities: Option<&str>,
    ) -> RhemaResult<()> {
        let capabilities_vec = capabilities
            .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        let agent_info = AgentInfo {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            agent_type: agent_type.to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: scope.to_string(),
            capabilities: capabilities_vec,
            last_heartbeat: chrono::Utc::now(),
            is_online: true,
            performance_metrics: rhema_ai::agent::real_time_coordination::AgentPerformanceMetrics {
                tasks_completed: 0,
                tasks_failed: 0,
                avg_completion_time_seconds: 0.0,
                success_rate: 1.0,
                collaboration_score: 0.0,
                avg_response_time_ms: 0.0,
            },
        };

        let agent_id = agent_info.id.clone();
        self.coordination_system.register_agent(agent_info).await?;
        println!("✅ Agent '{}' registered successfully with ID: {}", name, agent_id);
        Ok(())
    }

    async fn list_agents(
        &self,
        agent_type: Option<&str>,
        status: Option<AgentStatus>,
        scope: Option<&str>,
        detailed: bool,
    ) -> RhemaResult<()> {
        let agents = self.coordination_system.get_all_agents().await;

        let filtered_agents: Vec<_> = agents
            .into_iter()
            .filter(|agent| {
                agent_type.map_or(true, |t| agent.agent_type == t)
                    && status.as_ref().map_or(true, |s| agent.status == *s)
                    && scope.map_or(true, |s| agent.assigned_scope == s)
            })
            .collect();

        if filtered_agents.is_empty() {
            println!("No agents found matching the specified criteria.");
            return Ok(());
        }

        println!("Found {} agent(s):", filtered_agents.len());
        println!();

        for agent in filtered_agents {
            if detailed {
                println!("Agent ID: {}", agent.id);
                println!("  Name: {}", agent.name);
                println!("  Type: {}", agent.agent_type);
                println!("  Status: {:?}", agent.status);
                println!("  Scope: {}", agent.assigned_scope);
                println!("  Capabilities: {}", agent.capabilities.join(", "));
                println!("  Online: {}", agent.is_online);
                println!("  Last Heartbeat: {}", agent.last_heartbeat);
                println!("  Performance:");
                println!("    Tasks Completed: {}", agent.performance_metrics.tasks_completed);
                println!("    Success Rate: {:.2}%", agent.performance_metrics.success_rate * 100.0);
                println!("    Avg Response Time: {:.2}ms", agent.performance_metrics.avg_response_time_ms);
                println!();
            } else {
                println!("  {} ({}) - {:?} - {}", agent.name, agent.id, agent.status, agent.assigned_scope);
            }
        }

        Ok(())
    }

    async fn unregister_agent(&self, agent_id: &str) -> RhemaResult<()> {
        self.coordination_system.unregister_agent(agent_id).await?;
        println!("✅ Agent '{}' unregistered successfully", agent_id);
        Ok(())
    }

    async fn update_agent_status(&self, agent_id: &str, status: AgentStatus) -> RhemaResult<()> {
        let status_clone = status.clone();
        self.coordination_system.update_agent_status(agent_id, status).await?;
        println!("✅ Agent '{}' status updated to {:?}", agent_id, status_clone);
        Ok(())
    }

    async fn get_agent_info(&self, agent_id: &str) -> RhemaResult<()> {
        if let Some(agent) = self.coordination_system.get_agent_info(agent_id).await {
            println!("Agent Information:");
            println!("  ID: {}", agent.id);
            println!("  Name: {}", agent.name);
            println!("  Type: {}", agent.agent_type);
            println!("  Status: {:?}", agent.status);
            println!("  Scope: {}", agent.assigned_scope);
            println!("  Capabilities: {}", agent.capabilities.join(", "));
            println!("  Online: {}", agent.is_online);
            println!("  Last Heartbeat: {}", agent.last_heartbeat);
            println!("  Current Task: {:?}", agent.current_task_id);
            println!("  Performance Metrics:");
            println!("    Tasks Completed: {}", agent.performance_metrics.tasks_completed);
            println!("    Tasks Failed: {}", agent.performance_metrics.tasks_failed);
            println!("    Success Rate: {:.2}%", agent.performance_metrics.success_rate * 100.0);
            println!("    Avg Completion Time: {:.2}s", agent.performance_metrics.avg_completion_time_seconds);
            println!("    Collaboration Score: {:.2}", agent.performance_metrics.collaboration_score);
            println!("    Avg Response Time: {:.2}ms", agent.performance_metrics.avg_response_time_ms);
        } else {
            println!("❌ Agent '{}' not found", agent_id);
        }
        Ok(())
    }

    async fn send_message(
        &self,
        to: &str,
        content: &str,
        message_type: &str,
        priority: MessagePriority,
        payload: Option<&str>,
        require_ack: bool,
    ) -> RhemaResult<()> {
        let payload_value = if let Some(payload_str) = payload {
            Some(serde_json::from_str(payload_str)?)
        } else {
            None
        };

        let message_type_enum = match message_type {
            "TaskAssignment" => MessageType::TaskAssignment,
            "TaskCompletion" => MessageType::TaskCompletion,
            "TaskBlocked" => MessageType::TaskBlocked,
            "ResourceRequest" => MessageType::ResourceRequest,
            "ResourceRelease" => MessageType::ResourceRelease,
            "ConflictNotification" => MessageType::ConflictNotification,
            "CoordinationRequest" => MessageType::CoordinationRequest,
            "StatusUpdate" => MessageType::StatusUpdate,
            "KnowledgeShare" => MessageType::KnowledgeShare,
            "DecisionRequest" => MessageType::DecisionRequest,
            "DecisionResponse" => MessageType::DecisionResponse,
            _ => MessageType::Custom(message_type.to_string()),
        };

        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: message_type_enum,
            priority,
            sender_id: "cli".to_string(),
            recipient_ids: vec![to.to_string()],
            content: content.to_string(),
            payload: payload_value,
            timestamp: chrono::Utc::now(),
            requires_ack: require_ack,
            expires_at: None,
            metadata: HashMap::new(),
        };

        self.coordination_system.send_message(message).await?;
        println!("✅ Message sent to agent '{}'", to);
        Ok(())
    }

    async fn broadcast_message(
        &self,
        content: &str,
        message_type: &str,
        priority: MessagePriority,
        payload: Option<&str>,
    ) -> RhemaResult<()> {
        let payload_value = if let Some(payload_str) = payload {
            Some(serde_json::from_str(payload_str)?)
        } else {
            None
        };

        let message_type_enum = match message_type {
            "TaskAssignment" => MessageType::TaskAssignment,
            "TaskCompletion" => MessageType::TaskCompletion,
            "TaskBlocked" => MessageType::TaskBlocked,
            "ResourceRequest" => MessageType::ResourceRequest,
            "ResourceRelease" => MessageType::ResourceRelease,
            "ConflictNotification" => MessageType::ConflictNotification,
            "CoordinationRequest" => MessageType::CoordinationRequest,
            "StatusUpdate" => MessageType::StatusUpdate,
            "KnowledgeShare" => MessageType::KnowledgeShare,
            "DecisionRequest" => MessageType::DecisionRequest,
            "DecisionResponse" => MessageType::DecisionResponse,
            _ => MessageType::Custom(message_type.to_string()),
        };

        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: message_type_enum,
            priority,
            sender_id: "cli".to_string(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: payload_value,
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: HashMap::new(),
        };

        self.coordination_system.broadcast_message(message).await?;
        println!("✅ Message broadcasted to all agents");
        Ok(())
    }

    // Session management methods
    async fn create_session(&self, topic: &str, participants: &str) -> RhemaResult<()> {
        let participant_ids: Vec<String> = participants
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let session_id = self.coordination_system.create_session(topic.to_string(), participant_ids).await?;
        println!("✅ Coordination session created with ID: {}", session_id);
        println!("  Topic: {}", topic);
        Ok(())
    }

    async fn list_sessions(&self, _active_only: bool, _detailed: bool) -> RhemaResult<()> {
        // Note: This would need to be implemented in the coordination system
        // For now, we'll show a placeholder
        println!("Session listing functionality will be implemented in the coordination system");
        Ok(())
    }

    async fn join_session(&self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        self.coordination_system.join_session(session_id, agent_id).await?;
        println!("✅ Agent '{}' joined session '{}'", agent_id, session_id);
        Ok(())
    }

    async fn leave_session(&self, session_id: &str, agent_id: &str) -> RhemaResult<()> {
        self.coordination_system.leave_session(session_id, agent_id).await?;
        println!("✅ Agent '{}' left session '{}'", agent_id, session_id);
        Ok(())
    }

    async fn send_session_message(
        &self,
        session_id: &str,
        content: &str,
        message_type: &str,
        priority: MessagePriority,
        sender_id: &str,
    ) -> RhemaResult<()> {
        let message_type_enum = match message_type {
            "TaskAssignment" => MessageType::TaskAssignment,
            "TaskCompletion" => MessageType::TaskCompletion,
            "TaskBlocked" => MessageType::TaskBlocked,
            "ResourceRequest" => MessageType::ResourceRequest,
            "ResourceRelease" => MessageType::ResourceRelease,
            "ConflictNotification" => MessageType::ConflictNotification,
            "CoordinationRequest" => MessageType::CoordinationRequest,
            "StatusUpdate" => MessageType::StatusUpdate,
            "KnowledgeShare" => MessageType::KnowledgeShare,
            "DecisionRequest" => MessageType::DecisionRequest,
            "DecisionResponse" => MessageType::DecisionResponse,
            _ => MessageType::Custom(message_type.to_string()),
        };

        let message = AgentMessage {
            id: uuid::Uuid::new_v4().to_string(),
            message_type: message_type_enum,
            priority,
            sender_id: sender_id.to_string(),
            recipient_ids: vec![],
            content: content.to_string(),
            payload: None,
            timestamp: chrono::Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: HashMap::new(),
        };

        self.coordination_system.send_session_message(session_id, message).await?;
        println!("✅ Message sent to session '{}'", session_id);
        Ok(())
    }

    async fn get_session_info(&self, _session_id: &str) -> RhemaResult<()> {
        // Note: This would need to be implemented in the coordination system
        println!("Session info functionality will be implemented in the coordination system");
        Ok(())
    }

    // System monitoring methods
    async fn show_stats(&self, detailed: bool, export: Option<&str>) -> RhemaResult<()> {
        let stats = self.coordination_system.get_stats();
        
        if detailed {
            println!("Coordination System Statistics:");
            println!("  Total Messages: {}", stats.total_messages);
            println!("  Messages Delivered: {}", stats.messages_delivered);
            println!("  Messages Failed: {}", stats.messages_failed);
            println!("  Active Agents: {}", stats.active_agents);
            println!("  Active Sessions: {}", stats.active_sessions);
            println!("  Average Response Time: {:.2}ms", stats.avg_response_time_ms);
            println!("  Coordination Efficiency: {:.2}%", stats.coordination_efficiency * 100.0);
        } else {
            println!("Messages: {}/{} ({} failed), Agents: {}, Sessions: {}, Efficiency: {:.1}%",
                stats.messages_delivered, stats.total_messages, stats.messages_failed,
                stats.active_agents, stats.active_sessions, stats.coordination_efficiency * 100.0);
        }

        if let Some(export_path) = export {
            let stats_json = serde_json::to_string_pretty(&stats)?;
            std::fs::write(export_path, stats_json)?;
            println!("✅ Statistics exported to {}", export_path);
        }

        Ok(())
    }

    async fn show_message_history(
        &self,
        limit: usize,
        agent_id: Option<&str>,
        message_type: Option<String>,
        show_payloads: bool,
    ) -> RhemaResult<()> {
        let history = self.coordination_system.get_message_history(limit);
        
        let filtered_history: Vec<_> = history
            .into_iter()
            .filter(|msg| {
                agent_id.map_or(true, |id| {
                    msg.sender_id == id || msg.recipient_ids.contains(&id.to_string())
                })
                && message_type.as_ref().map_or(true, |t| {
                    match (&msg.message_type, t.as_str()) {
                        (MessageType::TaskAssignment, "TaskAssignment") => true,
                        (MessageType::TaskCompletion, "TaskCompletion") => true,
                        (MessageType::TaskBlocked, "TaskBlocked") => true,
                        (MessageType::ResourceRequest, "ResourceRequest") => true,
                        (MessageType::ResourceRelease, "ResourceRelease") => true,
                        (MessageType::ConflictNotification, "ConflictNotification") => true,
                        (MessageType::CoordinationRequest, "CoordinationRequest") => true,
                        (MessageType::StatusUpdate, "StatusUpdate") => true,
                        (MessageType::KnowledgeShare, "KnowledgeShare") => true,
                        (MessageType::DecisionRequest, "DecisionRequest") => true,
                        (MessageType::DecisionResponse, "DecisionResponse") => true,
                        (MessageType::Custom(custom), filter_type) => custom == filter_type,
                        _ => false,
                    }
                })
            })
            .collect();

        if filtered_history.is_empty() {
            println!("No messages found matching the specified criteria.");
            return Ok(());
        }

        println!("Message History (showing {} messages):", filtered_history.len());
        println!();

        for msg in filtered_history {
            println!("[{}] {} -> {}: {:?} (Priority: {:?})",
                msg.timestamp.format("%H:%M:%S"),
                msg.sender_id,
                if msg.recipient_ids.is_empty() { "ALL".to_string() } else { msg.recipient_ids.join(", ") },
                msg.message_type,
                msg.priority
            );
            println!("  Content: {}", msg.content);
            if show_payloads && msg.payload.is_some() {
                println!("  Payload: {}", serde_json::to_string_pretty(&msg.payload).unwrap());
            }
            println!();
        }

        Ok(())
    }

    async fn monitor_system(
        &self,
        interval: u64,
        agent_status: bool,
        messages: bool,
        sessions: bool,
    ) -> RhemaResult<()> {
        println!("Starting system monitoring (refresh every {}s)...", interval);
        println!("Press Ctrl+C to stop");
        println!();

        loop {
            let stats = self.coordination_system.get_stats();
            
            if agent_status {
                let agents = self.coordination_system.get_all_agents().await;
                println!("[{}] Active Agents: {}", 
                    chrono::Utc::now().format("%H:%M:%S"),
                    agents.len()
                );
            }

            if messages {
                println!("[{}] Messages: {}/{} ({} failed)", 
                    chrono::Utc::now().format("%H:%M:%S"),
                    stats.messages_delivered, stats.total_messages, stats.messages_failed
                );
            }

            if sessions {
                println!("[{}] Active Sessions: {}", 
                    chrono::Utc::now().format("%H:%M:%S"),
                    stats.active_sessions
                );
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
        }
    }

    async fn health_check(&self, detailed: bool, _components: Option<&str>) -> RhemaResult<()> {
        let stats = self.coordination_system.get_stats();
        
        println!("Coordination System Health Check:");
        println!("  Status: {}", if stats.coordination_efficiency > 0.8 { "✅ Healthy" } else { "⚠️  Degraded" });
        println!("  Coordination Efficiency: {:.1}%", stats.coordination_efficiency * 100.0);
        println!("  Message Success Rate: {:.1}%", 
            if stats.total_messages > 0 { 
                (stats.messages_delivered as f64 / stats.total_messages as f64) * 100.0 
            } else { 
                100.0 
            }
        );

        if detailed {
            println!("  Active Agents: {}", stats.active_agents);
            println!("  Active Sessions: {}", stats.active_sessions);
            println!("  Average Response Time: {:.2}ms", stats.avg_response_time_ms);
            println!("  Failed Messages: {}", stats.messages_failed);
        }

        Ok(())
    }
} 

/// Run coordination commands
pub async fn run(_rhema: &crate::Rhema, subcommand: &CoordinationSubcommands) -> RhemaResult<()> {
    let manager = CoordinationManager::new();
    
    match subcommand {
        CoordinationSubcommands::Agent { subcommand } => {
            manager.execute_agent_command(subcommand).await
        }
        CoordinationSubcommands::Session { subcommand } => {
            manager.execute_session_command(subcommand).await
        }
        CoordinationSubcommands::System { subcommand } => {
            manager.execute_system_command(subcommand).await
        }
    }
} 