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

use rhema_action::schema::SafetyLevel;
use rhema_action::{ActionConfig, ActionType, ApprovalCondition, ApprovalPolicy, ApprovalWorkflow};
use serde_json::Value;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt::init();

    println!("=== Email Notification Example ===\n");

    // Create an approval workflow
    let workflow = ApprovalWorkflow::new().await?;
    println!("✓ Approval workflow created\n");

    // Create an action intent that requires approval
    let mut intent = ActionConfig::new(
        "email-notification-demo",
        ActionType::Security,
        "Update security policies for production environment",
        vec![
            "src/security/".to_string(),
            "config/security.yaml".to_string(),
        ],
        SafetyLevel::High,
    );

    // Add approvers (these would be real email addresses in production)
    intent.approval_workflow.approvers = Some(vec![
        "security-team@company.com".to_string(),
        "devops-lead@company.com".to_string(),
        "cto@company.com".to_string(),
    ]);

    // Add metadata for the email body
    intent.metadata = Some(HashMap::from([
        (
            "priority".to_string(),
            Value::String("critical".to_string()),
        ),
        (
            "environment".to_string(),
            Value::String("production".to_string()),
        ),
        ("risk_level".to_string(), Value::String("high".to_string())),
        (
            "estimated_impact".to_string(),
            Value::String("affects all users".to_string()),
        ),
    ]));

    println!("✓ Action intent created:");
    println!("  - ID: {}", intent.id);
    println!("  - Type: {:?}", intent.action_type);
    println!("  - Description: {}", intent.description);
    println!("  - Safety Level: {:?}", intent.safety_level);
    println!(
        "  - Approvers: {:?}",
        intent
            .approval_workflow
            .approvers
            .as_ref()
            .unwrap_or(&vec![])
    );
    println!();

    // Request approval (this will trigger email notifications)
    println!("Requesting approval...");
    let approved = workflow.request_approval(&intent).await?;

    if approved {
        println!("✓ Approval granted!");
    } else {
        println!("✗ Approval denied!");
    }
    println!();

    // Demonstrate enhanced approval with policy
    println!("=== Enhanced Approval with Policy ===\n");

    let policies = workflow.get_default_policies().await;
    let high_safety_policy = policies
        .iter()
        .find(|p| p.id == "high_safety_policy")
        .unwrap();

    println!("✓ Using policy: {}", high_safety_policy.name);
    println!("  - Description: {}", high_safety_policy.description);
    println!(
        "  - Required approvers: {}",
        high_safety_policy.required_approvers
    );
    println!(
        "  - Timeout: {} seconds",
        high_safety_policy.timeout_seconds
    );
    println!("  - Auto-approve: {}", high_safety_policy.auto_approve);
    println!();

    // Create enhanced approval request
    let enhanced_request = workflow
        .create_enhanced_approval_request(&intent, high_safety_policy)
        .await?;

    println!("✓ Enhanced approval request created:");
    println!("  - Request ID: {}", enhanced_request.id);
    println!("  - Status: {:?}", enhanced_request.status);
    println!("  - Auto-approved: {}", enhanced_request.auto_approved);
    println!("  - Approvers: {}", enhanced_request.approvers.len());
    println!();

    // Show approval statistics
    let stats = workflow.get_approval_stats().await;
    println!("=== Approval Statistics ===");
    println!("  - Total requests: {}", stats.total_requests);
    println!("  - Pending: {}", stats.pending_requests);
    println!("  - Approved: {}", stats.approved_requests);
    println!("  - Rejected: {}", stats.rejected_requests);
    println!("  - Expired: {}", stats.expired_requests);
    println!("  - Cancelled: {}", stats.cancelled_requests);
    println!();

    println!("=== Email Notification Summary ===");
    println!("✓ Email notifications were sent to all approvers");
    println!("✓ Email content included:");
    println!("  - Request details and metadata");
    println!("  - Approval instructions");
    println!("  - Response format guidelines");
    println!("  - Expiration information");
    println!();

    println!("Note: In this example, email sending is simulated.");
    println!("In production, this would use a real email service like:");
    println!("  - SMTP with libraries like lettre");
    println!("  - Email service APIs (SendGrid, AWS SES, etc.)");
    println!("  - Internal notification systems");

    Ok(())
}
