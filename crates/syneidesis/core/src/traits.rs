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

//! Common traits and interfaces for the Syneidesis coordination ecosystem

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::error::CoreResult;
// Note: These types are used in the trait definitions but not directly in this module
// They are imported for documentation and potential future use

/// Trait for entities that have a unique identifier
pub trait Identifiable {
    /// The type of the identifier
    type Id;

    /// Get the unique identifier
    fn id(&self) -> &Self::Id;

    /// Get the identifier as a string
    fn id_str(&self) -> &str;
}

/// Trait for entities that have a state
pub trait Stateful {
    /// The type of the state
    type State;

    /// Get the current state
    fn state(&self) -> &Self::State;

    /// Get the current state as a mutable reference
    fn state_mut(&mut self) -> &mut Self::State;

    /// Set the state
    fn set_state(&mut self, state: Self::State);

    /// Check if the entity is in a specific state
    fn is_in_state(&self, state: &Self::State) -> bool
    where
        Self::State: PartialEq,
    {
        self.state() == state
    }
}

/// Trait for entities that can be validated
pub trait Validatable {
    /// Validate the entity
    fn validate(&self) -> CoreResult<()>;

    /// Check if the entity is valid
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

/// Trait for entities that have timestamps
pub trait Timestamped {
    /// Get the creation timestamp
    fn created_at(&self) -> DateTime<Utc>;

    /// Get the last update timestamp
    fn updated_at(&self) -> DateTime<Utc>;

    /// Update the last update timestamp
    fn touch(&mut self);

    /// Check if the entity was created after a given time
    fn created_after(&self, time: DateTime<Utc>) -> bool {
        self.created_at() > time
    }

    /// Check if the entity was updated after a given time
    fn updated_after(&self, time: DateTime<Utc>) -> bool {
        self.updated_at() > time
    }
}

/// Trait for entities that can be serialized and deserialized
pub trait Serializable: serde::Serialize + serde::de::DeserializeOwned {
    /// Serialize to JSON string
    fn to_json(&self) -> CoreResult<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    /// Deserialize from JSON string
    fn from_json(json: &str) -> CoreResult<Self> {
        serde_json::from_str(json).map_err(Into::into)
    }

    /// Serialize to JSON value
    fn to_json_value(&self) -> CoreResult<serde_json::Value> {
        serde_json::to_value(self).map_err(Into::into)
    }

    /// Deserialize from JSON value
    fn from_json_value(value: serde_json::Value) -> CoreResult<Self> {
        serde_json::from_value(value).map_err(Into::into)
    }
}

/// Trait for entities that can be cloned
pub trait Cloneable: Clone {
    /// Create a deep copy
    fn deep_clone(&self) -> Self {
        self.clone()
    }
}

/// Trait for entities that have metadata
pub trait Metadata {
    /// Get metadata value by key
    fn get_metadata(&self, key: &str) -> Option<&serde_json::Value>;

    /// Set metadata value
    fn set_metadata(&mut self, key: String, value: serde_json::Value);

    /// Remove metadata value
    fn remove_metadata(&mut self, key: &str) -> Option<serde_json::Value>;

    /// Check if metadata contains a key
    fn has_metadata(&self, key: &str) -> bool {
        self.get_metadata(key).is_some()
    }

    /// Get all metadata keys
    fn metadata_keys(&self) -> Vec<String>;
}

/// Trait for entities that can be compared
pub trait Comparable {
    /// Compare this entity with another
    fn compare(&self, other: &Self) -> std::cmp::Ordering;

    /// Check if this entity equals another
    fn equals(&self, other: &Self) -> bool {
        self.compare(other) == std::cmp::Ordering::Equal
    }

    /// Check if this entity is less than another
    fn less_than(&self, other: &Self) -> bool {
        self.compare(other) == std::cmp::Ordering::Less
    }

    /// Check if this entity is greater than another
    fn greater_than(&self, other: &Self) -> bool {
        self.compare(other) == std::cmp::Ordering::Greater
    }
}

/// Trait for entities that can be converted to and from other types
pub trait Convertible<T> {
    /// Convert to the target type
    fn convert_to(&self) -> CoreResult<T>;

    /// Convert from the source type
    fn convert_from(source: T) -> CoreResult<Self>
    where
        Self: Sized;
}

/// Trait for entities that can be initialized
#[async_trait]
pub trait Initializable {
    /// Initialize the entity
    async fn initialize(&mut self) -> CoreResult<()>;

    /// Check if the entity is initialized
    fn is_initialized(&self) -> bool;

    /// Reset the entity to uninitialized state
    fn reset(&mut self);
}

/// Trait for entities that can be started and stopped
#[async_trait]
pub trait Lifecycle {
    /// Start the entity
    async fn start(&mut self) -> CoreResult<()>;

    /// Stop the entity
    async fn stop(&mut self) -> CoreResult<()>;

    /// Check if the entity is running
    fn is_running(&self) -> bool;

    /// Restart the entity
    async fn restart(&mut self) -> CoreResult<()> {
        self.stop().await?;
        self.start().await
    }
}

/// Trait for entities that can be monitored
pub trait Monitorable {
    /// Get health status
    fn health(&self) -> HealthStatus;

    /// Get performance metrics
    fn metrics(&self) -> Option<serde_json::Value>;

    /// Get status information
    fn status(&self) -> StatusInfo;
}

/// Health status enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// Entity is healthy
    Healthy,
    /// Entity is degraded but functional
    Degraded,
    /// Entity is unhealthy
    Unhealthy,
    /// Entity health is unknown
    Unknown,
}

/// Status information
#[derive(Debug, Clone)]
pub struct StatusInfo {
    /// Status message
    pub message: String,
    /// Status code
    pub code: u32,
    /// Additional status data
    pub data: serde_json::Value,
}

impl Default for StatusInfo {
    fn default() -> Self {
        Self {
            message: "OK".to_string(),
            code: 0,
            data: serde_json::Value::Null,
        }
    }
}

/// Trait for entities that can be persisted
#[async_trait]
pub trait Persistable {
    /// Save the entity
    async fn save(&self) -> CoreResult<()>;

    /// Load the entity
    async fn load(&mut self) -> CoreResult<()>;

    /// Delete the entity
    async fn delete(&self) -> CoreResult<()>;

    /// Check if the entity exists
    async fn exists(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AgentId;

    // Test implementation for Identifiable
    struct TestEntity {
        id: AgentId,
        name: String,
    }

    impl Identifiable for TestEntity {
        type Id = AgentId;

        fn id(&self) -> &Self::Id {
            &self.id
        }

        fn id_str(&self) -> &str {
            self.id.as_str()
        }
    }

    #[test]
    fn test_identifiable() {
        let entity = TestEntity {
            id: AgentId::new("test-agent"),
            name: "Test".to_string(),
        };

        assert_eq!(entity.id_str(), "test-agent");
    }

    // Test implementation for Stateful
    #[derive(Debug, Clone, PartialEq)]
    enum TestState {
        Idle,
        Busy,
    }

    struct StatefulEntity {
        state: TestState,
    }

    impl Stateful for StatefulEntity {
        type State = TestState;

        fn state(&self) -> &Self::State {
            &self.state
        }

        fn state_mut(&mut self) -> &mut Self::State {
            &mut self.state
        }

        fn set_state(&mut self, state: Self::State) {
            self.state = state;
        }
    }

    #[test]
    fn test_stateful() {
        let mut entity = StatefulEntity {
            state: TestState::Idle,
        };

        assert!(entity.is_in_state(&TestState::Idle));
        entity.set_state(TestState::Busy);
        assert!(entity.is_in_state(&TestState::Busy));
    }

    // Test implementation for Validatable
    struct ValidatableEntity {
        name: String,
    }

    impl Validatable for ValidatableEntity {
        fn validate(&self) -> CoreResult<()> {
            if self.name.is_empty() {
                return Err(crate::error::CoreError::validation("Name cannot be empty"));
            }
            Ok(())
        }
    }

    #[test]
    fn test_validatable() {
        let valid_entity = ValidatableEntity {
            name: "Test".to_string(),
        };
        assert!(valid_entity.is_valid());

        let invalid_entity = ValidatableEntity {
            name: "".to_string(),
        };
        assert!(!invalid_entity.is_valid());
    }
}
