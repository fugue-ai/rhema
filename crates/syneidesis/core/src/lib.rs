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

//! # Syneidesis Core Library
//!
//! Core types, traits, and utilities for the Syneidesis coordination ecosystem.
//! This library provides the foundational building blocks used by all other
//! coordination crates.
//!
//! ## Features
//!
//! - **Shared Types**: Common data structures used across the ecosystem
//! - **Error Types**: Core error definitions and conversions
//! - **Traits**: Common interfaces and abstractions
//! - **Utilities**: Helper functions and macros
//! - **Constants**: Shared constants and configuration defaults
//!
//! ## Quick Start
//!
//! ```rust
//! use syneidesis_core::{
//!     AgentId, TaskId, AgentStatus, AgentHealth, TaskPriority, TaskStatus
//! };
//!
//! // Create agent and task identifiers
//! let agent_id = AgentId::new("agent-1");
//! let task_id = TaskId::new("task-1");
//!
//! // Use shared enums
//! let status = AgentStatus::Idle;
//! let health = AgentHealth::Healthy;
//! let priority = TaskPriority::Normal;
//! ```

pub mod constants;
pub mod error;
pub mod traits;
pub mod types;
pub mod utils;

// Re-export main types for easy access
pub use constants::*;
pub use error::{CoreError, CoreResult};
pub use traits::{Identifiable, Stateful, Validatable};
pub use types::{
    AgentCapability, AgentEvent, AgentHealth, AgentId, AgentMetadata, AgentMetrics, AgentStatus,
    ConflictId, EventType, SessionId, Task, TaskId, TaskPriority, TaskStatus,
};
pub use utils::{generate_id, timestamp_now, validate_id};

/// Initialize the core library
///
/// This function sets up any global state or configuration needed by the core library.
/// It should be called once at the start of your application.
pub async fn init() -> Result<(), CoreError> {
    // For now, this is a no-op, but could be used for global initialization
    // in the future (e.g., setting up global logging, configuration, etc.)
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() {
        let result = init().await;
        assert!(result.is_ok());
    }
}
