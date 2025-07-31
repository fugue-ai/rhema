use crate::{Rhema, RhemaError, RhemaResult};
use rhema_core::schema::{
    ExportMetadata, PromptInjectionMethod, PromptPattern, PromptVersion, Prompts, SharedTemplate,
    TemplateAccessControl, TemplateComplexity, TemplateExport, TemplateLibrary, TemplateMetadata,
    TemplateUsageStats, UsageAnalytics,
};
// TODO: Implement these functions or import from appropriate module
use crate::{
    load_prompts, load_template_export, load_template_library, save_prompts, save_template_export,
    save_template_library,
};
use chrono::Utc;
use clap::Subcommand;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum TemplateSubcommands {
    CreateLibrary {
        name: String,
        description: Option<String>,
        owner: String,
        tags: Option<String>,
        public: bool,
        scope: Option<String>,
    },
    ListLibraries {
        scope: Option<String>,
        tags: Option<String>,
    },
    AddTemplate {
        library: String,
        name: String,
        template: String,
        description: Option<String>,
        category: Option<String>,
        complexity: Option<String>,
        language: Option<String>,
        dependencies: Option<String>,
        examples: Option<String>,
        tags: Option<String>,
        scope: Option<String>,
    },
    ExportTemplates {
        templates: String,
        output_file: String,
        description: Option<String>,
        tags: Option<String>,
        scope: Option<String>,
    },
    ImportTemplates {
        input_file: String,
        library: Option<String>,
        scope: Option<String>,
    },
    ShareLibrary {
        library: String,
        target_scope: String,
        scope: Option<String>,
    },
    DownloadTemplate {
        library: String,
        template: String,
        scope: Option<String>,
    },
    RateTemplate {
        library: String,
        template: String,
        rating: f64,
        scope: Option<String>,
    },
    ShowLibrary {
        library: String,
        scope: Option<String>,
    },
    ShowTemplate {
        library: String,
        template: String,
        scope: Option<String>,
    },
}

pub fn run(rhema: &Rhema, subcommand: &TemplateSubcommands) -> RhemaResult<()> {
    match subcommand {
        TemplateSubcommands::CreateLibrary {
            name,
            description,
            owner,
            tags,
            public,
            scope,
        } => create_library(rhema, name, description, owner, tags, *public, scope),
        TemplateSubcommands::ListLibraries { scope, tags } => list_libraries(rhema, scope, tags),
        TemplateSubcommands::AddTemplate {
            library,
            name,
            template,
            description,
            category,
            complexity,
            language,
            dependencies,
            examples,
            tags,
            scope,
        } => add_template(
            rhema,
            library,
            name,
            template,
            description,
            category,
            complexity,
            language,
            dependencies,
            examples,
            tags,
            scope,
        ),
        TemplateSubcommands::ExportTemplates {
            templates,
            output_file,
            description,
            tags,
            scope,
        } => export_templates(rhema, templates, output_file, description, tags, scope),
        TemplateSubcommands::ImportTemplates {
            input_file,
            library,
            scope,
        } => import_templates(rhema, input_file, library, scope),
        TemplateSubcommands::ShareLibrary {
            library,
            target_scope,
            scope,
        } => share_library(rhema, library, target_scope, scope),
        TemplateSubcommands::DownloadTemplate {
            library,
            template,
            scope,
        } => download_template(rhema, library, template, scope),
        TemplateSubcommands::RateTemplate {
            library,
            template,
            rating,
            scope,
        } => rate_template(rhema, library, template, *rating, scope),
        TemplateSubcommands::ShowLibrary { library, scope } => show_library(rhema, library, scope),
        TemplateSubcommands::ShowTemplate {
            library,
            template,
            scope,
        } => show_template(rhema, library, template, scope),
    }
}

fn create_library(
    rhema: &Rhema,
    name: &str,
    description: &Option<String>,
    owner: &str,
    tags: &Option<String>,
    public: bool,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let library_path = scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", name));

    // Create directory if it doesn't exist
    if let Some(parent) = library_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Parse tags
    let tags_vec = if let Some(tags_str) = tags {
        Some(tags_str.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };

    // Create access control
    let access_control = TemplateAccessControl {
        public,
        allowed_teams: None,
        allowed_users: None,
        read_only: false,
    };

    // Create new library
    let new_library = TemplateLibrary {
        name: name.to_string(),
        description: description.clone(),
        owner: owner.to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        templates: Vec::new(),
        tags: tags_vec,
        access_control: Some(access_control),
    };

    save_template_library(&library_path, &new_library)?;

    println!(
        "✅ Created template library '{}' at {}",
        name,
        library_path.display()
    );
    println!("   Owner: {}", owner);
    println!("   Public: {}", public);
    println!("   Use 'rhema template add-template' to add templates to this library");

    Ok(())
}

fn list_libraries(rhema: &Rhema, scope: &Option<String>, tags: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let libraries_dir = scope_path.join(".rhema").join("template-libraries");

    if !libraries_dir.exists() {
        println!("No template libraries found in {}", scope_path.display());
        return Ok(());
    }

    let mut libraries = Vec::new();

    // Load all library files
    for entry in std::fs::read_dir(&libraries_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            if let Ok(library) = load_template_library(&path) {
                libraries.push(library);
            }
        }
    }

    // Filter by tags if specified
    let filtered_libraries = if let Some(tags_str) = tags {
        let filter_tags: Vec<String> = tags_str.split(',').map(|s| s.trim().to_string()).collect();
        libraries
            .into_iter()
            .filter(|lib| {
                if let Some(lib_tags) = &lib.tags {
                    filter_tags.iter().any(|tag| lib_tags.contains(tag))
                } else {
                    false
                }
            })
            .collect()
    } else {
        libraries
    };

    if filtered_libraries.is_empty() {
        println!("No template libraries found");
        return Ok(());
    }

    println!("📚 Template Libraries in {}:", scope_path.display());
    println!("{}", "=".repeat(60));

    for library in filtered_libraries {
        println!("Name: {}", library.name);
        if let Some(desc) = library.description {
            println!("Description: {}", desc);
        }
        println!("Owner: {}", library.owner);
        println!("Version: {}", library.version);
        println!("Templates: {}", library.templates.len());
        if let Some(access) = &library.access_control {
            println!("Public: {}", access.public);
        }
        if let Some(tags) = library.tags {
            println!("Tags: {}", tags.join(", "));
        }
        println!("Created: {}", library.created_at.format("%Y-%m-%d %H:%M"));
        println!("{}", "-".repeat(40));
    }

    Ok(())
}

fn add_template(
    rhema: &Rhema,
    library: &str,
    name: &str,
    template: &str,
    description: &Option<String>,
    category: &Option<String>,
    complexity: &Option<String>,
    language: &Option<String>,
    dependencies: &Option<String>,
    examples: &Option<String>,
    tags: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let library_path = scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));

    if !library_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Template library '{}' not found",
            library
        )));
    }

    let mut template_library = load_template_library(&library_path)?;

    // Parse complexity
    let complexity_enum = if let Some(comp_str) = complexity {
        match comp_str.to_lowercase().as_str() {
            "beginner" => Some(TemplateComplexity::Beginner),
            "intermediate" => Some(TemplateComplexity::Intermediate),
            "advanced" => Some(TemplateComplexity::Advanced),
            "expert" => Some(TemplateComplexity::Expert),
            _ => None,
        }
    } else {
        None
    };

    // Parse dependencies
    let dependencies_vec = if let Some(deps_str) = dependencies {
        Some(deps_str.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };

    // Parse examples
    let examples_vec = if let Some(examples_str) = examples {
        Some(
            examples_str
                .split('|')
                .map(|s| s.trim().to_string())
                .collect(),
        )
    } else {
        None
    };

    // Parse tags
    let tags_vec = if let Some(tags_str) = tags {
        Some(tags_str.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };

    // Create template metadata
    let metadata = TemplateMetadata {
        author: None,
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        category: category.clone(),
        complexity: complexity_enum,
        language: language.clone(),
        dependencies: dependencies_vec,
        examples: examples_vec,
    };

    // Create new template
    let new_template = SharedTemplate {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        description: description.clone(),
        template: template.to_string(),
        metadata,
        tags: tags_vec,
        usage_stats: TemplateUsageStats::new(),
    };

    // Add template to library
    template_library.templates.push(new_template);
    template_library.updated_at = Utc::now();

    save_template_library(&library_path, &template_library)?;

    println!("✅ Added template '{}' to library '{}'", name, library);
    println!(
        "   Template ID: {}",
        template_library.templates.last().unwrap().id
    );
    if let Some(cat) = category {
        println!("   Category: {}", cat);
    }
    if let Some(comp) = complexity {
        println!("   Complexity: {}", comp);
    }

    Ok(())
}

fn export_templates(
    rhema: &Rhema,
    templates: &str,
    output_file: &str,
    description: &Option<String>,
    tags: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

    if !prompts_path.exists() {
        return Err(RhemaError::InvalidCommand(
            "No prompts.yaml found".to_string(),
        ));
    }

    let prompts = load_prompts(&prompts_path)?;

    // Parse template names/IDs
    let template_names: Vec<String> = templates.split(',').map(|s| s.trim().to_string()).collect();

    // Find matching templates
    let mut exported_templates = Vec::new();
    for template_name in template_names {
        let template = prompts
            .prompts
            .iter()
            .find(|p| p.id == template_name || p.name == template_name)
            .ok_or_else(|| {
                RhemaError::InvalidCommand(format!("Template '{}' not found", template_name))
            })?;

        // Convert PromptPattern to SharedTemplate
        let shared_template = SharedTemplate {
            id: template.id.clone(),
            name: template.name.clone(),
            description: template.description.clone(),
            template: template.template.clone(),
            metadata: TemplateMetadata {
                author: None,
                version: template.version.current.clone(),
                created_at: template.version.created_at,
                updated_at: template.version.updated_at,
                category: None,
                complexity: None,
                language: None,
                dependencies: None,
                examples: None,
            },
            tags: template.tags.clone(),
            usage_stats: TemplateUsageStats::new(),
        };

        exported_templates.push(shared_template);
    }

    // Parse tags
    let tags_vec = if let Some(tags_str) = tags {
        Some(tags_str.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        None
    };

    // Create export
    let export = TemplateExport {
        metadata: ExportMetadata {
            source_scope: scope_path.to_string_lossy().to_string(),
            description: description.clone(),
            tags: tags_vec,
            author: None,
        },
        templates: exported_templates,
        exported_at: Utc::now(),
        export_version: "1.0.0".to_string(),
    };

    // Save export file
    let output_path = PathBuf::from(output_file);
    save_template_export(&output_path, &export)?;

    println!(
        "✅ Exported {} templates to {}",
        export.templates.len(),
        output_file
    );
    println!("   Export version: {}", export.export_version);
    println!(
        "   Exported at: {}",
        export.exported_at.format("%Y-%m-%d %H:%M")
    );

    Ok(())
}

fn import_templates(
    rhema: &Rhema,
    input_file: &str,
    library: &Option<String>,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let input_path = PathBuf::from(input_file);
    if !input_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Import file '{}' not found",
            input_file
        )));
    }

    let export = load_template_export(&input_path)?;

    if let Some(library_name) = library {
        // Import to specific library
        let library_path = scope_path
            .join(".rhema")
            .join("template-libraries")
            .join(format!("{}.yaml", library_name));

        if !library_path.exists() {
            return Err(RhemaError::InvalidCommand(format!(
                "Template library '{}' not found",
                library_name
            )));
        }

        let mut template_library = load_template_library(&library_path)?;

        // Store the count before moving
        let template_count = export.templates.len();

        // Add imported templates
        for template in export.templates {
            template_library.templates.push(template);
        }

        template_library.updated_at = Utc::now();
        save_template_library(&library_path, &template_library)?;

        println!(
            "✅ Imported {} templates to library '{}'",
            template_count, library_name
        );
    } else {
        // Import to prompts.yaml
        let prompts_path = scope_path.join(".rhema").join("prompts.yaml");

        let mut prompts = if prompts_path.exists() {
            load_prompts(&prompts_path)?
        } else {
            Prompts {
                prompts: Vec::new(),
            }
        };

        // Import templates to prompts.yaml
        for template in &export.templates {
            let prompt_pattern = PromptPattern {
                id: template.id.clone(),
                name: template.name.clone(),
                description: template.description.clone(),
                template: template.template.clone(),
                injection: PromptInjectionMethod::TemplateVariable,
                usage_analytics: UsageAnalytics::new(),
                version: PromptVersion::new("1.0.0"),
                tags: template.tags.clone(),
            };

            prompts.prompts.push(prompt_pattern);
        }

        save_prompts(&prompts_path, &prompts)?;

        println!(
            "✅ Imported {} templates to prompts.yaml",
            export.templates.len()
        );
    }

    println!("   Source: {}", export.metadata.source_scope);
    println!(
        "   Exported: {}",
        export.exported_at.format("%Y-%m-%d %H:%M")
    );

    Ok(())
}

fn share_library(
    rhema: &Rhema,
    library: &str,
    target_scope: &str,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let source_scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let target_scope_path = rhema.find_scope_path(target_scope)?;

    let source_library_path = source_scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));
    let target_library_path = target_scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));

    if !source_library_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Template library '{}' not found",
            library
        )));
    }

    // Create target directory if it doesn't exist
    if let Some(parent) = target_library_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Copy library file
    std::fs::copy(&source_library_path, &target_library_path)?;

    println!(
        "✅ Shared library '{}' with scope '{}'",
        library, target_scope
    );
    println!("   Source: {}", source_library_path.display());
    println!("   Target: {}", target_library_path.display());

    Ok(())
}

fn download_template(
    rhema: &Rhema,
    library: &str,
    template: &str,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let library_path = scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));

    if !library_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Template library '{}' not found",
            library
        )));
    }

    let mut template_library = load_template_library(&library_path)?;

    // Find template
    let template_index = template_library
        .templates
        .iter()
        .position(|t| t.id == template || t.name == template)
        .ok_or_else(|| {
            RhemaError::InvalidCommand(format!(
                "Template '{}' not found in library '{}'",
                template, library
            ))
        })?;

    // Record download
    template_library.templates[template_index]
        .usage_stats
        .record_download();
    save_template_library(&library_path, &template_library)?;

    let template_entry = &template_library.templates[template_index];

    println!(
        "✅ Downloaded template '{}' from library '{}'",
        template_entry.name, library
    );
    println!("   Template ID: {}", template_entry.id);
    println!(
        "   Downloads: {}",
        template_entry.usage_stats.total_downloads
    );
    println!("   Template:");
    println!("{}", template_entry.template);

    Ok(())
}

fn rate_template(
    rhema: &Rhema,
    library: &str,
    template: &str,
    rating: f64,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let library_path = scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));

    if !library_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Template library '{}' not found",
            library
        )));
    }

    let mut template_library = load_template_library(&library_path)?;

    // Find template
    let template_index = template_library
        .templates
        .iter()
        .position(|t| t.id == template || t.name == template)
        .ok_or_else(|| {
            RhemaError::InvalidCommand(format!(
                "Template '{}' not found in library '{}'",
                template, library
            ))
        })?;

    // Validate rating
    if rating < 1.0 || rating > 5.0 {
        return Err(RhemaError::InvalidCommand(
            "Rating must be between 1.0 and 5.0".to_string(),
        ));
    }

    // Add rating
    template_library.templates[template_index]
        .usage_stats
        .add_rating(rating);
    save_template_library(&library_path, &template_library)?;

    let template_entry = &template_library.templates[template_index];

    println!(
        "✅ Rated template '{}' with {} stars",
        template_entry.name, rating
    );
    println!(
        "   Average rating: {:.1}/5.0",
        template_entry.usage_stats.average_rating.unwrap_or(0.0)
    );
    println!(
        "   Total ratings: {}",
        template_entry.usage_stats.rating_count
    );

    Ok(())
}

fn show_library(rhema: &Rhema, library: &str, scope: &Option<String>) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let library_path = scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));

    if !library_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Template library '{}' not found",
            library
        )));
    }

    let template_library = load_template_library(&library_path)?;

    println!("📚 Template Library: {}", template_library.name);
    println!("{}", "=".repeat(60));
    if let Some(desc) = &template_library.description {
        println!("Description: {}", desc);
    }
    println!("Owner: {}", template_library.owner);
    println!("Version: {}", template_library.version);
    println!(
        "Created: {}",
        template_library.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "Updated: {}",
        template_library.updated_at.format("%Y-%m-%d %H:%M")
    );

    if let Some(access) = &template_library.access_control {
        println!("Public: {}", access.public);
        println!("Read-only: {}", access.read_only);
    }

    if let Some(tags) = &template_library.tags {
        println!("Tags: {}", tags.join(", "));
    }
    println!();

    println!("📋 Templates ({}):", template_library.templates.len());
    println!("{}", "-".repeat(40));

    for template in &template_library.templates {
        println!("• {}", template.name);
        if let Some(desc) = &template.description {
            println!("  Description: {}", desc);
        }
        println!("  ID: {}", template.id);
        println!("  Version: {}", template.metadata.version);
        println!("  Downloads: {}", template.usage_stats.total_downloads);
        println!("  Uses: {}", template.usage_stats.total_uses);
        if let Some(rating) = template.usage_stats.average_rating {
            println!(
                "  Rating: {:.1}/5.0 ({} ratings)",
                rating, template.usage_stats.rating_count
            );
        }
        if let Some(category) = &template.metadata.category {
            println!("  Category: {}", category);
        }
        if let Some(complexity) = &template.metadata.complexity {
            println!("  Complexity: {:?}", complexity);
        }
        println!();
    }

    Ok(())
}

fn show_template(
    rhema: &Rhema,
    library: &str,
    template: &str,
    scope: &Option<String>,
) -> RhemaResult<()> {
    let scope_path = if let Some(scope_name) = scope {
        rhema.find_scope_path(scope_name)?
    } else {
        rhema.get_current_scope_path()?
    };

    let library_path = scope_path
        .join(".rhema")
        .join("template-libraries")
        .join(format!("{}.yaml", library));

    if !library_path.exists() {
        return Err(RhemaError::InvalidCommand(format!(
            "Template library '{}' not found",
            library
        )));
    }

    let template_library = load_template_library(&library_path)?;

    // Find template
    let template_entry = template_library
        .templates
        .iter()
        .find(|t| t.id == template || t.name == template)
        .ok_or_else(|| {
            RhemaError::InvalidCommand(format!(
                "Template '{}' not found in library '{}'",
                template, library
            ))
        })?;

    println!("📝 Template: {}", template_entry.name);
    println!("{}", "=".repeat(60));
    println!("ID: {}", template_entry.id);
    if let Some(desc) = &template_entry.description {
        println!("Description: {}", desc);
    }
    println!("Version: {}", template_entry.metadata.version);
    println!(
        "Created: {}",
        template_entry.metadata.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "Updated: {}",
        template_entry.metadata.updated_at.format("%Y-%m-%d %H:%M")
    );

    if let Some(author) = &template_entry.metadata.author {
        println!("Author: {}", author);
    }
    if let Some(category) = &template_entry.metadata.category {
        println!("Category: {}", category);
    }
    if let Some(complexity) = &template_entry.metadata.complexity {
        println!("Complexity: {:?}", complexity);
    }
    if let Some(language) = &template_entry.metadata.language {
        println!("Language: {}", language);
    }
    if let Some(deps) = &template_entry.metadata.dependencies {
        println!("Dependencies: {}", deps.join(", "));
    }
    if let Some(tags) = &template_entry.tags {
        println!("Tags: {}", tags.join(", "));
    }
    println!();

    println!("📊 Usage Statistics:");
    println!(
        "   Downloads: {}",
        template_entry.usage_stats.total_downloads
    );
    println!("   Uses: {}", template_entry.usage_stats.total_uses);
    if let Some(rating) = template_entry.usage_stats.average_rating {
        println!("   Average rating: {:.1}/5.0", rating);
        println!(
            "   Rating count: {}",
            template_entry.usage_stats.rating_count
        );
    }
    if let Some(last_downloaded) = template_entry.usage_stats.last_downloaded {
        println!(
            "   Last downloaded: {}",
            last_downloaded.format("%Y-%m-%d %H:%M")
        );
    }
    if let Some(last_used) = template_entry.usage_stats.last_used {
        println!("   Last used: {}", last_used.format("%Y-%m-%d %H:%M"));
    }
    println!();

    if let Some(examples) = &template_entry.metadata.examples {
        println!("📖 Examples:");
        for (i, example) in examples.iter().enumerate() {
            println!("   {}. {}", i + 1, example);
        }
        println!();
    }

    println!("📄 Template Content:");
    println!("{}", "-".repeat(40));
    println!("{}", template_entry.template);

    Ok(())
}
