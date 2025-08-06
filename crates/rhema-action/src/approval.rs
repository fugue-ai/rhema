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

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::{ActionIntent, SafetyLevel};
use crate::error::{ActionError, ActionResult};

/// Approval request
#[derive(Debug, Clone)]
pub struct ApprovalRequest {
    pub id: String,
    pub intent_id: String,
    pub requested_by: String,
    pub requested_at: DateTime<Utc>,
    pub approvers: Vec<String>,
    pub status: ApprovalStatus,
    pub comments: Vec<ApprovalComment>,
    pub expires_at: DateTime<Utc>,
}

/// Approval status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Cancelled,
}

/// Approval comment
#[derive(Debug, Clone)]
pub struct ApprovalComment {
    pub id: String,
    pub author: String,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub is_decision: bool,
}

/// Approval policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalPolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub safety_levels: Vec<SafetyLevel>,
    pub required_approvers: usize,
    pub timeout_seconds: u64,
    pub auto_approve: bool,
    pub conditions: Vec<ApprovalCondition>,
}

/// Approval condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
    pub description: String,
}

/// Enhanced approval request
#[derive(Debug, Clone)]
pub struct EnhancedApprovalRequest {
    pub id: String,
    pub intent_id: String,
    pub requested_by: String,
    pub requested_at: DateTime<Utc>,
    pub approvers: Vec<Approver>,
    pub status: ApprovalStatus,
    pub comments: Vec<ApprovalComment>,
    pub expires_at: DateTime<Utc>,
    pub policy_id: Option<String>,
    pub safety_level: SafetyLevel,
    pub auto_approved: bool,
    pub approval_history: Vec<ApprovalEvent>,
}

/// Approver information
#[derive(Debug, Clone)]
pub struct Approver {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub status: ApproverStatus,
    pub responded_at: Option<DateTime<Utc>>,
    pub response: Option<ApproverResponse>,
}

/// Approver status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApproverStatus {
    Pending,
    Approved,
    Rejected,
    Notified,
}

/// Approver response
#[derive(Debug, Clone)]
pub struct ApproverResponse {
    pub decision: ApprovalStatus,
    pub comment: String,
    pub timestamp: DateTime<Utc>,
}

/// Approval event
#[derive(Debug, Clone)]
pub struct ApprovalEvent {
    pub event_type: ApprovalEventType,
    pub actor: String,
    pub timestamp: DateTime<Utc>,
    pub details: String,
}

/// Approval event types
#[derive(Debug, Clone)]
pub enum ApprovalEventType {
    RequestCreated,
    ApproverNotified,
    ApprovalGranted,
    ApprovalRejected,
    RequestExpired,
    RequestCancelled,
    CommentAdded,
}

/// Approval workflow manager
pub struct ApprovalWorkflow {
    requests: Arc<RwLock<HashMap<String, ApprovalRequest>>>,
    notification_channels: Vec<String>,
    default_timeout: u64, // seconds
}

impl ApprovalWorkflow {
    /// Create a new approval workflow
    pub async fn new() -> ActionResult<Self> {
        info!("Initializing Approval Workflow");
        
        let workflow = Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
            notification_channels: vec!["console".to_string(), "email".to_string()],
            default_timeout: 3600, // 1 hour
        };
        
        info!("Approval Workflow initialized successfully");
        Ok(workflow)
    }

    /// Initialize the approval workflow (stub)
    pub async fn initialize() -> ActionResult<()> {
        info!("ApprovalWorkflow initialized (stub)");
        Ok(())
    }

    /// Shutdown the approval workflow (stub)
    pub async fn shutdown() -> ActionResult<()> {
        info!("ApprovalWorkflow shutdown (stub)");
        Ok(())
    }
    
    /// Request approval for an action intent
    pub async fn request_approval(&self, intent: &ActionIntent) -> ActionResult<bool> {
        info!("Requesting approval for intent: {}", intent.id);
        
        let request_id = Uuid::new_v4().simple().to_string();
        let expires_at = Utc::now() + chrono::Duration::seconds(intent.approval_workflow.timeout as i64);
        
        let approvers = intent.approval_workflow.approvers.clone().unwrap_or_default();
        if approvers.is_empty() {
            warn!("No approvers specified for intent: {}", intent.id);
            return Ok(false);
        }
        
        let request = ApprovalRequest {
            id: request_id.clone(),
            intent_id: intent.id.clone(),
            requested_by: intent.created_by.clone().unwrap_or_else(|| "system".to_string()),
            requested_at: Utc::now(),
            approvers: approvers.clone(),
            status: ApprovalStatus::Pending,
            comments: Vec::new(),
            expires_at,
        };
        
        // Store the request
        {
            let mut requests = self.requests.write().await;
            requests.insert(request_id.clone(), request.clone());
        }
        
        // Send notifications
        self.send_approval_notifications(&request, &intent).await?;
        
        // For now, simulate approval process
        // In a real implementation, this would wait for human input
        let approved = self.simulate_approval_process(&request).await?;
        
        // Update request status
        {
            let mut requests = self.requests.write().await;
            if let Some(existing_request) = requests.get_mut(&request_id) {
                existing_request.status = if approved {
                    ApprovalStatus::Approved
                } else {
                    ApprovalStatus::Rejected
                };
            }
        }
        
        if approved {
            info!("Approval granted for intent: {}", intent.id);
        } else {
            info!("Approval denied for intent: {}", intent.id);
        }
        
        Ok(approved)
    }
    
    /// Simulate approval process (placeholder for real implementation)
    async fn simulate_approval_process(&self, request: &ApprovalRequest) -> ActionResult<bool> {
        info!("Simulating approval process for request: {}", request.id);
        
        // For now, auto-approve low-risk actions and reject high-risk ones
        // In a real implementation, this would present a UI or wait for external input
        
        // Simulate some processing time
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Simple logic: approve if there are multiple approvers (indicating lower risk)
        let approved = request.approvers.len() > 1;
        
        Ok(approved)
    }
    
    /// Send approval notifications
    async fn send_approval_notifications(&self, request: &ApprovalRequest, intent: &ActionIntent) -> ActionResult<()> {
        info!("Sending approval notifications for request: {}", request.id);
        
        for channel in &self.notification_channels {
            match channel.as_str() {
                "console" => {
                    self.send_console_notification(request, intent).await?;
                }
                "email" => {
                    self.send_email_notification(request, intent).await?;
                }
                _ => {
                    warn!("Unknown notification channel: {}", channel);
                }
            }
        }
        
        Ok(())
    }
    
    /// Send console notification
    async fn send_console_notification(&self, request: &ApprovalRequest, intent: &ActionIntent) -> ActionResult<()> {
        info!("=== APPROVAL REQUEST ===");
        info!("Request ID: {}", request.id);
        info!("Intent ID: {}", request.intent_id);
        info!("Description: {}", intent.description);
        info!("Requested by: {}", request.requested_by);
        info!("Approvers: {}", request.approvers.join(", "));
        info!("Expires at: {}", request.expires_at);
        info!("========================");
        
        Ok(())
    }
    
    /// Send email notification
    async fn send_email_notification(&self, request: &ApprovalRequest, _intent: &ActionIntent) -> ActionResult<()> {
        // TODO: Implement email notification
        info!("Email notification would be sent for request: {}", request.id);
        Ok(())
    }
    
    /// Approve a request
    pub async fn approve_request(&self, request_id: &str, approver: &str, comment: Option<&str>) -> ActionResult<()> {
        info!("Approving request: {} by {}", request_id, approver);
        
        let mut requests = self.requests.write().await;
        
        if let Some(request) = requests.get_mut(request_id) {
            // Check if the approver is authorized
            if !request.approvers.contains(&approver.to_string()) {
                return Err(ActionError::approval(format!(
                    "User {} is not authorized to approve this request",
                    approver
                )));
            }
            
            // Check if request is still pending
            if request.status != ApprovalStatus::Pending {
                return Err(ActionError::approval(format!(
                    "Request is not pending (status: {:?})",
                    request.status
                )));
            }
            
            // Check if request has expired
            if Utc::now() > request.expires_at {
                request.status = ApprovalStatus::Expired;
                return Err(ActionError::approval("Request has expired"));
            }
            
            // Update status
            request.status = ApprovalStatus::Approved;
            
            // Add approval comment
            if let Some(comment_text) = comment {
                let comment = ApprovalComment {
                    id: Uuid::new_v4().simple().to_string(),
                    author: approver.to_string(),
                    content: comment_text.to_string(),
                    timestamp: Utc::now(),
                    is_decision: true,
                };
                request.comments.push(comment);
            }
            
            info!("Request approved successfully: {}", request_id);
        } else {
            return Err(ActionError::approval(format!("Request not found: {}", request_id)));
        }
        
        Ok(())
    }
    
    /// Reject a request
    pub async fn reject_request(&self, request_id: &str, approver: &str, reason: &str) -> ActionResult<()> {
        info!("Rejecting request: {} by {}", request_id, approver);
        
        let mut requests = self.requests.write().await;
        
        if let Some(request) = requests.get_mut(request_id) {
            // Check if the approver is authorized
            if !request.approvers.contains(&approver.to_string()) {
                return Err(ActionError::approval(format!(
                    "User {} is not authorized to reject this request",
                    approver
                )));
            }
            
            // Check if request is still pending
            if request.status != ApprovalStatus::Pending {
                return Err(ActionError::approval(format!(
                    "Request is not pending (status: {:?})",
                    request.status
                )));
            }
            
            // Check if request has expired
            if Utc::now() > request.expires_at {
                request.status = ApprovalStatus::Expired;
                return Err(ActionError::approval("Request has expired"));
            }
            
            // Update status
            request.status = ApprovalStatus::Rejected;
            
            // Add rejection comment
            let comment = ApprovalComment {
                id: Uuid::new_v4().simple().to_string(),
                author: approver.to_string(),
                content: reason.to_string(),
                timestamp: Utc::now(),
                is_decision: true,
            };
            request.comments.push(comment);
            
            info!("Request rejected successfully: {}", request_id);
        } else {
            return Err(ActionError::approval(format!("Request not found: {}", request_id)));
        }
        
        Ok(())
    }
    
    /// Add a comment to a request
    pub async fn add_comment(&self, request_id: &str, author: &str, content: &str) -> ActionResult<()> {
        info!("Adding comment to request: {} by {}", request_id, author);
        
        let mut requests = self.requests.write().await;
        
        if let Some(request) = requests.get_mut(request_id) {
            let comment = ApprovalComment {
                id: Uuid::new_v4().simple().to_string(),
                author: author.to_string(),
                content: content.to_string(),
                timestamp: Utc::now(),
                is_decision: false,
            };
            request.comments.push(comment);
            
            info!("Comment added successfully to request: {}", request_id);
        } else {
            return Err(ActionError::approval(format!("Request not found: {}", request_id)));
        }
        
        Ok(())
    }
    
    /// Get approval request by ID
    pub async fn get_request(&self, request_id: &str) -> Option<ApprovalRequest> {
        let requests = self.requests.read().await;
        requests.get(request_id).cloned()
    }
    
    /// List all approval requests
    pub async fn list_requests(&self) -> Vec<ApprovalRequest> {
        let requests = self.requests.read().await;
        requests.values().cloned().collect()
    }
    
    /// List pending approval requests
    pub async fn list_pending_requests(&self) -> Vec<ApprovalRequest> {
        let requests = self.requests.read().await;
        requests
            .values()
            .filter(|request| request.status == ApprovalStatus::Pending)
            .cloned()
            .collect()
    }
    
    /// List requests for an intent
    pub async fn list_requests_for_intent(&self, intent_id: &str) -> Vec<ApprovalRequest> {
        let requests = self.requests.read().await;
        requests
            .values()
            .filter(|request| request.intent_id == intent_id)
            .cloned()
            .collect()
    }
    
    /// Cancel a request
    pub async fn cancel_request(&self, request_id: &str, cancelled_by: &str) -> ActionResult<()> {
        info!("Cancelling request: {} by {}", request_id, cancelled_by);
        
        let mut requests = self.requests.write().await;
        
        if let Some(request) = requests.get_mut(request_id) {
            // Check if request is still pending
            if request.status != ApprovalStatus::Pending {
                return Err(ActionError::approval(format!(
                    "Request is not pending (status: {:?})",
                    request.status
                )));
            }
            
            // Update status
            request.status = ApprovalStatus::Cancelled;
            
            // Add cancellation comment
            let comment = ApprovalComment {
                id: Uuid::new_v4().simple().to_string(),
                author: cancelled_by.to_string(),
                content: "Request cancelled".to_string(),
                timestamp: Utc::now(),
                is_decision: true,
            };
            request.comments.push(comment);
            
            info!("Request cancelled successfully: {}", request_id);
        } else {
            return Err(ActionError::approval(format!("Request not found: {}", request_id)));
        }
        
        Ok(())
    }
    
    /// Clean up expired requests
    pub async fn cleanup_expired_requests(&self) -> ActionResult<usize> {
        info!("Cleaning up expired approval requests");
        
        let mut requests = self.requests.write().await;
        let mut expired_count = 0;
        
        let now = Utc::now();
        let expired_requests: Vec<String> = requests
            .iter()
            .filter(|(_, request)| {
                request.status == ApprovalStatus::Pending && now > request.expires_at
            })
            .map(|(id, _)| id.clone())
            .collect();
        
        for request_id in expired_requests {
            if let Some(request) = requests.get_mut(&request_id) {
                request.status = ApprovalStatus::Expired;
                expired_count += 1;
            }
        }
        
        info!("Cleaned up {} expired requests", expired_count);
        Ok(expired_count)
    }
    
    /// Get approval statistics
    pub async fn get_approval_stats(&self) -> ApprovalStats {
        let requests = self.requests.read().await;
        
        let total_requests = requests.len();
        let pending_requests = requests.values().filter(|r| r.status == ApprovalStatus::Pending).count();
        let approved_requests = requests.values().filter(|r| r.status == ApprovalStatus::Approved).count();
        let rejected_requests = requests.values().filter(|r| r.status == ApprovalStatus::Rejected).count();
        let expired_requests = requests.values().filter(|r| r.status == ApprovalStatus::Expired).count();
        let cancelled_requests = requests.values().filter(|r| r.status == ApprovalStatus::Cancelled).count();
        
        ApprovalStats {
            total_requests,
            pending_requests,
            approved_requests,
            rejected_requests,
            expired_requests,
            cancelled_requests,
        }
    }

    /// Create an enhanced approval request with policy-based approval
    pub async fn create_enhanced_approval_request(&self, intent: &ActionIntent, policy: &ApprovalPolicy) -> ActionResult<EnhancedApprovalRequest> {
        info!("Creating enhanced approval request for intent: {}", intent.id);
        
        let request_id = Uuid::new_v4().simple().to_string();
        let expires_at = Utc::now() + chrono::Duration::seconds(policy.timeout_seconds as i64);
        
        // Create approvers based on policy
        let mut approvers = Vec::new();
        for i in 0..policy.required_approvers {
            approvers.push(Approver {
                id: Uuid::new_v4().simple().to_string(),
                name: format!("Approver {}", i + 1),
                email: format!("approver{}@example.com", i + 1),
                role: "Reviewer".to_string(),
                status: ApproverStatus::Pending,
                responded_at: None,
                response: None,
            });
        }
        
        // Check if auto-approval applies
        let auto_approved = policy.auto_approve && self.check_auto_approval_conditions(intent, policy).await?;
        
        let status = if auto_approved {
            ApprovalStatus::Approved
        } else {
            ApprovalStatus::Pending
        };
        
        let mut approval_history = vec![
            ApprovalEvent {
                event_type: ApprovalEventType::RequestCreated,
                actor: "system".to_string(),
                timestamp: Utc::now(),
                details: format!("Created approval request for intent: {}", intent.id),
            }
        ];
        
        if auto_approved {
            approval_history.push(ApprovalEvent {
                event_type: ApprovalEventType::ApprovalGranted,
                actor: "system".to_string(),
                timestamp: Utc::now(),
                details: "Auto-approved based on policy".to_string(),
            });
        }
        
        let enhanced_request = EnhancedApprovalRequest {
            id: request_id,
            intent_id: intent.id.clone(),
            requested_by: "system".to_string(),
            requested_at: Utc::now(),
            approvers,
            status,
            comments: vec![],
            expires_at,
            policy_id: Some(policy.id.clone()),
            safety_level: intent.safety_level.clone(),
            auto_approved,
            approval_history,
        };
        
        // Store the request
        let mut requests = self.requests.write().await;
        requests.insert(enhanced_request.id.clone(), ApprovalRequest {
            id: enhanced_request.id.clone(),
            intent_id: enhanced_request.intent_id.clone(),
            requested_by: enhanced_request.requested_by.clone(),
            requested_at: enhanced_request.requested_at,
            approvers: enhanced_request.approvers.iter().map(|a| a.name.clone()).collect(),
            status: enhanced_request.status.clone(),
            comments: enhanced_request.comments.clone(),
            expires_at: enhanced_request.expires_at,
        });
        
        // Send notifications if not auto-approved
        if !auto_approved {
            self.send_enhanced_notifications(&enhanced_request, intent).await?;
        }
        
        Ok(enhanced_request)
    }

    /// Check auto-approval conditions
    async fn check_auto_approval_conditions(&self, intent: &ActionIntent, policy: &ApprovalPolicy) -> ActionResult<bool> {
        for condition in &policy.conditions {
            let condition_met = match condition.field.as_str() {
                "safety_level" => {
                    let intent_level = format!("{:?}", intent.safety_level).to_lowercase();
                    match condition.operator.as_str() {
                        "equals" => intent_level == condition.value.to_lowercase(),
                        "not_equals" => intent_level != condition.value.to_lowercase(),
                        _ => false,
                    }
                }
                "action_type" => {
                    let intent_type = format!("{:?}", intent.action_type).to_lowercase();
                    match condition.operator.as_str() {
                        "equals" => intent_type == condition.value.to_lowercase(),
                        "not_equals" => intent_type != condition.value.to_lowercase(),
                        _ => false,
                    }
                }
                "description" => {
                    match condition.operator.as_str() {
                        "contains" => intent.description.contains(&condition.value),
                        "not_contains" => !intent.description.contains(&condition.value),
                        _ => false,
                    }
                }
                _ => false,
            };
            
            if !condition_met {
                return Ok(false);
            }
        }
        
        Ok(true)
    }

    /// Send enhanced notifications
    async fn send_enhanced_notifications(&self, request: &EnhancedApprovalRequest, intent: &ActionIntent) -> ActionResult<()> {
        info!("Sending enhanced notifications for approval request: {}", request.id);
        
        for approver in &request.approvers {
            // Send notification to each approver
            self.send_approver_notification(approver, request, intent).await?;
            
            // Update approver status
            // In a real implementation, this would be done after successful notification
        }
        
        Ok(())
    }

    /// Send notification to a specific approver
    async fn send_approver_notification(&self, approver: &Approver, request: &EnhancedApprovalRequest, intent: &ActionIntent) -> ActionResult<()> {
        info!("Sending notification to approver: {} ({})", approver.name, approver.email);
        
        // In a real implementation, this would send actual notifications
        // For now, we'll just log the notification
        
        let notification_message = format!(
            "Approval Request: {}\nIntent: {}\nAction: {:?}\nSafety Level: {:?}\nExpires: {}\n\nPlease review and approve/reject this action.",
            request.id,
            intent.description,
            intent.action_type,
            intent.safety_level,
            request.expires_at.format("%Y-%m-%d %H:%M:%S UTC")
        );
        
        info!("Notification sent to {}: {}", approver.email, notification_message);
        
        Ok(())
    }

    /// Get approval policies (stub implementation)
    pub async fn get_default_policies(&self) -> Vec<ApprovalPolicy> {
        vec![
            ApprovalPolicy {
                id: "high_safety_policy".to_string(),
                name: "High Safety Level Policy".to_string(),
                description: "Requires approval for high safety level actions".to_string(),
                enabled: true,
                safety_levels: vec![SafetyLevel::High],
                required_approvers: 2,
                timeout_seconds: 7200, // 2 hours
                auto_approve: false,
                conditions: vec![
                    ApprovalCondition {
                        field: "safety_level".to_string(),
                        operator: "equals".to_string(),
                        value: "high".to_string(),
                        description: "High safety level actions".to_string(),
                    }
                ],
            },
            ApprovalPolicy {
                id: "low_safety_policy".to_string(),
                name: "Low Safety Level Policy".to_string(),
                description: "Auto-approves low safety level actions".to_string(),
                enabled: true,
                safety_levels: vec![SafetyLevel::Low],
                required_approvers: 0,
                timeout_seconds: 3600, // 1 hour
                auto_approve: true,
                conditions: vec![
                    ApprovalCondition {
                        field: "safety_level".to_string(),
                        operator: "equals".to_string(),
                        value: "low".to_string(),
                        description: "Low safety level actions".to_string(),
                    }
                ],
            },
        ]
    }
}

/// Approval statistics
#[derive(Debug, Clone)]
pub struct ApprovalStats {
    pub total_requests: usize,
    pub pending_requests: usize,
    pub approved_requests: usize,
    pub rejected_requests: usize,
    pub expired_requests: usize,
    pub cancelled_requests: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::{ActionType, SafetyLevel};

    #[tokio::test]
    async fn test_approval_workflow_creation() {
        let workflow = ApprovalWorkflow::new().await;
        assert!(workflow.is_ok());
    }

    #[tokio::test]
    async fn test_approval_request() {
        let workflow = ApprovalWorkflow::new().await.unwrap();
        
        let mut intent = ActionIntent::new(
            "test-approval",
            ActionType::Refactor,
            "Test approval request",
            vec!["src/".to_string()],
            SafetyLevel::High,
        );
        
        intent.add_approver("user1");
        intent.add_approver("user2");
        
        let approved = workflow.request_approval(&intent).await;
        assert!(approved.is_ok());
        
        // Should be approved since there are multiple approvers
        let approved = approved.unwrap();
        assert!(approved);
    }

    #[tokio::test]
    async fn test_approval_request_single_approver() {
        let workflow = ApprovalWorkflow::new().await.unwrap();
        
        let mut intent = ActionIntent::new(
            "test-single-approver",
            ActionType::Refactor,
            "Test single approver",
            vec!["src/".to_string()],
            SafetyLevel::High,
        );
        
        intent.add_approver("user1");
        
        let approved = workflow.request_approval(&intent).await;
        assert!(approved.is_ok());
        
        // Should be rejected since there's only one approver
        let approved = approved.unwrap();
        assert!(!approved);
    }

    #[tokio::test]
    async fn test_approval_request_no_approvers() {
        let workflow = ApprovalWorkflow::new().await.unwrap();
        
        let intent = ActionIntent::new(
            "test-no-approvers",
            ActionType::Refactor,
            "Test no approvers",
            vec!["src/".to_string()],
            SafetyLevel::High,
        );
        
        let approved = workflow.request_approval(&intent).await;
        assert!(approved.is_ok());
        
        // Should be rejected since there are no approvers
        let approved = approved.unwrap();
        assert!(!approved);
    }

    #[tokio::test]
    async fn test_approval_stats() {
        let workflow = ApprovalWorkflow::new().await.unwrap();
        
        let mut intent = ActionIntent::new(
            "test-stats",
            ActionType::Refactor,
            "Test approval stats",
            vec!["src/".to_string()],
            SafetyLevel::High,
        );
        
        intent.add_approver("user1");
        intent.add_approver("user2");
        
        let _approved = workflow.request_approval(&intent).await.unwrap();
        
        let stats = workflow.get_approval_stats().await;
        assert!(stats.total_requests > 0);
        assert!(stats.approved_requests > 0 || stats.rejected_requests > 0);
    }

    #[tokio::test]
    async fn test_cleanup_expired_requests() {
        let workflow = ApprovalWorkflow::new().await.unwrap();
        
        // Create a request with very short timeout
        let mut intent = ActionIntent::new(
            "test-expired",
            ActionType::Refactor,
            "Test expired request",
            vec!["src/".to_string()],
            SafetyLevel::High,
        );
        
        intent.add_approver("user1");
        intent.approval_workflow.timeout = 1; // 1 second
        
        let _approved = workflow.request_approval(&intent).await.unwrap();
        
        // Wait for expiration
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        let expired_count = workflow.cleanup_expired_requests().await;
        assert!(expired_count.is_ok());
        
        let expired_count = expired_count.unwrap();
        assert!(expired_count > 0);
    }
} 