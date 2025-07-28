use crate::{Gacp, GacpResult, DecisionSubcommands, DecisionStatus};
use crate::file_ops;
use crate::scope::find_nearest_scope;
use colored::*;

pub fn run(gacp: &Gacp, subcommand: &DecisionSubcommands) -> GacpResult<()> {
    // Get the current working directory to find the nearest scope
    let current_dir = std::env::current_dir()
        .map_err(|e| crate::GacpError::IoError(e))?;
    
    // Discover all scopes
    let scopes = gacp.discover_scopes()?;
    
    // Find the nearest scope to the current directory
    let scope = find_nearest_scope(&current_dir, &scopes)
        .ok_or_else(|| crate::GacpError::ConfigError("No GACP scope found in current directory or parent directories".to_string()))?;
    
    match subcommand {
        DecisionSubcommands::Record { title, description, status, context, makers, alternatives, rationale, consequences } => {
            record_decision(scope, title, description, status, context, makers, alternatives, rationale, consequences)
        }
        DecisionSubcommands::List { status, maker } => {
            list_decisions(scope, status, maker)
        }
        DecisionSubcommands::Update { id, title, description, status, context, makers, alternatives, rationale, consequences } => {
            update_decision(scope, id, title, description, status, context, makers, alternatives, rationale, consequences)
        }
        DecisionSubcommands::Delete { id } => {
            delete_decision(scope, id)
        }
    }
}

fn record_decision(
    scope: &crate::Scope,
    title: &str,
    description: &str,
    status: &DecisionStatus,
    context: &Option<String>,
    makers: &Option<String>,
    alternatives: &Option<String>,
    rationale: &Option<String>,
    consequences: &Option<String>,
) -> GacpResult<()> {
    let id = file_ops::add_decision(
        &scope.path,
        title.to_string(),
        description.to_string(),
        status.clone(),
        context.clone(),
        makers.clone(),
        alternatives.clone(),
        rationale.clone(),
        consequences.clone(),
    )?;
    
    println!("🎯 Decision recorded successfully with ID: {}", id.green());
    println!("📝 Title: {}", title);
    println!("📄 Description: {}", description);
    println!("📊 Status: {:?}", status);
    if let Some(ctx) = context {
        println!("🔍 Context: {}", ctx);
    }
    if let Some(makers) = makers {
        println!("👥 Decision makers: {}", makers);
    }
    if let Some(alternatives) = alternatives {
        println!("🔄 Alternatives considered: {}", alternatives);
    }
    if let Some(rationale) = rationale {
        println!("🧠 Rationale: {}", rationale);
    }
    if let Some(consequences) = consequences {
        println!("📈 Consequences: {}", consequences);
    }
    
    Ok(())
}

fn list_decisions(
    scope: &crate::Scope,
    status: &Option<DecisionStatus>,
    maker: &Option<String>,
) -> GacpResult<()> {
    let decisions = file_ops::list_decisions(
        &scope.path,
        status.clone(),
        maker.clone(),
    )?;
    
    if decisions.is_empty() {
        println!("📭 No decisions found");
        return Ok(());
    }
    
    println!("🎯 Decisions in scope: {}", scope.definition.name);
    println!("{}", "─".repeat(80));
    
    for decision in decisions {
        let status_color = match decision.status {
            DecisionStatus::Proposed => "yellow",
            DecisionStatus::UnderReview => "blue",
            DecisionStatus::Approved => "green",
            DecisionStatus::Rejected => "red",
            DecisionStatus::Implemented => "green",
            DecisionStatus::Deprecated => "dimmed",
        };
        
        println!("🆔 ID: {}", decision.id);
        println!("📝 Title: {}", decision.title);
        println!("📄 Description: {}", decision.description);
        println!("📊 Status: {}", format!("{:?}", decision.status).color(status_color));
        if let Some(ctx) = &decision.context {
            println!("🔍 Context: {}", ctx);
        }
        if let Some(alternatives) = &decision.alternatives {
            println!("🔄 Alternatives: {}", alternatives.join(", "));
        }
        if let Some(rationale) = &decision.rationale {
            println!("🧠 Rationale: {}", rationale);
        }
        if let Some(consequences) = &decision.consequences {
            println!("📈 Consequences: {}", consequences.join(", "));
        }
        if let Some(makers) = &decision.decision_makers {
            println!("👥 Decision makers: {}", makers.join(", "));
        }
        println!("📅 Decided: {}", decision.decided_at.format("%Y-%m-%d %H:%M"));
        if let Some(review_date) = &decision.review_date {
            println!("📋 Review date: {}", review_date.format("%Y-%m-%d %H:%M"));
        }
        println!("{}", "─".repeat(80));
    }
    
    Ok(())
}

fn update_decision(
    scope: &crate::Scope,
    id: &str,
    title: &Option<String>,
    description: &Option<String>,
    status: &Option<DecisionStatus>,
    context: &Option<String>,
    makers: &Option<String>,
    alternatives: &Option<String>,
    rationale: &Option<String>,
    consequences: &Option<String>,
) -> GacpResult<()> {
    file_ops::update_decision(
        &scope.path,
        id,
        title.clone(),
        description.clone(),
        status.clone(),
        context.clone(),
        makers.clone(),
        alternatives.clone(),
        rationale.clone(),
        consequences.clone(),
    )?;
    
    println!("✅ Decision {} updated successfully", id.green());
    
    if title.is_some() || description.is_some() || status.is_some() || 
       context.is_some() || makers.is_some() || alternatives.is_some() || 
       rationale.is_some() || consequences.is_some() {
        println!("📝 Updated fields:");
        if title.is_some() {
            println!("  - Title: {}", title.as_ref().unwrap());
        }
        if description.is_some() {
            println!("  - Description: {}", description.as_ref().unwrap());
        }
        if status.is_some() {
            println!("  - Status: {:?}", status.as_ref().unwrap());
        }
        if context.is_some() {
            println!("  - Context: {}", context.as_ref().unwrap());
        }
        if makers.is_some() {
            println!("  - Decision makers: {}", makers.as_ref().unwrap());
        }
        if alternatives.is_some() {
            println!("  - Alternatives: {}", alternatives.as_ref().unwrap());
        }
        if rationale.is_some() {
            println!("  - Rationale: {}", rationale.as_ref().unwrap());
        }
        if consequences.is_some() {
            println!("  - Consequences: {}", consequences.as_ref().unwrap());
        }
    }
    
    Ok(())
}

fn delete_decision(
    scope: &crate::Scope,
    id: &str,
) -> GacpResult<()> {
    file_ops::delete_decision(&scope.path, id)?;
    
    println!("🗑️  Decision {} deleted successfully", id.green());
    
    Ok(())
} 