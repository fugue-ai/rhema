//! Rhema MCP Server Runtime
//! 
//! This crate provides the runtime implementation for the Model Context Protocol (MCP) server
//! for the Rhema Protocol. It wraps the core MCP functionality from the `rhema-mcp` crate
//! and provides a standalone server binary.

pub mod server;

pub use server::RhemaMcpServer;

/// Re-export common types from the core MCP crate
pub use rhema_mcp::*;
