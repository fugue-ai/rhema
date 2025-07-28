use crate::{Gacp, GacpResult, scope::build_dependency_graph, file_ops::read_yaml_file, Knowledge, KnowledgeEntry};
use colored::*;
use std::collections::HashMap;
use chrono::Utc;

pub fn run(gacp: &Gacp) -> GacpResult<()> {
    println!("{}", "🔄 Syncing knowledge across scopes...".blue().bold());
    
    // Discover all scopes
    let scopes = gacp.discover_scopes()?;
    
    if scopes.is_empty() {
        println!("{}", "No scopes found in the repository.".yellow());
        return Ok(());
    }
    
    // Build dependency graph
    let dependency_graph = build_dependency_graph(&scopes)?;
    
    // Collect all knowledge from all scopes
    let all_knowledge = collect_all_knowledge(gacp, &scopes)?;
    
    // Identify knowledge conflicts
    let conflicts = identify_knowledge_conflicts(&all_knowledge)?;
    
    // Sync knowledge across dependent scopes
    sync_knowledge_across_scopes(gacp, &scopes, &dependency_graph, &all_knowledge)?;
    
    // Update cross-references
    update_cross_references(gacp, &scopes, &all_knowledge)?;
    
    // Resolve conflicts
    resolve_knowledge_conflicts(gacp, &conflicts)?;
    
    println!("{}", "✅ Knowledge sync completed!".green().bold());
    
    Ok(())
}

fn collect_all_knowledge(
    gacp: &Gacp,
    scopes: &[crate::scope::Scope],
) -> GacpResult<HashMap<String, Vec<KnowledgeEntry>>> {
    println!("{}", "📚 Collecting knowledge from all scopes...".blue());
    
    let mut all_knowledge = HashMap::new();
    
    for scope in scopes {
        let scope_path = scope.relative_path(gacp.repo_root())?;
        
        if scope.has_file("knowledge.yaml") {
            let knowledge_file = scope.path.join("knowledge.yaml");
            match read_yaml_file::<Knowledge>(&knowledge_file) {
                Ok(knowledge) => {
                    let entry_count = knowledge.entries.len();
                    all_knowledge.insert(scope_path.clone(), knowledge.entries);
                    println!("  📖 Found {} knowledge entries in {}", entry_count, scope_path);
                }
                Err(e) => {
                    println!("  ⚠️  Could not read knowledge from {}: {}", scope_path, e);
                }
            }
        }
    }
    
    Ok(all_knowledge)
}

fn identify_knowledge_conflicts(
    all_knowledge: &HashMap<String, Vec<KnowledgeEntry>>,
) -> GacpResult<Vec<KnowledgeConflict>> {
    println!("{}", "🔍 Identifying knowledge conflicts...".blue());
    
    let mut conflicts = Vec::new();
    let mut knowledge_by_title = HashMap::new();
    
    // Group knowledge entries by title
    for (scope_path, entries) in all_knowledge {
        for entry in entries {
            knowledge_by_title
                .entry(entry.title.clone())
                .or_insert_with(Vec::new)
                .push((scope_path.clone(), entry.clone()));
        }
    }
    
    // Find conflicts (same title, different content)
    for (title, entries) in knowledge_by_title {
        if entries.len() > 1 {
            // Check if content differs
            let first_content = &entries[0].1.content;
            let mut has_conflict = false;
            
            for (_, entry) in &entries[1..] {
                if entry.content != *first_content {
                    has_conflict = true;
                    break;
                }
            }
            
            if has_conflict {
                conflicts.push(KnowledgeConflict {
                    title,
                    entries: entries.into_iter().map(|(scope, entry)| (scope, entry)).collect(),
                });
            }
        }
    }
    
    if conflicts.is_empty() {
        println!("  ✅ No knowledge conflicts found");
    } else {
        println!("  ⚠️  Found {} knowledge conflicts", conflicts.len());
        for conflict in &conflicts {
            println!("    🔴 Conflict in '{}' across {} scopes", conflict.title, conflict.entries.len());
        }
    }
    
    Ok(conflicts)
}

fn sync_knowledge_across_scopes(
    gacp: &Gacp,
    scopes: &[crate::scope::Scope],
    dependency_graph: &HashMap<String, Vec<String>>,
    all_knowledge: &HashMap<String, Vec<KnowledgeEntry>>,
) -> GacpResult<()> {
    println!("{}", "🔄 Syncing knowledge across dependent scopes...".blue());
    
    let mut synced_count = 0;
    
    for scope in scopes {
        let scope_path = scope.relative_path(gacp.repo_root())?;
        let empty_vec = Vec::new();
        let dependencies = dependency_graph.get(&scope_path).unwrap_or(&empty_vec);
        let dependencies = dependencies.clone();
        
        for dep_scope_path in dependencies {
            if let Some(dep_knowledge) = all_knowledge.get(&dep_scope_path) {
                // Sync relevant knowledge from dependent scope
                let synced = sync_knowledge_from_dependency(
                    gacp,
                    scope,
                    &dep_scope_path,
                    dep_knowledge,
                )?;
                synced_count += synced;
            }
        }
    }
    
    println!("  📤 Synced {} knowledge entries across scopes", synced_count);
    
    Ok(())
}

fn sync_knowledge_from_dependency(
    _gacp: &Gacp,
    target_scope: &crate::scope::Scope,
    dep_scope_path: &str,
    dep_knowledge: &[KnowledgeEntry],
) -> GacpResult<usize> {
    let mut synced_count = 0;
    
    // Get target scope's existing knowledge
    let target_knowledge_file = target_scope.path.join("knowledge.yaml");
    let mut target_knowledge = if target_knowledge_file.exists() {
        read_yaml_file::<Knowledge>(&target_knowledge_file)?
    } else {
        Knowledge {
            entries: Vec::new(),
            categories: Some(HashMap::new()),
            custom: HashMap::new(),
        }
    };
    
    // Find knowledge that should be synced (high confidence, relevant categories)
    for entry in dep_knowledge {
        if should_sync_knowledge(entry, &target_scope.definition) {
            // Check if this knowledge already exists
            let exists = target_knowledge.entries.iter().any(|e| e.title == entry.title);
            
            if !exists {
                // Create a synced version of the knowledge
                let mut synced_entry = entry.clone();
                synced_entry.id = uuid::Uuid::new_v4().to_string();
                synced_entry.created_at = Utc::now();
                synced_entry.source = Some(format!("Synced from {}", dep_scope_path));
                
                target_knowledge.entries.push(synced_entry);
                synced_count += 1;
            }
        }
    }
    
    // Write updated knowledge back to target scope
    if synced_count > 0 {
        crate::file_ops::write_yaml_file(&target_knowledge_file, &target_knowledge)?;
    }
    
    Ok(synced_count)
}

fn should_sync_knowledge(entry: &KnowledgeEntry, _target_scope: &crate::schema::GacpScope) -> bool {
    // Only sync high-confidence knowledge
    if let Some(confidence) = entry.confidence {
        if confidence < 7 {
            return false;
        }
    }
    
    // Check if knowledge is relevant to target scope
    if let Some(category) = &entry.category {
        // Simple relevance check - could be enhanced with more sophisticated logic
        let relevant_categories = ["architecture", "design", "patterns", "decisions", "api"];
        if relevant_categories.iter().any(|c| category.to_lowercase().contains(c)) {
            return true;
        }
    }
    
    // Check tags for relevance
    if let Some(tags) = &entry.tags {
        let relevant_tags = ["shared", "common", "core", "fundamental"];
        if tags.iter().any(|tag| relevant_tags.iter().any(|rt| tag.to_lowercase().contains(rt))) {
            return true;
        }
    }
    
    false
}

fn update_cross_references(
    gacp: &Gacp,
    scopes: &[crate::scope::Scope],
    all_knowledge: &HashMap<String, Vec<KnowledgeEntry>>,
) -> GacpResult<()> {
    println!("{}", "🔗 Updating cross-references...".blue());
    
    let mut updated_count = 0;
    
    for scope in scopes {
        let scope_path = scope.relative_path(gacp.repo_root())?;
        
        if scope.has_file("knowledge.yaml") {
            let knowledge_file = scope.path.join("knowledge.yaml");
            let mut knowledge = read_yaml_file::<Knowledge>(&knowledge_file)?;
            let mut updated = false;
            
            for entry in &mut knowledge.entries {
                // Update cross-references to other scopes
                if let Some(updated_content) = update_knowledge_references(entry, &scope_path, all_knowledge) {
                    entry.content = updated_content;
                    entry.updated_at = Some(Utc::now());
                    updated = true;
                }
            }
            
            if updated {
                crate::file_ops::write_yaml_file(&knowledge_file, &knowledge)?;
                updated_count += 1;
            }
        }
    }
    
    println!("  🔄 Updated cross-references in {} scope(s)", updated_count);
    
    Ok(())
}

fn update_knowledge_references(
    entry: &KnowledgeEntry,
    current_scope: &str,
    all_knowledge: &HashMap<String, Vec<KnowledgeEntry>>,
) -> Option<String> {
    let mut updated_content = entry.content.clone();
    let mut updated = false;
    
    // Look for references to other scopes in the content
    for (scope_path, scope_knowledge) in all_knowledge {
        if scope_path != current_scope {
            for scope_entry in scope_knowledge {
                // Simple reference detection - could be enhanced with more sophisticated parsing
                let reference_pattern = format!("@{}", scope_entry.title);
                if updated_content.contains(&reference_pattern) {
                    let replacement = format!("@{} (from {})", scope_entry.title, scope_path);
                    updated_content = updated_content.replace(&reference_pattern, &replacement);
                    updated = true;
                }
            }
        }
    }
    
    if updated {
        Some(updated_content)
    } else {
        None
    }
}

fn resolve_knowledge_conflicts(
    gacp: &Gacp,
    conflicts: &[KnowledgeConflict],
) -> GacpResult<()> {
    if conflicts.is_empty() {
        return Ok(());
    }
    
    println!("{}", "⚖️  Resolving knowledge conflicts...".blue());
    
    for conflict in conflicts {
        println!("  🔴 Resolving conflict in '{}':", conflict.title);
        
        // Simple conflict resolution strategy: prefer higher confidence, then newer entries
        let mut sorted_entries: Vec<_> = conflict.entries.iter().collect();
        sorted_entries.sort_by(|a, b| {
            // First by confidence (higher is better)
            let a_conf = a.1.confidence.unwrap_or(0);
            let b_conf = b.1.confidence.unwrap_or(0);
            b_conf.cmp(&a_conf)
                .then_with(|| {
                    // Then by creation date (newer is better)
                    b.1.created_at.cmp(&a.1.created_at)
                })
        });
        
        // Use the best entry as the canonical version
        if let Some((best_scope, best_entry)) = sorted_entries.first() {
            println!("    ✅ Using version from {} (confidence: {}, created: {})", 
                best_scope, 
                best_entry.confidence.unwrap_or(0),
                best_entry.created_at.format("%Y-%m-%d %H:%M")
            );
            
            // Update other scopes with the canonical version
            for (scope_path, entry) in &conflict.entries {
                if *scope_path != *best_scope {
                    update_knowledge_entry(gacp, scope_path, &entry.id, &best_entry.content)?;
                }
            }
        }
    }
    
    Ok(())
}

fn update_knowledge_entry(
    gacp: &Gacp,
    scope_path: &str,
    entry_id: &str,
    new_content: &str,
) -> GacpResult<()> {
    // Find the scope
    let scopes = gacp.discover_scopes()?;
    let scope = scopes.iter().find(|s| s.relative_path(gacp.repo_root()).unwrap_or_default() == scope_path);
    
    if let Some(scope) = scope {
        if scope.has_file("knowledge.yaml") {
            let knowledge_file = scope.path.join("knowledge.yaml");
            let mut knowledge = read_yaml_file::<Knowledge>(&knowledge_file)?;
            
            // Find and update the entry
            for entry in &mut knowledge.entries {
                if entry.id == entry_id {
                    entry.content = new_content.to_string();
                    entry.updated_at = Some(Utc::now());
                    break;
                }
            }
            
            crate::file_ops::write_yaml_file(&knowledge_file, &knowledge)?;
        }
    }
    
    Ok(())
}

#[derive(Debug)]
struct KnowledgeConflict {
    title: String,
    entries: Vec<(String, KnowledgeEntry)>,
} 