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

//! gRPC services for real-time agent coordination
//!
//! This crate provides gRPC-based communication services for agent coordination,
//! including server implementation, client utilities, and protocol definitions.

pub mod client;
pub mod server;
pub mod service;
pub mod types;

// Include the generated protobuf code
pub mod coordination {
    tonic::include_proto!("syneidesis.coordination.v1");
}

// Re-export the generated protobuf types
pub use coordination::*;

// Re-export commonly used types
pub use client::CoordinationClient;
pub use server::CoordinationServer;

// Re-export configuration types from syneidesis-config
pub use syneidesis_config::types::{GrpcClientConfig, GrpcConfig};

/// Default gRPC server port
pub const DEFAULT_GRPC_PORT: u16 = 50051;

/// Default gRPC server address
pub const DEFAULT_GRPC_ADDR: &str = "127.0.0.1:50051";
