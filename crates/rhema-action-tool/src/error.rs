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

use thiserror::Error;

/// Error types for action operations
#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Tool execution error for {tool}: {message}")]
    ToolExecution { tool: String, message: String },
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for action operations
pub type ActionResult<T> = Result<T, ActionError>; 