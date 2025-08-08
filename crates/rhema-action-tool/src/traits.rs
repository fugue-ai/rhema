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

use crate::{ActionIntent, ActionResult, SafetyLevel, ToolResult};
use async_trait::async_trait;

/// Trait for transformation tools
#[async_trait]
pub trait TransformationTool: Send + Sync {
    /// Execute the tool with the given intent
    async fn execute(&self, intent: &ActionIntent) -> ActionResult<ToolResult>;

    /// Check if the tool supports the given language
    fn supports_language(&self, language: &str) -> bool;

    /// Get the safety level of this tool
    fn safety_level(&self) -> SafetyLevel;

    /// Get the name of this tool
    fn name(&self) -> &str;

    /// Get the version of this tool
    fn version(&self) -> &str;

    /// Check if the tool is available
    async fn is_available(&self) -> bool;
}

/// Trait for validation tools
#[async_trait]
pub trait ValidationTool: Send + Sync {
    /// Run validation with the given intent
    async fn validate(&self, intent: &ActionIntent) -> ActionResult<ToolResult>;

    /// Get the name of this tool
    fn name(&self) -> &str;

    /// Get the version of this tool
    fn version(&self) -> &str;

    /// Check if the tool is available
    async fn is_available(&self) -> bool;
}

/// Trait for safety tools
#[async_trait]
pub trait SafetyTool: Send + Sync {
    /// Run safety check with the given intent
    async fn check(&self, intent: &ActionIntent) -> ActionResult<ToolResult>;

    /// Get the name of this tool
    fn name(&self) -> &str;

    /// Get the version of this tool
    fn version(&self) -> &str;

    /// Check if the tool is available
    async fn is_available(&self) -> bool;
}
