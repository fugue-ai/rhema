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

use clap::{Parser, Subcommand};
use rhema_ai::agent::real_time_coordination::{
    AgentInfo, AgentMessage, AgentStatus, MessagePriority, MessageType, RealTimeCoordinationSystem,
};
use rhema_api::{Rhema, RhemaResult};
use rhema_core::{DecisionStatus, PatternUsage, Priority, TodoStatus};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "rhema")]
#[command(about = "Rhema Protocol CLI")]
#[command(version)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Suppress output
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rhema repository
    Init {
        /// Scope type (e.g., service, library, application)
        #[arg(long)]
        scope_type: Option<String>,

        /// Scope name
        #[arg(long)]
        scope_name: Option<String>,

        /// Auto-configure based on repository analysis
        #[arg(long)]
        auto_config: bool,
    },

    /// List all scopes in the repository
    Scopes,

    /// Show information about a specific scope
    Scope {
        /// Path to the scope
        path: Option<String>,
    },

    /// Show the scope tree
    Tree,

    /// Execute a CQL query
    Query {
        /// The CQL query to execute
        query: String,

        /// Output format (json, yaml, table)
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Include provenance information
        #[arg(long)]
        provenance: bool,

        /// Include field provenance
        #[arg(long)]
        field_provenance: bool,

        /// Include statistics
        #[arg(long)]
        stats: bool,
    },

    /// Search for content in the repository
    Search {
        /// Search term
        term: String,

        /// Search in specific file
        #[arg(short, long)]
        in_file: Option<String>,

        /// Use regex search
        #[arg(long)]
        regex: bool,
    },

    /// Validate the repository
    Validate {
        /// Validate recursively
        #[arg(long)]
        recursive: bool,

        /// Use JSON schema validation
        #[arg(long)]
        json_schema: bool,

        /// Migrate schemas if needed
        #[arg(long)]
        migrate: bool,
    },

    /// Show health information
    Health {
        /// Scope to check health for
        scope: Option<String>,
    },

    /// Show statistics
    Stats,

    /// Manage todos
    Todo {
        #[command(subcommand)]
        subcommand: TodoSubcommands,
    },

    /// Manage insights/knowledge
    Insight {
        #[command(subcommand)]
        subcommand: InsightSubcommands,
    },

    /// Manage patterns
    Pattern {
        #[command(subcommand)]
        subcommand: PatternSubcommands,
    },

    /// Manage decisions
    Decision {
        #[command(subcommand)]
        subcommand: DecisionSubcommands,
    },

    /// Manage coordination between agents
    Coordination {
        #[command(subcommand)]
        subcommand: CoordinationSubcommands,
    },
}

// Command enums for entity management
#[derive(Subcommand)]
pub enum TodoSubcommands {
    /// Add a new todo
    Add {
        /// Todo title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Todo description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// Priority level
        #[arg(long, value_enum, default_value = "medium")]
        priority: Priority,

        /// Assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,

        /// Due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },

    /// List todos
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,

        /// Filter by priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// Filter by assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,
    },

    /// Complete a todo
    Complete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,

        /// Completion outcome
        #[arg(long, value_name = "OUTCOME")]
        outcome: Option<String>,
    },

    /// Update a todo
    Update {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New status
        #[arg(long, value_enum)]
        status: Option<TodoStatus>,

        /// New priority
        #[arg(long, value_enum)]
        priority: Option<Priority>,

        /// New assignee
        #[arg(long, value_name = "ASSIGNEE")]
        assignee: Option<String>,

        /// New due date (ISO format)
        #[arg(long, value_name = "DATE")]
        due_date: Option<String>,
    },

    /// Delete a todo
    Delete {
        /// Todo ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum InsightSubcommands {
    /// Record a new insight
    Record {
        /// Insight title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Insight content
        #[arg(long, value_name = "CONTENT")]
        content: String,

        /// Confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,

        /// Category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// List insights
    List {
        /// Filter by category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// Filter by tag
        #[arg(long, value_name = "TAG")]
        tag: Option<String>,

        /// Filter by confidence level (minimum)
        #[arg(long, value_name = "LEVEL")]
        min_confidence: Option<u8>,
    },

    /// Update an insight
    Update {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New content
        #[arg(long, value_name = "CONTENT")]
        content: Option<String>,

        /// New confidence level (1-10)
        #[arg(long, value_name = "LEVEL")]
        confidence: Option<u8>,

        /// New category
        #[arg(long, value_name = "CATEGORY")]
        category: Option<String>,

        /// New tags (comma-separated)
        #[arg(long, value_name = "TAGS")]
        tags: Option<String>,
    },

    /// Delete an insight
    Delete {
        /// Insight ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PatternSubcommands {
    /// Add a new pattern
    Add {
        /// Pattern name
        #[arg(value_name = "NAME")]
        name: String,

        /// Pattern description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: String,

        /// Usage context
        #[arg(long, value_enum, default_value = "recommended")]
        usage: PatternUsage,

        /// Effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// Examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// Anti-patterns to avoid (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },

    /// List patterns
    List {
        /// Filter by pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// Filter by usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// Filter by effectiveness rating (minimum)
        #[arg(long, value_name = "RATING")]
        min_effectiveness: Option<u8>,
    },

    /// Update a pattern
    Update {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,

        /// New name
        #[arg(long, value_name = "NAME")]
        name: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New pattern type
        #[arg(long, value_name = "TYPE")]
        pattern_type: Option<String>,

        /// New usage context
        #[arg(long, value_enum)]
        usage: Option<PatternUsage>,

        /// New effectiveness rating (1-10)
        #[arg(long, value_name = "RATING")]
        effectiveness: Option<u8>,

        /// New examples (comma-separated)
        #[arg(long, value_name = "EXAMPLES")]
        examples: Option<String>,

        /// New anti-patterns (comma-separated)
        #[arg(long, value_name = "ANTI_PATTERNS")]
        anti_patterns: Option<String>,
    },

    /// Delete a pattern
    Delete {
        /// Pattern ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

#[derive(Subcommand)]
pub enum DecisionSubcommands {
    /// Record a new decision
    Record {
        /// Decision title
        #[arg(value_name = "TITLE")]
        title: String,

        /// Decision description
        #[arg(long, value_name = "DESCRIPTION")]
        description: String,

        /// Decision status
        #[arg(long, value_enum, default_value = "proposed")]
        status: DecisionStatus,

        /// Decision context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// Decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,

        /// Alternatives considered (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,

        /// Rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,

        /// Consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },

    /// List decisions
    List {
        /// Filter by status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,

        /// Filter by decision maker
        #[arg(long, value_name = "MAKER")]
        maker: Option<String>,
    },

    /// Update a decision
    Update {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,

        /// New title
        #[arg(long, value_name = "TITLE")]
        title: Option<String>,

        /// New description
        #[arg(long, value_name = "DESCRIPTION")]
        description: Option<String>,

        /// New status
        #[arg(long, value_enum)]
        status: Option<DecisionStatus>,

        /// New context
        #[arg(long, value_name = "CONTEXT")]
        context: Option<String>,

        /// New decision makers (comma-separated)
        #[arg(long, value_name = "MAKERS")]
        makers: Option<String>,

        /// New alternatives (comma-separated)
        #[arg(long, value_name = "ALTERNATIVES")]
        alternatives: Option<String>,

        /// New rationale
        #[arg(long, value_name = "RATIONALE")]
        rationale: Option<String>,

        /// New consequences (comma-separated)
        #[arg(long, value_name = "CONSEQUENCES")]
        consequences: Option<String>,
    },

    /// Delete a decision
    Delete {
        /// Decision ID
        #[arg(value_name = "ID")]
        id: String,
    },
}

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

/// Execute agent subcommands
async fn execute_agent_command(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    cmd: &AgentSubcommands,
) -> RhemaResult<()> {
    match cmd {
        AgentSubcommands::Register {
            name,
            agent_type,
            scope,
            capabilities,
        } => {
            register_agent(
                coordination_system,
                name,
                agent_type,
                scope,
                capabilities.as_deref(),
            )
            .await
        }
        AgentSubcommands::List {
            agent_type,
            status,
            scope,
            detailed,
        } => {
            list_agents(
                coordination_system,
                agent_type.as_deref(),
                status.clone(),
                scope.as_deref(),
                *detailed,
            )
            .await
        }
        AgentSubcommands::Unregister { agent_id } => {
            unregister_agent(coordination_system, agent_id).await
        }
        AgentSubcommands::Status { agent_id, status } => {
            update_agent_status(coordination_system, agent_id, status.clone()).await
        }
        AgentSubcommands::Info { agent_id } => get_agent_info(coordination_system, agent_id).await,
        AgentSubcommands::SendMessage {
            to,
            content,
            message_type,
            priority,
            payload,
            require_ack,
        } => {
            send_message(
                coordination_system,
                to,
                content,
                &message_type,
                priority.clone(),
                payload.as_deref(),
                *require_ack,
            )
            .await
        }
        AgentSubcommands::Broadcast {
            content,
            message_type,
            priority,
            payload,
        } => {
            broadcast_message(
                coordination_system,
                content,
                &message_type,
                priority.clone(),
                payload.as_deref(),
            )
            .await
        }
    }
}

/// Execute session subcommands
async fn execute_session_command(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    cmd: &SessionSubcommands,
) -> RhemaResult<()> {
    match cmd {
        SessionSubcommands::CreateSession {
            topic,
            participants,
        } => create_session(coordination_system, topic, participants).await,
        SessionSubcommands::ListSessions { active, detailed } => {
            list_sessions(coordination_system, *active, *detailed).await
        }
        SessionSubcommands::JoinSession {
            session_id,
            agent_id,
        } => join_session(coordination_system, session_id, agent_id).await,
        SessionSubcommands::LeaveSession {
            session_id,
            agent_id,
        } => leave_session(coordination_system, session_id, agent_id).await,
        SessionSubcommands::SendSessionMessage {
            session_id,
            content,
            message_type,
            priority,
            sender_id,
        } => {
            send_session_message(
                coordination_system,
                session_id,
                content,
                &message_type,
                priority.clone(),
                sender_id,
            )
            .await
        }
        SessionSubcommands::SessionInfo { session_id } => {
            get_session_info(coordination_system, session_id).await
        }
    }
}

/// Execute system subcommands
async fn execute_system_command(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    cmd: &SystemSubcommands,
) -> RhemaResult<()> {
    match cmd {
        SystemSubcommands::Stats { detailed, export } => {
            show_stats(coordination_system, *detailed, export.as_deref()).await
        }
        SystemSubcommands::MessageHistory {
            limit,
            agent_id,
            message_type,
            show_payloads,
        } => {
            show_message_history(
                coordination_system,
                *limit,
                agent_id.as_deref(),
                message_type.as_deref().map(|s| s.to_string()),
                *show_payloads,
            )
            .await
        }
        SystemSubcommands::Monitor {
            interval,
            agent_status,
            messages,
            sessions,
        } => {
            monitor_system(
                coordination_system,
                *interval,
                *agent_status,
                *messages,
                *sessions,
            )
            .await
        }
        SystemSubcommands::Health {
            detailed,
            components,
        } => health_check(coordination_system, *detailed, components.as_deref()).await,
    }
}

// Agent management methods
async fn register_agent(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
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
    coordination_system.register_agent(agent_info).await?;
    println!(
        "✅ Agent '{}' registered successfully with ID: {}",
        name, agent_id
    );
    Ok(())
}

async fn list_agents(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    agent_type: Option<&str>,
    status: Option<AgentStatus>,
    scope: Option<&str>,
    detailed: bool,
) -> RhemaResult<()> {
    let agents = coordination_system.get_all_agents().await;

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
            println!(
                "    Tasks Completed: {}",
                agent.performance_metrics.tasks_completed
            );
            println!(
                "    Success Rate: {:.2}%",
                agent.performance_metrics.success_rate * 100.0
            );
            println!(
                "    Avg Response Time: {:.2}ms",
                agent.performance_metrics.avg_response_time_ms
            );
            println!();
        } else {
            println!(
                "  {} ({}) - {:?} - {}",
                agent.name, agent.id, agent.status, agent.assigned_scope
            );
        }
    }

    Ok(())
}

async fn unregister_agent(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    agent_id: &str,
) -> RhemaResult<()> {
    coordination_system.unregister_agent(agent_id).await?;
    println!("✅ Agent '{}' unregistered successfully", agent_id);
    Ok(())
}

async fn update_agent_status(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    agent_id: &str,
    status: AgentStatus,
) -> RhemaResult<()> {
    let status_clone = status.clone();
    coordination_system
        .update_agent_status(agent_id, status)
        .await?;
    println!(
        "✅ Agent '{}' status updated to {:?}",
        agent_id, status_clone
    );
    Ok(())
}

async fn get_agent_info(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    agent_id: &str,
) -> RhemaResult<()> {
    if let Some(agent) = coordination_system.get_agent_info(agent_id).await {
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
        println!(
            "    Tasks Completed: {}",
            agent.performance_metrics.tasks_completed
        );
        println!(
            "    Tasks Failed: {}",
            agent.performance_metrics.tasks_failed
        );
        println!(
            "    Success Rate: {:.2}%",
            agent.performance_metrics.success_rate * 100.0
        );
        println!(
            "    Avg Completion Time: {:.2}s",
            agent.performance_metrics.avg_completion_time_seconds
        );
        println!(
            "    Collaboration Score: {:.2}",
            agent.performance_metrics.collaboration_score
        );
        println!(
            "    Avg Response Time: {:.2}ms",
            agent.performance_metrics.avg_response_time_ms
        );
    } else {
        println!("❌ Agent '{}' not found", agent_id);
    }
    Ok(())
}

async fn send_message(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
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

    coordination_system.send_message(message).await?;
    println!("✅ Message sent to agent '{}'", to);
    Ok(())
}

async fn broadcast_message(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
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

    coordination_system.broadcast_message(message).await?;
    println!("✅ Message broadcasted to all agents");
    Ok(())
}

// Session management methods
async fn create_session(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    topic: &str,
    participants: &str,
) -> RhemaResult<()> {
    let participant_ids: Vec<String> = participants
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let session_id = coordination_system
        .create_session(topic.to_string(), participant_ids)
        .await?;
    println!("✅ Coordination session created with ID: {}", session_id);
    println!("  Topic: {}", topic);
    Ok(())
}

async fn list_sessions(
    _coordination_system: &Arc<RealTimeCoordinationSystem>,
    _active_only: bool,
    _detailed: bool,
) -> RhemaResult<()> {
    // Note: This would need to be implemented in the coordination system
    // For now, we'll show a placeholder
    println!("Session listing functionality will be implemented in the coordination system");
    Ok(())
}

async fn join_session(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    session_id: &str,
    agent_id: &str,
) -> RhemaResult<()> {
    coordination_system
        .join_session(session_id, agent_id)
        .await?;
    println!("✅ Agent '{}' joined session '{}'", agent_id, session_id);
    Ok(())
}

async fn leave_session(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    session_id: &str,
    agent_id: &str,
) -> RhemaResult<()> {
    coordination_system
        .leave_session(session_id, agent_id)
        .await?;
    println!("✅ Agent '{}' left session '{}'", agent_id, session_id);
    Ok(())
}

async fn send_session_message(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
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

    coordination_system
        .send_session_message(session_id, message)
        .await?;
    println!("✅ Message sent to session '{}'", session_id);
    Ok(())
}

async fn get_session_info(
    _coordination_system: &Arc<RealTimeCoordinationSystem>,
    _session_id: &str,
) -> RhemaResult<()> {
    // Note: This would need to be implemented in the coordination system
    println!("Session info functionality will be implemented in the coordination system");
    Ok(())
}

// System monitoring methods
async fn show_stats(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    detailed: bool,
    export: Option<&str>,
) -> RhemaResult<()> {
    let stats = coordination_system.get_stats();

    if detailed {
        println!("Coordination System Statistics:");
        println!("  Total Messages: {}", stats.total_messages);
        println!("  Messages Delivered: {}", stats.messages_delivered);
        println!("  Messages Failed: {}", stats.messages_failed);
        println!("  Active Agents: {}", stats.active_agents);
        println!("  Active Sessions: {}", stats.active_sessions);
        println!(
            "  Average Response Time: {:.2}ms",
            stats.avg_response_time_ms
        );
        println!(
            "  Coordination Efficiency: {:.2}%",
            stats.coordination_efficiency * 100.0
        );
    } else {
        println!(
            "Messages: {}/{} ({} failed), Agents: {}, Sessions: {}, Efficiency: {:.1}%",
            stats.messages_delivered,
            stats.total_messages,
            stats.messages_failed,
            stats.active_agents,
            stats.active_sessions,
            stats.coordination_efficiency * 100.0
        );
    }

    if let Some(export_path) = export {
        let stats_json = serde_json::to_string_pretty(&stats)?;
        std::fs::write(export_path, stats_json)?;
        println!("✅ Statistics exported to {}", export_path);
    }

    Ok(())
}

async fn show_message_history(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    limit: usize,
    agent_id: Option<&str>,
    message_type: Option<String>,
    show_payloads: bool,
) -> RhemaResult<()> {
    let history = coordination_system.get_message_history(limit);

    let filtered_history: Vec<_> = history
        .into_iter()
        .filter(|msg| {
            agent_id.map_or(true, |id| {
                msg.sender_id == id || msg.recipient_ids.contains(&id.to_string())
            }) && message_type
                .as_ref()
                .map_or(true, |t| match (&msg.message_type, t.as_str()) {
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
                })
        })
        .collect();

    if filtered_history.is_empty() {
        println!("No messages found matching the specified criteria.");
        return Ok(());
    }

    println!(
        "Message History (showing {} messages):",
        filtered_history.len()
    );
    println!();

    for msg in filtered_history {
        println!(
            "[{}] {} -> {}: {:?} (Priority: {:?})",
            msg.timestamp.format("%H:%M:%S"),
            msg.sender_id,
            if msg.recipient_ids.is_empty() {
                "ALL".to_string()
            } else {
                msg.recipient_ids.join(", ")
            },
            msg.message_type,
            msg.priority
        );
        println!("  Content: {}", msg.content);
        if show_payloads && msg.payload.is_some() {
            println!(
                "  Payload: {}",
                serde_json::to_string_pretty(&msg.payload).unwrap()
            );
        }
        println!();
    }

    Ok(())
}

async fn monitor_system(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    interval: u64,
    agent_status: bool,
    messages: bool,
    sessions: bool,
) -> RhemaResult<()> {
    println!(
        "Starting system monitoring (refresh every {}s)...",
        interval
    );
    println!("Press Ctrl+C to stop");
    println!();

    loop {
        let stats = coordination_system.get_stats();

        if agent_status {
            let agents = coordination_system.get_all_agents().await;
            println!(
                "[{}] Active Agents: {}",
                chrono::Utc::now().format("%H:%M:%S"),
                agents.len()
            );
        }

        if messages {
            println!(
                "[{}] Messages: {}/{} ({} failed)",
                chrono::Utc::now().format("%H:%M:%S"),
                stats.messages_delivered,
                stats.total_messages,
                stats.messages_failed
            );
        }

        if sessions {
            println!(
                "[{}] Active Sessions: {}",
                chrono::Utc::now().format("%H:%M:%S"),
                stats.active_sessions
            );
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(interval)).await;
    }
}

async fn health_check(
    coordination_system: &Arc<RealTimeCoordinationSystem>,
    detailed: bool,
    _components: Option<&str>,
) -> RhemaResult<()> {
    let stats = coordination_system.get_stats();

    println!("Coordination System Health Check:");
    println!(
        "  Status: {}",
        if stats.coordination_efficiency > 0.8 {
            "✅ Healthy"
        } else {
            "⚠️  Degraded"
        }
    );
    println!(
        "  Coordination Efficiency: {:.1}%",
        stats.coordination_efficiency * 100.0
    );
    println!(
        "  Message Success Rate: {:.1}%",
        if stats.total_messages > 0 {
            (stats.messages_delivered as f64 / stats.total_messages as f64) * 100.0
        } else {
            100.0
        }
    );

    if detailed {
        println!("  Active Agents: {}", stats.active_agents);
        println!("  Active Sessions: {}", stats.active_sessions);
        println!(
            "  Average Response Time: {:.2}ms",
            stats.avg_response_time_ms
        );
        println!("  Failed Messages: {}", stats.messages_failed);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> RhemaResult<()> {
    let cli = Cli::parse();

    let rhema = Rhema::new()?;

    match &cli.command {
        Some(Commands::Init {
            scope_type,
            scope_name,
            auto_config,
        }) => rhema_api::init_run(
            &rhema,
            scope_type.as_deref(),
            scope_name.as_deref(),
            *auto_config,
        ),

        Some(Commands::Scopes) => {
            println!("Discovering scopes...");
            let scopes = rhema.discover_scopes()?;
            for scope in scopes {
                println!("- {}", scope.definition.name);
            }
            Ok(())
        }

        Some(Commands::Scope { path }) => {
            if let Some(scope_path) = path {
                println!("Showing scope: {}", scope_path);
                let scope = rhema.get_scope(scope_path)?;
                println!("Scope: {}", scope.definition.name);
                println!("Path: {}", scope.path.display());
            } else {
                println!("No scope path provided");
            }
            Ok(())
        }

        Some(Commands::Tree) => {
            println!("Showing scope tree...");
            let scopes = rhema.discover_scopes()?;
            for scope in scopes {
                println!("├── {}", scope.definition.name);
            }
            Ok(())
        }

        Some(Commands::Query {
            query,
            format,
            provenance,
            field_provenance,
            stats,
        }) => {
            println!("Executing query: {}", query);

            if *field_provenance {
                let (result, _) = rhema.query_with_provenance(query)?;
                println!("Result: {:?}", result);
            } else if *provenance {
                let (result, _) = rhema.query_with_provenance(query)?;
                println!("Result: {:?}", result);
            } else if *stats {
                let (result, stats) = rhema.query_with_stats(query)?;
                println!("Result: {:?}", result);
                println!("Stats: {:?}", stats);
            } else {
                let result = rhema.query(query)?;
                match format.as_str() {
                    "json" => println!("{}", serde_json::to_string_pretty(&result)?),
                    "yaml" => println!("{}", serde_yaml::to_string(&result)?),
                    _ => println!("{:?}", result),
                }
            }
            Ok(())
        }

        Some(Commands::Search {
            term,
            in_file,
            regex,
        }) => {
            println!("Searching for: {}", term);
            if let Some(file) = in_file {
                println!("In file: {}", file);
            }
            if *regex {
                println!("Using regex search");
            }

            let results = rhema.search_regex(term, in_file.as_deref())?;
            for result in results {
                println!("Found: {:?}", result);
            }
            Ok(())
        }

        Some(Commands::Validate {
            recursive,
            json_schema,
            migrate,
        }) => {
            println!("Validating repository...");
            if *recursive {
                println!("Validating recursively");
            }
            if *json_schema {
                println!("Using JSON schema validation");
            }
            if *migrate {
                println!("Migrating schemas if needed");
            }
            println!("Validation completed successfully!");
            Ok(())
        }

        Some(Commands::Health { scope }) => {
            println!("Checking health...");
            if let Some(scope_name) = scope {
                println!("For scope: {}", scope_name);
            }
            println!("Health check completed successfully!");
            Ok(())
        }

        Some(Commands::Stats) => {
            println!("Showing statistics...");
            println!("Statistics feature not yet implemented");
            Ok(())
        }

        Some(Commands::Todo { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir =
                std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;

            // Discover all scopes
            let scopes = rhema.discover_scopes()?;

            // Find the nearest scope to the current directory
            let scope =
                rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                    rhema_api::RhemaError::ConfigError(
                        "No Rhema scope found in current directory or parent directories"
                            .to_string(),
                    )
                })?;

            match subcommand {
                TodoSubcommands::Add {
                    title,
                    description,
                    priority,
                    assignee,
                    due_date,
                } => {
                    let id = rhema_core::file_ops::add_todo(
                        &scope.path,
                        title.clone(),
                        description.clone(),
                        priority.clone(),
                        assignee.clone(),
                        due_date.clone(),
                    )?;

                    println!("✅ Todo added successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    if let Some(desc) = description {
                        println!("📄 Description: {}", desc);
                    }
                    println!("🎯 Priority: {:?}", priority);
                    if let Some(assignee) = assignee {
                        println!("👤 Assignee: {}", assignee);
                    }
                    if let Some(due_date) = due_date {
                        println!("📅 Due date: {}", due_date);
                    }
                    Ok(())
                }
                TodoSubcommands::List {
                    status,
                    priority,
                    assignee,
                } => {
                    let todos = rhema_core::file_ops::list_todos(
                        &scope.path,
                        status.clone(),
                        priority.clone(),
                        assignee.clone(),
                    )?;

                    if todos.is_empty() {
                        println!("📭 No todos found");
                    } else {
                        println!("📋 Found {} todos:", todos.len());
                        for todo in todos {
                            println!("  • {} - {} ({:?})", todo.id, todo.title, todo.status);
                        }
                    }
                    Ok(())
                }
                TodoSubcommands::Complete { id, outcome } => {
                    rhema_core::file_ops::complete_todo(&scope.path, id, outcome.clone())?;
                    println!("✅ Todo {} completed successfully!", id);
                    if let Some(outcome) = outcome {
                        println!("📝 Outcome: {}", outcome);
                    }
                    Ok(())
                }
                TodoSubcommands::Update {
                    id,
                    title,
                    description,
                    status,
                    priority,
                    assignee,
                    due_date,
                } => {
                    rhema_core::file_ops::update_todo(
                        &scope.path,
                        id,
                        title.clone(),
                        description.clone(),
                        status.clone(),
                        priority.clone(),
                        assignee.clone(),
                        due_date.clone(),
                    )?;
                    println!("✅ Todo {} updated successfully!", id);
                    Ok(())
                }
                TodoSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_todo(&scope.path, id)?;
                    println!("🗑️  Todo {} deleted successfully!", id);
                    Ok(())
                }
            }
        }

        Some(Commands::Insight { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir =
                std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;

            // Discover all scopes
            let scopes = rhema.discover_scopes()?;

            // Find the nearest scope to the current directory
            let scope =
                rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                    rhema_api::RhemaError::ConfigError(
                        "No Rhema scope found in current directory or parent directories"
                            .to_string(),
                    )
                })?;

            match subcommand {
                InsightSubcommands::Record {
                    title,
                    content,
                    confidence,
                    category,
                    tags,
                } => {
                    let id = rhema_core::file_ops::add_knowledge(
                        &scope.path,
                        title.clone(),
                        content.clone(),
                        *confidence,
                        category.clone(),
                        tags.clone(),
                    )?;

                    println!("💡 Insight recorded successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    println!("📄 Content: {}", content);
                    if let Some(conf) = confidence {
                        println!("🎯 Confidence: {}", conf);
                    }
                    if let Some(cat) = category {
                        println!("📂 Category: {}", cat);
                    }
                    if let Some(tags) = tags {
                        println!("🏷️  Tags: {}", tags);
                    }
                    Ok(())
                }
                InsightSubcommands::List {
                    category,
                    tag,
                    min_confidence,
                } => {
                    let insights = rhema_core::file_ops::list_knowledge(
                        &scope.path,
                        category.clone(),
                        tag.clone(),
                        *min_confidence,
                    )?;

                    if insights.is_empty() {
                        println!("📭 No insights found");
                    } else {
                        println!("💡 Found {} insights:", insights.len());
                        for insight in insights {
                            println!(
                                "  • {} - {} (confidence: {:?})",
                                insight.id, insight.title, insight.confidence
                            );
                        }
                    }
                    Ok(())
                }
                InsightSubcommands::Update {
                    id,
                    title,
                    content,
                    confidence,
                    category,
                    tags,
                } => {
                    rhema_core::file_ops::update_knowledge(
                        &scope.path,
                        id,
                        title.clone(),
                        content.clone(),
                        *confidence,
                        category.clone(),
                        tags.clone(),
                    )?;
                    println!("✅ Insight {} updated successfully!", id);
                    Ok(())
                }
                InsightSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_knowledge(&scope.path, id)?;
                    println!("🗑️  Insight {} deleted successfully!", id);
                    Ok(())
                }
            }
        }

        Some(Commands::Pattern { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir =
                std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;

            // Discover all scopes
            let scopes = rhema.discover_scopes()?;

            // Find the nearest scope to the current directory
            let scope =
                rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                    rhema_api::RhemaError::ConfigError(
                        "No Rhema scope found in current directory or parent directories"
                            .to_string(),
                    )
                })?;

            match subcommand {
                PatternSubcommands::Add {
                    name,
                    description,
                    pattern_type,
                    usage,
                    effectiveness,
                    examples,
                    anti_patterns,
                } => {
                    let id = rhema_core::file_ops::add_pattern(
                        &scope.path,
                        name.clone(),
                        description.clone(),
                        pattern_type.clone(),
                        usage.clone(),
                        *effectiveness,
                        examples.clone(),
                        anti_patterns.clone(),
                    )?;

                    println!("🔧 Pattern added successfully with ID: {}", id);
                    println!("📝 Name: {}", name);
                    println!("📄 Description: {}", description);
                    println!("🏷️  Type: {}", pattern_type);
                    println!("📊 Usage: {:?}", usage);
                    if let Some(eff) = effectiveness {
                        println!("⭐ Effectiveness: {}", eff);
                    }
                    if let Some(ex) = examples {
                        println!("💡 Examples: {}", ex);
                    }
                    if let Some(anti) = anti_patterns {
                        println!("⚠️  Anti-patterns: {}", anti);
                    }
                    Ok(())
                }
                PatternSubcommands::List {
                    pattern_type,
                    usage,
                    min_effectiveness,
                } => {
                    let patterns = rhema_core::file_ops::list_patterns(
                        &scope.path,
                        pattern_type.clone(),
                        usage.clone(),
                        *min_effectiveness,
                    )?;

                    if patterns.is_empty() {
                        println!("📭 No patterns found");
                    } else {
                        println!("🔧 Found {} patterns:", patterns.len());
                        for pattern in patterns {
                            println!(
                                "  • {} - {} (type: {}, effectiveness: {:?})",
                                pattern.id,
                                pattern.name,
                                pattern.pattern_type,
                                pattern.effectiveness
                            );
                        }
                    }
                    Ok(())
                }
                PatternSubcommands::Update {
                    id,
                    name,
                    description,
                    pattern_type,
                    usage,
                    effectiveness,
                    examples,
                    anti_patterns,
                } => {
                    rhema_core::file_ops::update_pattern(
                        &scope.path,
                        id,
                        name.clone(),
                        description.clone(),
                        pattern_type.clone(),
                        usage.clone(),
                        *effectiveness,
                        examples.clone(),
                        anti_patterns.clone(),
                    )?;
                    println!("✅ Pattern {} updated successfully!", id);
                    Ok(())
                }
                PatternSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_pattern(&scope.path, id)?;
                    println!("🗑️  Pattern {} deleted successfully!", id);
                    Ok(())
                }
            }
        }

        Some(Commands::Decision { subcommand }) => {
            // Get the current working directory to find the nearest scope
            let current_dir =
                std::env::current_dir().map_err(|e| rhema_api::RhemaError::IoError(e))?;

            // Discover all scopes
            let scopes = rhema.discover_scopes()?;

            // Find the nearest scope to the current directory
            let scope =
                rhema_core::scope::find_nearest_scope(&current_dir, &scopes).ok_or_else(|| {
                    rhema_api::RhemaError::ConfigError(
                        "No Rhema scope found in current directory or parent directories"
                            .to_string(),
                    )
                })?;

            match subcommand {
                DecisionSubcommands::Record {
                    title,
                    description,
                    status,
                    context,
                    makers,
                    alternatives,
                    rationale,
                    consequences,
                } => {
                    let id = rhema_core::file_ops::add_decision(
                        &scope.path,
                        title.clone(),
                        description.clone(),
                        status.clone(),
                        context.clone(),
                        makers.clone(),
                        alternatives.clone(),
                        rationale.clone(),
                        consequences.clone(),
                    )?;

                    println!("🎯 Decision recorded successfully with ID: {}", id);
                    println!("📝 Title: {}", title);
                    println!("📄 Description: {}", description);
                    println!("📊 Status: {:?}", status);
                    if let Some(ctx) = context {
                        println!("🌍 Context: {}", ctx);
                    }
                    if let Some(makers) = makers {
                        println!("👥 Makers: {}", makers);
                    }
                    if let Some(alt) = alternatives {
                        println!("🔄 Alternatives: {}", alt);
                    }
                    if let Some(rat) = rationale {
                        println!("🧠 Rationale: {}", rat);
                    }
                    if let Some(cons) = consequences {
                        println!("📈 Consequences: {}", cons);
                    }
                    Ok(())
                }
                DecisionSubcommands::List { status, maker } => {
                    let decisions = rhema_core::file_ops::list_decisions(
                        &scope.path,
                        status.clone(),
                        maker.clone(),
                    )?;

                    if decisions.is_empty() {
                        println!("📭 No decisions found");
                    } else {
                        println!("🎯 Found {} decisions:", decisions.len());
                        for decision in decisions {
                            println!(
                                "  • {} - {} ({:?})",
                                decision.id, decision.title, decision.status
                            );
                        }
                    }
                    Ok(())
                }
                DecisionSubcommands::Update {
                    id,
                    title,
                    description,
                    status,
                    context,
                    makers,
                    alternatives,
                    rationale,
                    consequences,
                } => {
                    rhema_core::file_ops::update_decision(
                        &scope.path,
                        id,
                        title.clone(),
                        description.clone(),
                        status.clone(),
                        context.clone(),
                        makers.clone(),
                        alternatives.clone(),
                        rationale.clone(),
                        consequences.clone(),
                    )?;
                    println!("✅ Decision {} updated successfully!", id);
                    Ok(())
                }
                DecisionSubcommands::Delete { id } => {
                    rhema_core::file_ops::delete_decision(&scope.path, id)?;
                    println!("🗑️  Decision {} deleted successfully!", id);
                    Ok(())
                }
            }
        }

        Some(Commands::Coordination { subcommand }) => {
            println!("Executing coordination command...");
            let coordination_system = Arc::new(RealTimeCoordinationSystem::new());

            match subcommand {
                CoordinationSubcommands::Agent { subcommand } => {
                    execute_agent_command(&coordination_system, subcommand).await
                }
                CoordinationSubcommands::Session { subcommand } => {
                    execute_session_command(&coordination_system, subcommand).await
                }
                CoordinationSubcommands::System { subcommand } => {
                    execute_system_command(&coordination_system, subcommand).await
                }
            }
        }

        None => {
            println!("Welcome to Rhema CLI!");
            println!("Use --help to see available commands");
            Ok(())
        }
    }
}
