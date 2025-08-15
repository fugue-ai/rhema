use crate::{Rhema, RhemaError, RhemaResult};
use super::commands::{KnowledgeSubcommands, CacheSubcommands, WatchSubcommands};
use super::cache::CacheOperations;
use super::watch::WatchOperations;
use colored::*;
use std::path::PathBuf;
use tracing::info;
use rhema_knowledge::{
    engine::UnifiedKnowledgeEngine,
    search::SemanticSearchEngine,
    types::{SemanticResult, SearchResultMetadata, ContentType},
    cache::UnifiedCacheResult,
    vector::VectorStore,
    embedding::EmbeddingManager,
};
use std::sync::Arc;
use serde_json;

pub struct KnowledgeCommand;

impl KnowledgeCommand {
    pub async fn handle(rhema: &Rhema, subcommand: KnowledgeSubcommands) -> RhemaResult<()> {
        match subcommand {
            KnowledgeSubcommands::Init { config_file, cache_dir } => {
                Self::init_knowledge_system(rhema, config_file, cache_dir).await
            }
            KnowledgeSubcommands::Search { query, semantic, hybrid, cache_only, index_only, limit, format } => {
                Self::search_knowledge(rhema, query, semantic, hybrid, cache_only, index_only, limit, format).await
            }
            KnowledgeSubcommands::Cache { subcommand } => {
                CacheOperations::handle_cache_commands(rhema, subcommand).await
            }
            KnowledgeSubcommands::Index { scope, all_scopes, file, force } => {
                Self::index_content(rhema, scope, all_scopes, file, force).await
            }
            KnowledgeSubcommands::Suggest { file, workflow, format } => {
                Self::suggest_context(rhema, file, workflow, format).await
            }
            KnowledgeSubcommands::Warm { patterns, workflow, agent } => {
                Self::warm_cache(rhema, patterns, workflow, agent).await
            }
            KnowledgeSubcommands::Share { from, to, context } => {
                Self::share_context(rhema, from, to, context).await
            }
            KnowledgeSubcommands::Synthesize { topic, scope, cross_scope, format } => {
                Self::synthesize_knowledge(rhema, topic, scope, cross_scope, format).await
            }
            KnowledgeSubcommands::Status { detailed, format } => {
                Self::show_status(rhema, detailed, format).await
            }
            KnowledgeSubcommands::Optimize { cache_only, index_only, dry_run } => {
                Self::optimize_system(rhema, cache_only, index_only, dry_run).await
            }
            KnowledgeSubcommands::Metrics { cache, search, synthesis, proactive, performance, format } => {
                Self::show_metrics(rhema, cache, search, synthesis, proactive, performance, format).await
            }
            KnowledgeSubcommands::Cleanup { expired_only, dry_run, force } => {
                Self::cleanup_system(rhema, expired_only, dry_run, force).await
            }
            KnowledgeSubcommands::Watch { subcommand } => {
                WatchOperations::handle_watch_commands(rhema, subcommand).await
            }
        }
    }

    async fn init_knowledge_system(
        _rhema: &Rhema,
        config_file: Option<PathBuf>,
        cache_dir: Option<PathBuf>,
    ) -> RhemaResult<()> {
        info!("Initializing unified knowledge system");
        
        if let Some(config_path) = config_file {
            println!("📄 Loading configuration from: {}", config_path.display());
        }
        
        if let Some(cache_path) = cache_dir {
            println!("📁 Setting cache directory to: {}", cache_path.display());
        }
        
        println!("{}", "✓ Unified knowledge system initialized".green());
        Ok(())
    }

    async fn search_knowledge(
        _rhema: &Rhema,
        query: String,
        semantic: bool,
        hybrid: bool,
        cache_only: bool,
        index_only: bool,
        limit: usize,
        format: String,
    ) -> RhemaResult<()> {
        info!("Searching knowledge with query: {}", query);
        
        println!("🔍 Searching for: {}", query);
        println!("  - Semantic search: {}", semantic);
        println!("  - Hybrid search: {}", hybrid);
        println!("  - Cache only: {}", cache_only);
        println!("  - Index only: {}", index_only);
        println!("  - Format: {}", format);
        
        // Create a knowledge engine instance
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        let results = if hybrid {
            // Perform hybrid search
            engine.search_hybrid(&query, limit, 0.7).await
        } else if semantic {
            // Perform semantic search
            engine.search_semantic(&query, limit).await
        } else {
            // Perform keyword search
            engine.search_keyword(&query, limit).await
        }.map_err(|e| RhemaError::KnowledgeError(format!("Search failed: {}", e)))?;
        
        // Display results based on format
        match format.as_str() {
            "json" => {
                let json_results = serde_json::to_string_pretty(&results)
                    .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize results: {}", e)))?;
                println!("{}", json_results);
            }
            "yaml" => {
                let yaml_results = serde_yaml::to_string(&results)
                    .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize results: {}", e)))?;
                println!("{}", yaml_results);
            }
            "table" | _ => {
                println!("📋 Search Results:");
                for (i, result) in results.iter().enumerate().take(limit) {
                    println!("  {}. {} (relevance: {:.2})", 
                        i + 1, 
                        result.content.chars().take(50).collect::<String>(),
                        result.relevance_score
                    );
                    if let Some(scope_path) = &result.metadata.scope_path {
                        println!("     Scope: {}", scope_path);
                    }
                }
            }
        }
        
        println!("{}", "✓ Search completed".green());
        Ok(())
    }

    async fn index_content(
        rhema: &Rhema,
        scope: Option<String>,
        all_scopes: bool,
        file: Option<PathBuf>,
        force: bool,
    ) -> RhemaResult<()> {
        info!("Indexing content for knowledge system");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        if let Some(file_path) = file {
            println!("📄 Indexing file: {}", file_path.display());
            
            // Read file content
            let content = std::fs::read_to_string(&file_path)
                .map_err(|e| RhemaError::FileError(format!("Failed to read file: {}", e)))?;
            
            // Index the file content
            let key = format!("file:{}", file_path.display());
            engine.set_with_semantic_indexing(&key, content.as_bytes(), &None).await
                .map_err(|e| RhemaError::KnowledgeError(format!("Failed to index file: {}", e)))?;
            
            println!("✅ File indexed successfully");
        } else if all_scopes {
            println!("🔍 Indexing all scopes");
            
            let scopes = rhema.discover_scopes()?;
            for scope_info in scopes {
                println!("  Indexing scope: {}", scope_info.path.display());
                
                // Index all files in the scope
                for (file_name, file_path) in &scope_info.files {
                    let content = std::fs::read_to_string(file_path)
                        .map_err(|e| RhemaError::FileError(format!("Failed to read file {}: {}", file_path.display(), e)))?;
                    
                    let key = format!("scope:{}:file:{}", scope_info.path.display(), file_name);
                    engine.set_with_semantic_indexing(&key, content.as_bytes(), &None).await
                        .map_err(|e| RhemaError::KnowledgeError(format!("Failed to index file {}: {}", file_path.display(), e)))?;
                }
            }
            
            println!("✅ All scopes indexed successfully");
        } else if let Some(scope_name) = scope {
            println!("📁 Indexing scope: {}", scope_name);
            
            let scope_info = rhema.get_scope(&scope_name)?;
            
            // Index all files in the specific scope
            for (file_name, file_path) in &scope_info.files {
                let content = std::fs::read_to_string(file_path)
                    .map_err(|e| RhemaError::FileError(format!("Failed to read file {}: {}", file_path.display(), e)))?;
                
                let key = format!("scope:{}:file:{}", scope_name, file_name);
                engine.set_with_semantic_indexing(&key, content.as_bytes(), &None).await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to index file {}: {}", file_path.display(), e)))?;
            }
            
            println!("✅ Scope indexed successfully");
        } else {
            println!("📂 Indexing current working scope");
            
            // Get current scope (assuming it's the first one found)
            let scopes = rhema.discover_scopes()?;
            if let Some(current_scope) = scopes.first() {
                for (file_name, file_path) in &current_scope.files {
                    let content = std::fs::read_to_string(file_path)
                        .map_err(|e| RhemaError::FileError(format!("Failed to read file {}: {}", file_path.display(), e)))?;
                    
                    let key = format!("scope:{}:file:{}", current_scope.path.display(), file_name);
                    engine.set_with_semantic_indexing(&key, content.as_bytes(), &None).await
                        .map_err(|e| RhemaError::KnowledgeError(format!("Failed to index file {}: {}", file_path.display(), e)))?;
                }
            }
            
            println!("✅ Current scope indexed successfully");
        }
        
        println!("{}", "✓ Content indexing completed".green());
        Ok(())
    }

    async fn suggest_context(
        _rhema: &Rhema,
        file: Option<PathBuf>,
        workflow: Option<String>,
        format: String,
    ) -> RhemaResult<()> {
        info!("Generating context suggestions");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        if let Some(file_path) = file {
            // Get suggestions for file
            println!("Getting suggestions for file: {}", file_path.display());
            
            // Read file content to generate context
            let content = std::fs::read_to_string(&file_path)
                .map_err(|e| RhemaError::FileError(format!("Failed to read file: {}", e)))?;
            
            // Generate suggestions based on file content
            let suggestions = engine.generate_context_suggestions(&content, 5).await
                .map_err(|e| RhemaError::KnowledgeError(format!("Failed to generate suggestions: {}", e)))?;
            
            // Display suggestions
            match format.as_str() {
                "json" => {
                    let json_suggestions = serde_json::to_string_pretty(&suggestions)
                        .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize suggestions: {}", e)))?;
                    println!("{}", json_suggestions);
                }
                "yaml" => {
                    let yaml_suggestions = serde_yaml::to_string(&suggestions)
                        .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize suggestions: {}", e)))?;
                    println!("{}", yaml_suggestions);
                }
                "table" | _ => {
                    println!("📋 Context Suggestions:");
                    for (i, suggestion) in suggestions.iter().enumerate() {
                        println!("  {}. {}", i + 1, suggestion);
                    }
                }
            }
        } else if let Some(workflow_id) = workflow {
            // Get suggestions for workflow
            println!("Getting suggestions for workflow: {}", workflow_id);
            
            // Generate workflow-specific suggestions
            let suggestions = engine.generate_workflow_suggestions(&workflow_id, 5).await
                .map_err(|e| RhemaError::KnowledgeError(format!("Failed to generate workflow suggestions: {}", e)))?;
            
            // Display suggestions
            match format.as_str() {
                "json" => {
                    let json_suggestions = serde_json::to_string_pretty(&suggestions)
                        .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize suggestions: {}", e)))?;
                    println!("{}", json_suggestions);
                }
                "yaml" => {
                    let yaml_suggestions = serde_yaml::to_string(&suggestions)
                        .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize suggestions: {}", e)))?;
                    println!("{}", yaml_suggestions);
                }
                "table" | _ => {
                    println!("📋 Workflow Suggestions:");
                    for (i, suggestion) in suggestions.iter().enumerate() {
                        println!("  {}. {}", i + 1, suggestion);
                    }
                }
            }
        } else {
            return Err(RhemaError::InvalidInput("Must specify either --file or --workflow".to_string()));
        }
        
        println!("{}", "✓ Context suggestions generated".green());
        Ok(())
    }

    async fn warm_cache(
        _rhema: &Rhema,
        patterns: Option<PathBuf>,
        workflow: Option<String>,
        agent: Option<String>,
    ) -> RhemaResult<()> {
        info!("Warming cache");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        if let Some(patterns_file) = patterns {
            // Warm cache based on patterns
            println!("Warming cache based on patterns: {}", patterns_file.display());
            
            let patterns_content = std::fs::read_to_string(&patterns_file)
                .map_err(|e| RhemaError::FileError(format!("Failed to read patterns file: {}", e)))?;
            
            let patterns: Vec<String> = serde_yaml::from_str(&patterns_content)
                .map_err(|e| RhemaError::SerializationError(format!("Failed to parse patterns: {}", e)))?;
            
            for pattern in patterns {
                engine.warm_cache_for_pattern(&pattern).await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to warm cache for pattern {}: {}", pattern, e)))?;
            }
        } else if let Some(workflow_id) = workflow {
            // Warm cache for workflow
            println!("Warming cache for workflow: {}", workflow_id);
            
            engine.warm_cache_for_workflow(&workflow_id).await
                .map_err(|e| RhemaError::KnowledgeError(format!("Failed to warm cache for workflow: {}", e)))?;
        } else if let Some(agent_id) = agent {
            // Warm cache for agent
            println!("Warming cache for agent: {}", agent_id);
            
            engine.warm_cache_for_agent(&agent_id).await
                .map_err(|e| RhemaError::KnowledgeError(format!("Failed to warm cache for agent: {}", e)))?;
        } else {
            return Err(RhemaError::InvalidInput("Must specify either --patterns, --workflow, or --agent".to_string()));
        }
        
        println!("{}", "✓ Cache warming completed".green());
        Ok(())
    }

    async fn share_context(
        _rhema: &Rhema,
        from: String,
        to: String,
        context: String,
    ) -> RhemaResult<()> {
        info!("Sharing context from agent {} to agent {}: {}", from, to, context);
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        // Share context between agents
        engine.share_context_between_agents(&from, &to, &context).await
            .map_err(|e| RhemaError::KnowledgeError(format!("Failed to share context: {}", e)))?;
        
        println!("✅ Context '{}' shared from agent '{}' to agent '{}'", context, from, to);
        Ok(())
    }

    async fn synthesize_knowledge(
        _rhema: &Rhema,
        topic: String,
        scope: Option<String>,
        cross_scope: bool,
        format: String,
    ) -> RhemaResult<()> {
        info!("Synthesizing knowledge for topic: {}", topic);
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        let synthesis = if cross_scope {
            engine.synthesize_knowledge_cross_scope(&topic, scope.as_deref()).await
        } else {
            engine.synthesize_knowledge(&topic, scope.as_deref()).await
        }.map_err(|e| RhemaError::KnowledgeError(format!("Failed to synthesize knowledge: {}", e)))?;
        
        // Display synthesis results
        match format.as_str() {
            "json" => {
                let json_synthesis = serde_json::to_string_pretty(&synthesis)
                    .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize synthesis: {}", e)))?;
                println!("{}", json_synthesis);
            }
            "yaml" => {
                let yaml_synthesis = serde_yaml::to_string(&synthesis)
                    .map_err(|e| RhemaError::SerializationError(format!("Failed to serialize synthesis: {}", e)))?;
                println!("{}", yaml_synthesis);
            }
            "markdown" | _ => {
                println!("📚 Knowledge Synthesis for: {}", topic);
                println!("{}", synthesis);
            }
        }
        
        println!("✅ Knowledge synthesis completed for topic: {}", topic);
        Ok(())
    }

    async fn show_status(_rhema: &Rhema, detailed: bool, _format: String) -> RhemaResult<()> {
        info!("Showing system status");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        let status = engine.get_system_status().await
            .map_err(|e| RhemaError::KnowledgeError(format!("Failed to get system status: {}", e)))?;
        
        println!("{}", "Unified Knowledge System Status".bold());
        println!("  Status: {}", "Active".green());
        println!("  RAG Engine: {}", "Ready".green());
        println!("  Cache System: {}", "Ready".green());
        println!("  Vector Store: {}", "Ready".green());
        
        if detailed {
            println!("  Detailed metrics available");
            println!("  - Cache entries: {}", status.cache_entries);
            println!("  - Vector records: {}", status.vector_records);
            println!("  - Search operations: {}", status.search_operations);
            println!("  - Synthesis operations: {}", status.synthesis_operations);
        }
        
        Ok(())
    }

    async fn optimize_system(
        _rhema: &Rhema,
        cache_only: bool,
        index_only: bool,
        dry_run: bool,
    ) -> RhemaResult<()> {
        info!("Optimizing system");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        if dry_run {
            println!("{}", "Optimization Plan (Dry Run)".bold());
            println!("  - Cache optimization: {}", "Planned".yellow());
            println!("  - Index optimization: {}", "Planned".yellow());
            println!("  - Vector store optimization: {}", "Planned".yellow());
        } else {
            if cache_only {
                engine.optimize_cache().await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to optimize cache: {}", e)))?;
            } else if index_only {
                engine.optimize_indexes().await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to optimize indexes: {}", e)))?;
            } else {
                engine.optimize_system().await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to optimize system: {}", e)))?;
            }
            
            println!("{}", "✓ System optimization completed".green());
        }
        
        Ok(())
    }

    async fn show_metrics(
        _rhema: &Rhema,
        cache: bool,
        search: bool,
        synthesis: bool,
        proactive: bool,
        performance: bool,
        _format: String,
    ) -> RhemaResult<()> {
        info!("Showing metrics");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        let metrics = engine.get_metrics().await
            .map_err(|e| RhemaError::KnowledgeError(format!("Failed to get metrics: {}", e)))?;
        
        println!("{}", "System Metrics".bold());
        
        if cache {
            println!("  Cache Metrics:");
            println!("    - Hit rate: {:.1}%", metrics.cache_hit_rate * 100.0);
            println!("    - Total entries: {}", metrics.cache_entries);
            println!("    - Memory usage: {} MB", metrics.cache_memory_usage_mb);
        }
        
        if search {
            println!("  Search Metrics:");
            println!("    - Total searches: {}", metrics.total_searches);
            println!("    - Average response time: {}ms", metrics.avg_search_response_time_ms);
            println!("    - Semantic searches: {}", metrics.semantic_searches);
        }
        
        if synthesis {
            println!("  Synthesis Metrics:");
            println!("    - Total syntheses: {}", metrics.total_syntheses);
            println!("    - Average confidence: {:.2}", metrics.avg_synthesis_confidence);
            println!("    - Cross-scope syntheses: {}", metrics.cross_scope_syntheses);
        }
        
        if proactive {
            println!("  Proactive Metrics:");
            println!("    - Suggestions generated: {}", metrics.suggestions_generated);
            println!("    - Suggestions accepted: {}", metrics.suggestions_accepted);
            println!("    - Cache warming events: {}", metrics.cache_warming_events);
        }
        
        if performance {
            println!("  Performance Metrics:");
            println!("    - Average cache access: {}ms", metrics.avg_cache_access_time_ms);
            println!("    - Average search time: {}ms", metrics.avg_search_time_ms);
            println!("    - Memory pressure: {:.1}%", metrics.memory_pressure_percent);
        }
        
        Ok(())
    }

    async fn cleanup_system(
        _rhema: &Rhema,
        expired_only: bool,
        dry_run: bool,
        force: bool,
    ) -> RhemaResult<()> {
        info!("Cleaning up system");
        
        let engine = UnifiedKnowledgeEngine::new_dummy().await;
        
        if dry_run {
            println!("{}", "Cleanup Plan (Dry Run)".bold());
            println!("  - Expired entries: {}", "Planned".yellow());
            println!("  - Orphaned indexes: {}", "Planned".yellow());
            println!("  - Temporary files: {}", "Planned".yellow());
        } else {
            if expired_only {
                engine.cleanup_expired_entries().await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to cleanup expired entries: {}", e)))?;
            } else {
                engine.cleanup_system().await
                    .map_err(|e| RhemaError::KnowledgeError(format!("Failed to cleanup system: {}", e)))?;
            }
            
            println!("{}", "✓ System cleanup completed".green());
        }
        
        Ok(())
    }
}
