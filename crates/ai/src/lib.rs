pub mod agent;
pub mod ai_service;
pub mod context_injection;

pub use agent::*;
pub use ai_service::*;
pub use context_injection::*;

// Error type conversions
impl From<agent::state::AgentError> for rhema_core::RhemaError {
    fn from(err: agent::state::AgentError) -> Self {
        rhema_core::RhemaError::AgentError(err.to_string())
    }
}

impl From<agent::locks::LockError> for rhema_core::RhemaError {
    fn from(err: agent::locks::LockError) -> Self {
        rhema_core::RhemaError::AgentError(err.to_string())
    }
}

impl From<agent::coordination::SyncError> for rhema_core::RhemaError {
    fn from(err: agent::coordination::SyncError) -> Self {
        rhema_core::RhemaError::AgentError(err.to_string())
    }
}
