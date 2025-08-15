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

//! Type conversion utilities for Rhema ↔ Syneidesis coordination types
//!
//! This module provides conversion functions between Rhema's internal coordination
//! types and Syneidesis protobuf types for gRPC communication.

use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use std::collections::HashMap;

use crate::agent::real_time_coordination::{AgentInfo, AgentMessage, AgentPerformanceMetrics, AgentStatus, MessagePriority, MessageType};

// Use Syneidesis protobuf types
use syneidesis_grpc::coordination::v1::{
    agent_info::AgentHealth as ProtoAgentHealth,
    agent_info::AgentStatus as ProtoAgentStatus,
    agent_message::MessagePriority as ProtoMessagePriority,
    agent_message::MessageType as ProtoMessageType,
    AgentInfo as ProtoAgentInfo,
    AgentMessage as ProtoAgentMessage,
    AgentPerformanceMetrics as ProtoAgentPerformanceMetrics,
};

/// Convert Rhema AgentStatus to Syneidesis protobuf AgentStatus
pub fn rhema_status_to_proto(status: &AgentStatus) -> i32 {
    match status {
        AgentStatus::Idle => ProtoAgentStatus::AgentStatusIdle as i32,
        AgentStatus::Busy => ProtoAgentStatus::AgentStatusBusy as i32,
        AgentStatus::Working => ProtoAgentStatus::AgentStatusWorking as i32,
        AgentStatus::Blocked => ProtoAgentStatus::AgentStatusBlocked as i32,
        AgentStatus::Collaborating => ProtoAgentStatus::AgentStatusCollaborating as i32,
        AgentStatus::Offline => ProtoAgentStatus::AgentStatusOffline as i32,
        AgentStatus::Failed => ProtoAgentStatus::AgentStatusOffline as i32, // Map to offline
    }
}

/// Convert Syneidesis protobuf AgentStatus to Rhema AgentStatus
pub fn proto_status_to_rhema(status: i32) -> AgentStatus {
    match status {
        x if x == ProtoAgentStatus::AgentStatusIdle as i32 => AgentStatus::Idle,
        x if x == ProtoAgentStatus::AgentStatusBusy as i32 => AgentStatus::Busy,
        x if x == ProtoAgentStatus::AgentStatusWorking as i32 => AgentStatus::Working,
        x if x == ProtoAgentStatus::AgentStatusBlocked as i32 => AgentStatus::Blocked,
        x if x == ProtoAgentStatus::AgentStatusCollaborating as i32 => AgentStatus::Collaborating,
        x if x == ProtoAgentStatus::AgentStatusOffline as i32 => AgentStatus::Offline,
        _ => AgentStatus::Idle, // Default to idle for unknown status
    }
}

/// Convert Rhema MessageType to Syneidesis protobuf MessageType
pub fn rhema_message_type_to_proto(message_type: &MessageType) -> i32 {
    match message_type {
        MessageType::TaskAssignment => ProtoMessageType::MessageTypeTaskAssignment as i32,
        MessageType::TaskCompletion => ProtoMessageType::MessageTypeTaskCompletion as i32,
        MessageType::TaskBlocked => ProtoMessageType::MessageTypeTaskBlocked as i32,
        MessageType::ResourceRequest => ProtoMessageType::MessageTypeResourceRequest as i32,
        MessageType::ResourceRelease => ProtoMessageType::MessageTypeResourceRelease as i32,
        MessageType::ConflictNotification => ProtoMessageType::MessageTypeConflictNotification as i32,
        MessageType::CoordinationRequest => ProtoMessageType::MessageTypeCoordinationRequest as i32,
        MessageType::StatusUpdate => ProtoMessageType::MessageTypeStatusUpdate as i32,
        MessageType::KnowledgeShare => ProtoMessageType::MessageTypeKnowledgeShare as i32,
        MessageType::DecisionRequest => ProtoMessageType::MessageTypeDecisionRequest as i32,
        MessageType::DecisionResponse => ProtoMessageType::MessageTypeDecisionResponse as i32,
        MessageType::ConflictDetection => ProtoMessageType::MessageTypeConflictNotification as i32, // Map to conflict notification
        MessageType::ConsensusRequest => ProtoMessageType::MessageTypeCoordinationRequest as i32, // Map to coordination request
        MessageType::NegotiationRequest => ProtoMessageType::MessageTypeCoordinationRequest as i32, // Map to coordination request
        MessageType::SessionMessage => ProtoMessageType::MessageTypeCustom as i32, // Map to custom
        MessageType::Custom(_) => ProtoMessageType::MessageTypeCustom as i32,
    }
}

/// Convert Syneidesis protobuf MessageType to Rhema MessageType
pub fn proto_message_type_to_rhema(message_type: i32) -> MessageType {
    match message_type {
        x if x == ProtoMessageType::MessageTypeTaskAssignment as i32 => MessageType::TaskAssignment,
        x if x == ProtoMessageType::MessageTypeTaskCompletion as i32 => MessageType::TaskCompletion,
        x if x == ProtoMessageType::MessageTypeTaskBlocked as i32 => MessageType::TaskBlocked,
        x if x == ProtoMessageType::MessageTypeResourceRequest as i32 => MessageType::ResourceRequest,
        x if x == ProtoMessageType::MessageTypeResourceRelease as i32 => MessageType::ResourceRelease,
        x if x == ProtoMessageType::MessageTypeConflictNotification as i32 => MessageType::ConflictNotification,
        x if x == ProtoMessageType::MessageTypeCoordinationRequest as i32 => MessageType::CoordinationRequest,
        x if x == ProtoMessageType::MessageTypeStatusUpdate as i32 => MessageType::StatusUpdate,
        x if x == ProtoMessageType::MessageTypeKnowledgeShare as i32 => MessageType::KnowledgeShare,
        x if x == ProtoMessageType::MessageTypeDecisionRequest as i32 => MessageType::DecisionRequest,
        x if x == ProtoMessageType::MessageTypeDecisionResponse as i32 => MessageType::DecisionResponse,
        x if x == ProtoMessageType::MessageTypeCustom as i32 => MessageType::Custom("custom".to_string()),
        _ => MessageType::Custom("unknown".to_string()),
    }
}

/// Convert Rhema MessagePriority to Syneidesis protobuf MessagePriority
pub fn rhema_priority_to_proto(priority: &MessagePriority) -> i32 {
    match priority {
        MessagePriority::Low => ProtoMessagePriority::MessagePriorityLow as i32,
        MessagePriority::Normal => ProtoMessagePriority::MessagePriorityNormal as i32,
        MessagePriority::High => ProtoMessagePriority::MessagePriorityHigh as i32,
        MessagePriority::Critical => ProtoMessagePriority::MessagePriorityCritical as i32,
        MessagePriority::Emergency => ProtoMessagePriority::MessagePriorityEmergency as i32,
    }
}

/// Convert Syneidesis protobuf MessagePriority to Rhema MessagePriority
pub fn proto_priority_to_rhema(priority: i32) -> MessagePriority {
    match priority {
        x if x == ProtoMessagePriority::MessagePriorityLow as i32 => MessagePriority::Low,
        x if x == ProtoMessagePriority::MessagePriorityNormal as i32 => MessagePriority::Normal,
        x if x == ProtoMessagePriority::MessagePriorityHigh as i32 => MessagePriority::High,
        x if x == ProtoMessagePriority::MessagePriorityCritical as i32 => MessagePriority::Critical,
        x if x == ProtoMessagePriority::MessagePriorityEmergency as i32 => MessagePriority::Emergency,
        _ => MessagePriority::Normal, // Default to normal
    }
}

/// Convert DateTime<Utc> to prost_types::Timestamp
pub fn datetime_to_timestamp(dt: &DateTime<Utc>) -> Timestamp {
    Timestamp {
        seconds: dt.timestamp(),
        nanos: dt.timestamp_subsec_nanos() as i32,
    }
}

/// Convert prost_types::Timestamp to DateTime<Utc>
pub fn timestamp_to_datetime(ts: &Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32).unwrap_or_else(|| Utc::now())
}

/// Convert Rhema AgentPerformanceMetrics to Syneidesis protobuf AgentPerformanceMetrics
pub fn rhema_metrics_to_proto(metrics: &AgentPerformanceMetrics) -> ProtoAgentPerformanceMetrics {
    ProtoAgentPerformanceMetrics {
        tasks_completed: metrics.tasks_completed as u32,
        tasks_failed: metrics.tasks_failed as u32,
        avg_completion_time_seconds: metrics.avg_completion_time_seconds,
        success_rate: metrics.success_rate,
        collaboration_score: metrics.collaboration_score,
        avg_response_time_ms: metrics.avg_response_time_ms,
        cpu_usage_percent: 0.0, // Not available in Rhema metrics
        memory_usage_mb: 0.0,   // Not available in Rhema metrics
        active_connections: 0,   // Not available in Rhema metrics
    }
}

/// Convert Syneidesis protobuf AgentPerformanceMetrics to Rhema AgentPerformanceMetrics
pub fn proto_metrics_to_rhema(metrics: &ProtoAgentPerformanceMetrics) -> AgentPerformanceMetrics {
    AgentPerformanceMetrics {
        tasks_completed: metrics.tasks_completed as usize,
        tasks_failed: metrics.tasks_failed as usize,
        avg_completion_time_seconds: metrics.avg_completion_time_seconds,
        success_rate: metrics.success_rate,
        collaboration_score: metrics.collaboration_score,
        avg_response_time_ms: metrics.avg_response_time_ms,
    }
}

/// Convert Rhema AgentInfo to Syneidesis protobuf AgentInfo
pub fn rhema_agent_info_to_proto(agent: &AgentInfo) -> ProtoAgentInfo {
    ProtoAgentInfo {
        id: agent.id.clone(),
        name: agent.name.clone(),
        agent_type: agent.agent_type.clone(),
        status: rhema_status_to_proto(&agent.status),
        health: ProtoAgentHealth::AgentHealthHealthy as i32, // Default to healthy
        current_task_id: agent.current_task_id.clone(),
        assigned_scope: agent.assigned_scope.clone(),
        capabilities: agent.capabilities.clone(),
        last_heartbeat: Some(datetime_to_timestamp(&agent.last_heartbeat)),
        is_online: agent.is_online,
        performance_metrics: Some(rhema_metrics_to_proto(&agent.performance_metrics)),
        priority: 1, // Default priority
        version: "1.0.0".to_string(), // Default version
        endpoint: None,
        metadata: HashMap::new(), // Empty metadata for now
        created_at: Some(datetime_to_timestamp(&agent.last_heartbeat)), // Use last_heartbeat as created_at
        last_updated: Some(datetime_to_timestamp(&agent.last_heartbeat)),
    }
}

/// Convert Syneidesis protobuf AgentInfo to Rhema AgentInfo
pub fn proto_agent_info_to_rhema(agent: &ProtoAgentInfo) -> AgentInfo {
    AgentInfo {
        id: agent.id.clone(),
        name: agent.name.clone(),
        agent_type: agent.agent_type.clone(),
        status: proto_status_to_rhema(agent.status),
        current_task_id: agent.current_task_id.clone(),
        assigned_scope: agent.assigned_scope.clone(),
        capabilities: agent.capabilities.clone(),
        last_heartbeat: agent.last_heartbeat.as_ref().map_or_else(Utc::now, |ts| timestamp_to_datetime(ts)),
        is_online: agent.is_online,
        performance_metrics: agent.performance_metrics.as_ref().map_or_else(AgentPerformanceMetrics::default, proto_metrics_to_rhema),
    }
}

/// Convert Rhema AgentMessage to Syneidesis protobuf AgentMessage
pub fn rhema_message_to_proto(message: &AgentMessage) -> ProtoAgentMessage {
    ProtoAgentMessage {
        id: message.id.clone(),
        message_type: rhema_message_type_to_proto(&message.message_type),
        priority: rhema_priority_to_proto(&message.priority),
        sender_id: message.sender_id.clone(),
        recipient_ids: message.recipient_ids.clone(),
        content: message.content.clone(),
        payload: None, // TODO: Convert payload if needed
        timestamp: Some(datetime_to_timestamp(&message.timestamp)),
        requires_ack: message.requires_ack,
        expires_at: message.expires_at.as_ref().map(datetime_to_timestamp),
        metadata: message.metadata.clone(),
    }
}

/// Convert Syneidesis protobuf AgentMessage to Rhema AgentMessage
pub fn proto_message_to_rhema(message: &ProtoAgentMessage) -> AgentMessage {
    AgentMessage {
        id: message.id.clone(),
        message_type: proto_message_type_to_rhema(message.message_type),
        priority: proto_priority_to_rhema(message.priority),
        sender_id: message.sender_id.clone(),
        recipient_ids: message.recipient_ids.clone(),
        content: message.content.clone(),
        payload: None, // TODO: Convert payload if needed
        timestamp: message.timestamp.as_ref().map_or_else(Utc::now, timestamp_to_datetime),
        requires_ack: message.requires_ack,
        expires_at: message.expires_at.as_ref().map(timestamp_to_datetime),
        metadata: message.metadata.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_status_conversion() {
        let rhema_status = AgentStatus::Busy;
        let proto_status = rhema_status_to_proto(&rhema_status);
        let converted_back = proto_status_to_rhema(proto_status);
        assert_eq!(rhema_status, converted_back);
    }

    #[test]
    fn test_message_type_conversion() {
        let rhema_type = MessageType::TaskAssignment;
        let proto_type = rhema_message_type_to_proto(&rhema_type);
        let converted_back = proto_message_type_to_rhema(proto_type);
        assert!(matches!(converted_back, MessageType::TaskAssignment));
    }

    #[test]
    fn test_priority_conversion() {
        let rhema_priority = MessagePriority::High;
        let proto_priority = rhema_priority_to_proto(&rhema_priority);
        let converted_back = proto_priority_to_rhema(proto_priority);
        assert_eq!(rhema_priority, converted_back);
    }

    #[test]
    fn test_datetime_conversion() {
        let now = Utc::now();
        let timestamp = datetime_to_timestamp(&now);
        let converted_back = timestamp_to_datetime(&timestamp);
        assert_eq!(now.timestamp(), converted_back.timestamp());
    }

    #[test]
    fn test_agent_info_conversion() {
        let rhema_agent = AgentInfo {
            id: "test-agent".to_string(),
            name: "Test Agent".to_string(),
            agent_type: "test".to_string(),
            status: AgentStatus::Idle,
            current_task_id: None,
            assigned_scope: "test-scope".to_string(),
            capabilities: vec!["test".to_string()],
            last_heartbeat: Utc::now(),
            is_online: true,
            performance_metrics: AgentPerformanceMetrics::default(),
        };

        let proto_agent = rhema_agent_info_to_proto(&rhema_agent);
        let converted_back = proto_agent_info_to_rhema(&proto_agent);

        assert_eq!(rhema_agent.id, converted_back.id);
        assert_eq!(rhema_agent.name, converted_back.name);
        assert_eq!(rhema_agent.agent_type, converted_back.agent_type);
        assert_eq!(rhema_agent.status, converted_back.status);
    }

    #[test]
    fn test_message_conversion() {
        let rhema_message = AgentMessage {
            id: "test-message".to_string(),
            message_type: MessageType::TaskAssignment,
            priority: MessagePriority::Normal,
            sender_id: "sender".to_string(),
            recipient_ids: vec!["recipient".to_string()],
            content: "test content".to_string(),
            payload: None,
            timestamp: Utc::now(),
            requires_ack: false,
            expires_at: None,
            metadata: HashMap::new(),
        };

        let proto_message = rhema_message_to_proto(&rhema_message);
        let converted_back = proto_message_to_rhema(&proto_message);

        assert_eq!(rhema_message.id, converted_back.id);
        assert_eq!(rhema_message.sender_id, converted_back.sender_id);
        assert_eq!(rhema_message.content, converted_back.content);
    }
}
