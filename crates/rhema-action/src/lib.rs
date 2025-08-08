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

//! Rhema Action Protocol
//!
//! The Action Protocol provides a safe, validated layer for translating AI agent intent
//! into controlled codebase modifications. This crate extends Rhema from a "map" layer
//! to include a comprehensive "action" layer with safety controls, validation pipelines,
//! and human oversight.

pub mod cli;
pub mod error;
pub mod git;
pub mod pipeline;
pub mod rollback;
pub mod schema;
pub mod tools;
pub mod validation;

// Re-export shared types
pub use rhema_action_tool::{
    ActionError, ActionIntent, ActionResult, SafetyLevel, SafetyTool, ToolResult,
    TransformationTool, ValidationTool,
};

// Re-export internal types
pub use error::ActionError as LocalActionError;
pub use schema::{ActionIntent as ActionConfig, ActionType, ApprovalWorkflow as ActionContext};
pub use tools::ToolRegistry;

use anyhow::Result;

/// Main entry point for the Action Protocol
pub struct ActionProtocol;

impl ActionProtocol {
    /// Initialize the Action Protocol
    pub async fn initialize() -> Result<()> {
        tracing::info!("Initializing Rhema Action Protocol");

        // Initialize components
        tools::ToolRegistry::initialize().await?;

        tracing::info!("Rhema Action Protocol initialized successfully");
        Ok(())
    }

    /// Shutdown the Action Protocol
    pub async fn shutdown() -> Result<()> {
        tracing::info!("Shutting down Rhema Action Protocol");

        // Cleanup components
        tools::ToolRegistry::shutdown().await?;

        tracing::info!("Rhema Action Protocol shutdown successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_action_protocol_initialization() {
        let result = ActionProtocol::initialize().await;
        assert!(result.is_ok());

        let result = ActionProtocol::shutdown().await;
        assert!(result.is_ok());
    }
}
