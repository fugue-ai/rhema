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

//! Coordination client integration for Rhema Core
//!
//! This module provides integration between Rhema Core and the coordination client,
//! enabling agents to communicate and coordinate their activities.

pub mod client;
pub mod config;
pub mod manager;
pub mod types;

pub use client::*;
pub use config::*;
pub use manager::*;
pub use types::*;

#[cfg(feature = "coordination")]
// Re-export syneidesis-grpc types for convenience
pub use syneidesis_grpc::{CoordinationClient as SyneidesisClient, GrpcClientConfig};
