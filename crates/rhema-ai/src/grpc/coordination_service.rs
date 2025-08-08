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

use std::sync::Arc;
use tokio::sync::RwLock;
// use futures::stream::{self, Stream, StreamExt};
// use tokio_stream::wrappers::ReceiverStream;

use crate::agent::real_time_coordination::RealTimeCoordinationSystem;

// Temporarily comment out the generated protobuf code until we fix the dependencies
/*
use crate::grpc::coordination::{
    AgentInfo as ProtoAgentInfo, AgentMessage as ProtoAgentMessage, AgentStatus as ProtoAgentStatus,
    CoordinationSession as ProtoCoordinationSession, CoordinationStats as ProtoCoordinationStats,
    CreateSessionRequest, CreateSessionResponse, GetAgentInfoRequest, GetAgentInfoResponse,
    GetSessionInfoRequest, GetSessionInfoResponse, JoinSessionRequest, JoinSessionResponse,
    LeaveSessionRequest, LeaveSessionResponse, MessagePriority as ProtoMessagePriority,
    MessageType as ProtoMessageType, RegisterAgentRequest, RegisterAgentResponse,
    SendMessageRequest, SendMessageResponse, SendSessionMessageRequest, SendSessionMessageResponse, SessionStatus as ProtoSessionStatus,
    UnregisterAgentRequest, UnregisterAgentResponse,
};
*/

/// gRPC coordination service implementation
pub struct CoordinationService {
    coordination_system: Arc<RwLock<RealTimeCoordinationSystem>>,
}

impl CoordinationService {
    pub fn new(coordination_system: RealTimeCoordinationSystem) -> Self {
        Self {
            coordination_system: Arc::new(RwLock::new(coordination_system)),
        }
    }
}

// Temporarily comment out the service implementation until we fix the dependencies
/*
#[tonic::async_trait]
impl coordination_server::Coordination for CoordinationService {
    async fn register_agent(
        &self,
        request: Request<RegisterAgentRequest>,
    ) -> Result<Response<RegisterAgentResponse>, Status> {
        let proto_agent = request.into_inner().agent_info.ok_or_else(|| {
            Status::invalid_argument("Agent info is required")
        })?;

        let agent_info = AgentInfo {
            id: proto_agent.id,
            name: proto_agent.name,
            agent_type: proto_agent.agent_type,
            status: match proto_agent.status() {
                ProtoAgentStatus::AgentStatusIdle => AgentStatus::Idle,
                ProtoAgentStatus::AgentStatusBusy => AgentStatus::Busy,
                ProtoAgentStatus::AgentStatusWorking => AgentStatus::Working,
                ProtoAgentStatus::AgentStatusBlocked => AgentStatus::Blocked,
                ProtoAgentStatus::AgentStatusCollaborating => AgentStatus::Collaborating,
                ProtoAgentStatus::AgentStatusOffline => AgentStatus::Offline,
            },
            current_task_id: proto_agent.current_task_id,
            assigned_scope: proto_agent.assigned_scope,
            capabilities: proto_agent.capabilities,
            last_heartbeat: chrono::Utc::now(),
            is_online: proto_agent.is_online,
            performance_metrics: crate::agent::real_time_coordination::AgentPerformanceMetrics::default(),
        };

        match self.coordination_system.write().await.register_agent(agent_info).await {
            Ok(_) => Ok(Response::new(RegisterAgentResponse {
                success: true,
                message: "Agent registered successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(RegisterAgentResponse {
                success: false,
                message: format!("Failed to register agent: {}", e),
            })),
        }
    }

    async fn unregister_agent(
        &self,
        request: Request<UnregisterAgentRequest>,
    ) -> Result<Response<UnregisterAgentResponse>, Status> {
        let agent_id = request.into_inner().agent_id;

        match self.coordination_system.write().await.unregister_agent(&agent_id).await {
            Ok(_) => Ok(Response::new(UnregisterAgentResponse {
                success: true,
                message: "Agent unregistered successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(UnregisterAgentResponse {
                success: false,
                message: format!("Failed to unregister agent: {}", e),
            })),
        }
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<SendMessageResponse>, Status> {
        let proto_message = request.into_inner().message.ok_or_else(|| {
            Status::invalid_argument("Message is required")
        })?;

        let message = AgentMessage {
            id: proto_message.id,
            message_type: match proto_message.message_type() {
                ProtoMessageType::MessageTypeTaskAssignment => MessageType::TaskAssignment,
                ProtoMessageType::MessageTypeTaskCompletion => MessageType::TaskCompletion,
                ProtoMessageType::MessageTypeTaskBlocked => MessageType::TaskBlocked,
                ProtoMessageType::MessageTypeResourceRequest => MessageType::ResourceRequest,
                ProtoMessageType::MessageTypeResourceRelease => MessageType::ResourceRelease,
                ProtoMessageType::MessageTypeConflictNotification => MessageType::ConflictNotification,
                ProtoMessageType::MessageTypeCoordinationRequest => MessageType::CoordinationRequest,
                ProtoMessageType::MessageTypeStatusUpdate => MessageType::StatusUpdate,
                ProtoMessageType::MessageTypeKnowledgeShare => MessageType::KnowledgeShare,
                ProtoMessageType::MessageTypeDecisionRequest => MessageType::DecisionRequest,
                ProtoMessageType::MessageTypeDecisionResponse => MessageType::DecisionResponse,
                ProtoMessageType::MessageTypeCustom => MessageType::Custom(proto_message.content.clone()),
            },
            priority: match proto_message.priority() {
                ProtoMessagePriority::MessagePriorityLow => MessagePriority::Low,
                ProtoMessagePriority::MessagePriorityNormal => MessagePriority::Normal,
                ProtoMessagePriority::MessagePriorityHigh => MessagePriority::High,
                ProtoMessagePriority::MessagePriorityCritical => MessagePriority::Critical,
                ProtoMessagePriority::MessagePriorityEmergency => MessagePriority::Emergency,
            },
            sender_id: proto_message.sender_id,
            recipient_ids: proto_message.recipient_ids,
            content: proto_message.content,
            payload: proto_message.payload.as_ref().map(|p| serde_json::from_str(&p.type_url).unwrap_or_default()),
            timestamp: chrono::Utc::now(),
            requires_ack: proto_message.requires_ack,
            expires_at: proto_message.expires_at.as_ref().map(|dt| chrono::DateTime::from_timestamp(dt.seconds, dt.nanos as u32).unwrap_or_else(|| chrono::Utc::now())),
            metadata: proto_message.metadata,
        };

        match self.coordination_system.write().await.send_message(message).await {
            Ok(_) => Ok(Response::new(SendMessageResponse {
                success: true,
                message: "Message sent successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(SendMessageResponse {
                success: false,
                message: format!("Failed to send message: {}", e),
            })),
        }
    }

    async fn get_agent_info(
        &self,
        request: Request<GetAgentInfoRequest>,
    ) -> Result<Response<GetAgentInfoResponse>, Status> {
        let agent_id = request.into_inner().agent_id;

        match self.coordination_system.read().await.get_agent_info(&agent_id) {
            Some(agent_info) => {
                let proto_agent = ProtoAgentInfo {
                    id: agent_info.id,
                    name: agent_info.name,
                    agent_type: agent_info.agent_type,
                    status: match agent_info.status {
                        AgentStatus::Idle => ProtoAgentStatus::AgentStatusIdle,
                        AgentStatus::Busy => ProtoAgentStatus::AgentStatusBusy,
                        AgentStatus::Working => ProtoAgentStatus::AgentStatusWorking,
                        AgentStatus::Blocked => ProtoAgentStatus::AgentStatusBlocked,
                        AgentStatus::Collaborating => ProtoAgentStatus::AgentStatusCollaborating,
                        AgentStatus::Offline => ProtoAgentStatus::AgentStatusOffline,
                    },
                    current_task_id: agent_info.current_task_id,
                    assigned_scope: agent_info.assigned_scope,
                    capabilities: agent_info.capabilities,
                    last_heartbeat: Some(prost_types::Timestamp::from(agent_info.last_heartbeat)),
                    is_online: agent_info.is_online,
                    performance_metrics: None,
                    priority: 1,
                    version: "1.0.0".to_string(),
                    endpoint: None,
                    metadata: std::collections::HashMap::new(),
                    created_at: None,
                    last_updated: None,
                };

                Ok(Response::new(GetAgentInfoResponse {
                    agent_info: Some(proto_agent),
                }))
            }
            None => Err(Status::not_found("Agent not found")),
        }
    }

    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<CreateSessionResponse>, Status> {
        let req = request.into_inner();
        let topic = req.topic;
        let participants = req.participants;

        match self.coordination_system.write().await.create_session(topic, participants).await {
            Ok(session_id) => Ok(Response::new(CreateSessionResponse {
                session_id,
                success: true,
                message: "Session created successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(CreateSessionResponse {
                session_id: "".to_string(),
                success: false,
                message: format!("Failed to create session: {}", e),
            })),
        }
    }

    async fn join_session(
        &self,
        request: Request<JoinSessionRequest>,
    ) -> Result<Response<JoinSessionResponse>, Status> {
        let req = request.into_inner();
        let session_id = req.session_id;
        let agent_id = req.agent_id;

        match self.coordination_system.write().await.join_session(&session_id, &agent_id).await {
            Ok(_) => Ok(Response::new(JoinSessionResponse {
                success: true,
                message: "Joined session successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(JoinSessionResponse {
                success: false,
                message: format!("Failed to join session: {}", e),
            })),
        }
    }

    async fn leave_session(
        &self,
        request: Request<LeaveSessionRequest>,
    ) -> Result<Response<LeaveSessionResponse>, Status> {
        let req = request.into_inner();
        let session_id = req.session_id;
        let agent_id = req.agent_id;

        match self.coordination_system.write().await.leave_session(&session_id, &agent_id).await {
            Ok(_) => Ok(Response::new(LeaveSessionResponse {
                success: true,
                message: "Left session successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(LeaveSessionResponse {
                success: false,
                message: format!("Failed to leave session: {}", e),
            })),
        }
    }

    async fn send_session_message(
        &self,
        request: Request<SendSessionMessageRequest>,
    ) -> Result<Response<SendSessionMessageResponse>, Status> {
        let proto_message = request.into_inner().message.ok_or_else(|| {
            Status::invalid_argument("Message is required")
        })?;

        let message = AgentMessage {
            id: proto_message.id,
            message_type: match proto_message.message_type() {
                ProtoMessageType::MessageTypeTaskAssignment => MessageType::TaskAssignment,
                ProtoMessageType::MessageTypeTaskCompletion => MessageType::TaskCompletion,
                ProtoMessageType::MessageTypeTaskBlocked => MessageType::TaskBlocked,
                ProtoMessageType::MessageTypeResourceRequest => MessageType::ResourceRequest,
                ProtoMessageType::MessageTypeResourceRelease => MessageType::ResourceRelease,
                ProtoMessageType::MessageTypeConflictNotification => MessageType::ConflictNotification,
                ProtoMessageType::MessageTypeCoordinationRequest => MessageType::CoordinationRequest,
                ProtoMessageType::MessageTypeStatusUpdate => MessageType::StatusUpdate,
                ProtoMessageType::MessageTypeKnowledgeShare => MessageType::KnowledgeShare,
                ProtoMessageType::MessageTypeDecisionRequest => MessageType::DecisionRequest,
                ProtoMessageType::MessageTypeDecisionResponse => MessageType::DecisionResponse,
                ProtoMessageType::MessageTypeCustom => MessageType::Custom(proto_message.content.clone()),
            },
            priority: match proto_message.priority() {
                ProtoMessagePriority::MessagePriorityLow => MessagePriority::Low,
                ProtoMessagePriority::MessagePriorityNormal => MessagePriority::Normal,
                ProtoMessagePriority::MessagePriorityHigh => MessagePriority::High,
                ProtoMessagePriority::MessagePriorityCritical => MessagePriority::Critical,
                ProtoMessagePriority::MessagePriorityEmergency => MessagePriority::Emergency,
            },
            sender_id: proto_message.sender_id,
            recipient_ids: proto_message.recipient_ids,
            content: proto_message.content,
            payload: proto_message.payload.as_ref().map(|p| serde_json::from_str(&p.type_url).unwrap_or_default()),
            timestamp: chrono::Utc::now(),
            requires_ack: proto_message.requires_ack,
            expires_at: proto_message.expires_at.as_ref().map(|dt| chrono::DateTime::from_timestamp(dt.seconds, dt.nanos as u32).unwrap_or_else(|| chrono::Utc::now())),
            metadata: proto_message.metadata,
        };

        let session_id = request.into_inner().session_id;

        match self.coordination_system.write().await.send_session_message(&session_id, message).await {
            Ok(_) => Ok(Response::new(SendSessionMessageResponse {
                success: true,
                message: "Session message sent successfully".to_string(),
            })),
            Err(e) => Ok(Response::new(SendSessionMessageResponse {
                success: false,
                message: format!("Failed to send session message: {}", e),
            })),
        }
    }

    async fn get_session_info(
        &self,
        request: Request<GetSessionInfoRequest>,
    ) -> Result<Response<GetSessionInfoResponse>, Status> {
        let session_id = request.into_inner().session_id;

        // For now, return a placeholder response
        Ok(Response::new(GetSessionInfoResponse {
            session_info: None,
        }))
    }
}
*/
