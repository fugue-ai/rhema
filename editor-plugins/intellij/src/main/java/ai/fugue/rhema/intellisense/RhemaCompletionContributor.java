package ai.fugue.rhema.intellisense;

import com.intellij.codeInsight.completion.*;
import com.intellij.codeInsight.lookup.LookupElementBuilder;
import com.intellij.patterns.PlatformPatterns;
import com.intellij.util.ProcessingContext;
import org.jetbrains.annotations.NotNull;

/**
 * Completion contributor for Rhema YAML files.
 * Provides IntelliSense functionality for Rhema-specific completions.
 */
public class RhemaCompletionContributor extends CompletionContributor {
    
    public RhemaCompletionContributor() {
        // Register completion providers for Rhema YAML files
        extend(CompletionType.BASIC, 
               PlatformPatterns.psiElement().withLanguage(RhemaLanguageSupport.INSTANCE),
               new RhemaCompletionProvider());
    }
    
    /**
     * Completion provider for Rhema YAML files.
     */
    private static class RhemaCompletionProvider extends CompletionProvider<CompletionParameters> {
        
        @Override
        protected void addCompletions(@NotNull CompletionParameters parameters,
                                    @NotNull ProcessingContext context,
                                    @NotNull CompletionResultSet result) {
            
            // Add Rhema-specific keywords and values
            result.addElement(LookupElementBuilder.create("scope")
                    .withTypeText("Rhema scope definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of scope keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("context")
                    .withTypeText("Rhema context definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of context keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("todos")
                    .withTypeText("Rhema todos definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of todos keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("insights")
                    .withTypeText("Rhema insights definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of insights keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("patterns")
                    .withTypeText("Rhema patterns definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of patterns keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("decisions")
                    .withTypeText("Rhema decisions definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of decisions keyword
                    }));
            
            // Add more Rhema-specific completions
            addRhemaCompletions(result);
        }
        
        /**
         * Add Rhema-specific completions.
         */
        private void addRhemaCompletions(@NotNull CompletionResultSet result) {
            // Add Rhema document types
            addDocumentTypeCompletions(result);
            
            // Add Rhema field completions
            addFieldCompletions(result);
            
            // Add Rhema value completions
            addValueCompletions(result);
            
            // Add Rhema snippet completions
            addSnippetCompletions(result);
            
            // Add Rhema-specific keywords and values
            addRhemaKeywords(result);
            
            // Add Rhema enum values
            addRhemaEnumValues(result);
            
            // Add AI-powered intelligent completions
            addAIPoweredCompletions(result);
            
            // Add semantic analysis completions
            addSemanticCompletions(result);
        }
        
        /**
         * AI-powered intelligent completions based on context.
         */
        private void addAIPoweredCompletions(@NotNull CompletionResultSet result) {
            // Context-aware completions based on current document type
            String documentType = getCurrentDocumentType();
            if (documentType != null) {
                addContextAwareCompletions(result, documentType);
            }
            
            // Project-aware completions based on existing Rhema files
            addProjectAwareCompletions(result);
            
            // Pattern-based completions based on common Rhema patterns
            addPatternBasedCompletions(result);
            
            // Semantic completions based on content analysis
            addSemanticAnalysisCompletions(result);
        }
        
        /**
         * Get the current document type based on context.
         */
        private String getCurrentDocumentType() {
            // This would analyze the current file content and context
            // to determine the document type
            return null; // Placeholder
        }
        
        /**
         * Add context-aware completions based on document type.
         */
        private void addContextAwareCompletions(@NotNull CompletionResultSet result, String documentType) {
            switch (documentType) {
                case "scope":
                    addScopeSpecificCompletions(result);
                    break;
                case "todos":
                    addTodosSpecificCompletions(result);
                    break;
                case "context":
                    addContextSpecificCompletions(result);
                    break;
                case "insights":
                    addInsightsSpecificCompletions(result);
                    break;
                case "patterns":
                    addPatternsSpecificCompletions(result);
                    break;
                case "decisions":
                    addDecisionsSpecificCompletions(result);
                    break;
            }
        }
        
        /**
         * Add scope-specific intelligent completions.
         */
        private void addScopeSpecificCompletions(@NotNull CompletionResultSet result) {
            // Suggest common scope types based on project structure
            result.addElement(LookupElementBuilder.create("repository")
                    .withTypeText("Repository scope - for version control repositories")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with proper structure
                    }));
            
            result.addElement(LookupElementBuilder.create("service")
                    .withTypeText("Service scope - for microservices and APIs")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with proper structure
                    }));
            
            result.addElement(LookupElementBuilder.create("application")
                    .withTypeText("Application scope - for end-user applications")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with proper structure
                    }));
            
            result.addElement(LookupElementBuilder.create("library")
                    .withTypeText("Library scope - for reusable code libraries")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with proper structure
                    }));
            
            result.addElement(LookupElementBuilder.create("component")
                    .withTypeText("Component scope - for UI components and modules")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with proper structure
                    }));
        }
        
        /**
         * Add todos-specific intelligent completions.
         */
        private void addTodosSpecificCompletions(@NotNull CompletionResultSet result) {
            // Suggest common todo patterns based on project context
            result.addElement(LookupElementBuilder.create("bug_fix")
                    .withTypeText("Bug fix task")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with bug fix template
                    }));
            
            result.addElement(LookupElementBuilder.create("feature")
                    .withTypeText("Feature implementation task")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with feature template
                    }));
            
            result.addElement(LookupElementBuilder.create("refactor")
                    .withTypeText("Code refactoring task")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with refactor template
                    }));
            
            result.addElement(LookupElementBuilder.create("test")
                    .withTypeText("Testing task")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with test template
                    }));
            
            result.addElement(LookupElementBuilder.create("documentation")
                    .withTypeText("Documentation task")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with documentation template
                    }));
        }
        
        /**
         * Add context-specific intelligent completions.
         */
        private void addContextSpecificCompletions(@NotNull CompletionResultSet result) {
            // Suggest common context patterns
            result.addElement(LookupElementBuilder.create("architecture")
                    .withTypeText("Architecture context")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with architecture template
                    }));
            
            result.addElement(LookupElementBuilder.create("business_logic")
                    .withTypeText("Business logic context")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with business logic template
                    }));
            
            result.addElement(LookupElementBuilder.create("data_model")
                    .withTypeText("Data model context")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with data model template
                    }));
            
            result.addElement(LookupElementBuilder.create("api_design")
                    .withTypeText("API design context")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with API design template
                    }));
        }
        
        /**
         * Add insights-specific intelligent completions.
         */
        private void addInsightsSpecificCompletions(@NotNull CompletionResultSet result) {
            // Suggest common insight patterns
            result.addElement(LookupElementBuilder.create("performance")
                    .withTypeText("Performance insight")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with performance template
                    }));
            
            result.addElement(LookupElementBuilder.create("security")
                    .withTypeText("Security insight")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with security template
                    }));
            
            result.addElement(LookupElementBuilder.create("maintainability")
                    .withTypeText("Maintainability insight")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with maintainability template
                    }));
            
            result.addElement(LookupElementBuilder.create("usability")
                    .withTypeText("Usability insight")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with usability template
                    }));
        }
        
        /**
         * Add patterns-specific intelligent completions.
         */
        private void addPatternsSpecificCompletions(@NotNull CompletionResultSet result) {
            // Suggest common design patterns
            result.addElement(LookupElementBuilder.create("singleton")
                    .withTypeText("Singleton pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with singleton template
                    }));
            
            result.addElement(LookupElementBuilder.create("factory")
                    .withTypeText("Factory pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with factory template
                    }));
            
            result.addElement(LookupElementBuilder.create("observer")
                    .withTypeText("Observer pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with observer template
                    }));
            
            result.addElement(LookupElementBuilder.create("strategy")
                    .withTypeText("Strategy pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with strategy template
                    }));
        }
        
        /**
         * Add decisions-specific intelligent completions.
         */
        private void addDecisionsSpecificCompletions(@NotNull CompletionResultSet result) {
            // Suggest common decision patterns
            result.addElement(LookupElementBuilder.create("technology_choice")
                    .withTypeText("Technology choice decision")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with technology choice template
                    }));
            
            result.addElement(LookupElementBuilder.create("architecture_decision")
                    .withTypeText("Architecture decision")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with architecture decision template
                    }));
            
            result.addElement(LookupElementBuilder.create("design_decision")
                    .withTypeText("Design decision")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with design decision template
                    }));
            
            result.addElement(LookupElementBuilder.create("process_decision")
                    .withTypeText("Process decision")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with process decision template
                    }));
        }
        
        /**
         * Add project-aware completions based on existing Rhema files.
         */
        private void addProjectAwareCompletions(@NotNull CompletionResultSet result) {
            // This would analyze existing Rhema files in the project
            // and suggest completions based on what's already defined
            
            // For now, add some common project-aware suggestions
            result.addElement(LookupElementBuilder.create("existing_scope")
                    .withTypeText("Reference to existing scope")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with scope reference
                    }));
            
            result.addElement(LookupElementBuilder.create("existing_context")
                    .withTypeText("Reference to existing context")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with context reference
                    }));
            
            result.addElement(LookupElementBuilder.create("existing_pattern")
                    .withTypeText("Reference to existing pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with pattern reference
                    }));
        }
        
        /**
         * Add pattern-based completions based on common Rhema patterns.
         */
        private void addPatternBasedCompletions(@NotNull CompletionResultSet result) {
            // Suggest completions based on common Rhema usage patterns
            
            result.addElement(LookupElementBuilder.create("microservice_pattern")
                    .withTypeText("Microservice architecture pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with microservice pattern
                    }));
            
            result.addElement(LookupElementBuilder.create("monolith_pattern")
                    .withTypeText("Monolithic architecture pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with monolith pattern
                    }));
            
            result.addElement(LookupElementBuilder.create("event_driven_pattern")
                    .withTypeText("Event-driven architecture pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with event-driven pattern
                    }));
            
            result.addElement(LookupElementBuilder.create("layered_pattern")
                    .withTypeText("Layered architecture pattern")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with layered pattern
                    }));
        }
        
        /**
         * Add semantic analysis completions based on content analysis.
         */
        private void addSemanticAnalysisCompletions(@NotNull CompletionResultSet result) {
            // This would analyze the current content and suggest
            // semantically relevant completions
            
            result.addElement(LookupElementBuilder.create("semantic_suggestion")
                    .withTypeText("Semantically relevant suggestion")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with semantic suggestion
                    }));
        }
        
        /**
         * Add semantic completions based on deep content understanding.
         */
        private void addSemanticCompletions(@NotNull CompletionResultSet result) {
            // Cross-document reference completions
            addCrossDocumentCompletions(result);
            
            // Semantic relationship completions
            addSemanticRelationshipCompletions(result);
            
            // Context-aware field completions
            addContextAwareFieldCompletions(result);
        }
        
        /**
         * Add cross-document reference completions.
         */
        private void addCrossDocumentCompletions(@NotNull CompletionResultSet result) {
            // Suggest references to other Rhema documents
            
            result.addElement(LookupElementBuilder.create("ref:scope:")
                    .withTypeText("Reference to scope document")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with scope reference
                    }));
            
            result.addElement(LookupElementBuilder.create("ref:context:")
                    .withTypeText("Reference to context document")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with context reference
                    }));
            
            result.addElement(LookupElementBuilder.create("ref:todos:")
                    .withTypeText("Reference to todos document")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with todos reference
                    }));
            
            result.addElement(LookupElementBuilder.create("ref:insights:")
                    .withTypeText("Reference to insights document")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with insights reference
                    }));
            
            result.addElement(LookupElementBuilder.create("ref:patterns:")
                    .withTypeText("Reference to patterns document")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with patterns reference
                    }));
            
            result.addElement(LookupElementBuilder.create("ref:decisions:")
                    .withTypeText("Reference to decisions document")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with decisions reference
                    }));
        }
        
        /**
         * Add semantic relationship completions.
         */
        private void addSemanticRelationshipCompletions(@NotNull CompletionResultSet result) {
            // Suggest semantic relationships between Rhema elements
            
            result.addElement(LookupElementBuilder.create("depends_on")
                    .withTypeText("Dependency relationship")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with dependency relationship
                    }));
            
            result.addElement(LookupElementBuilder.create("implements")
                    .withTypeText("Implementation relationship")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with implementation relationship
                    }));
            
            result.addElement(LookupElementBuilder.create("extends")
                    .withTypeText("Extension relationship")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with extension relationship
                    }));
            
            result.addElement(LookupElementBuilder.create("composes")
                    .withTypeText("Composition relationship")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with composition relationship
                    }));
        }
        
        /**
         * Add context-aware field completions.
         */
        private void addContextAwareFieldCompletions(@NotNull CompletionResultSet result) {
            // Suggest fields based on the current context and document type
            
            result.addElement(LookupElementBuilder.create("context_aware_field")
                    .withTypeText("Context-aware field suggestion")
                    .withInsertHandler((context, item) -> {
                        // Handle insertion with context-aware field
                    }));
        }
        
        /**
         * Add document type completions.
         */
        private void addDocumentTypeCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("scope")
                    .withTypeText("Rhema scope document")
                    .withInsertHandler((context, item) -> {
                        // Insert scope template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  version: 1.0.0\n  tags: []\n  dependencies: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("context")
                    .withTypeText("Rhema context document")
                    .withInsertHandler((context, item) -> {
                        // Insert context template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  version: 1.0.0\n  tags: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("todos")
                    .withTypeText("Rhema todos document")
                    .withInsertHandler((context, item) -> {
                        // Insert todos template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  items: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("insights")
                    .withTypeText("Rhema insights document")
                    .withInsertHandler((context, item) -> {
                        // Insert insights template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  insights: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("patterns")
                    .withTypeText("Rhema patterns document")
                    .withInsertHandler((context, item) -> {
                        // Insert patterns template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  patterns: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("decisions")
                    .withTypeText("Rhema decisions document")
                    .withInsertHandler((context, item) -> {
                        // Insert decisions template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  decisions: []");
                    }));
        }
        
        /**
         * Add field completions.
         */
        private void addFieldCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("name")
                    .withTypeText("Document name")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("description")
                    .withTypeText("Document description")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("version")
                    .withTypeText("Document version")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": 1.0.0");
                    }));
            
            result.addElement(LookupElementBuilder.create("tags")
                    .withTypeText("Document tags")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("dependencies")
                    .withTypeText("Document dependencies")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("scope")
                    .withTypeText("Parent scope")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
        }
        
        /**
         * Add value completions.
         */
        private void addValueCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("high")
                    .withTypeText("Priority: High")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("medium")
                    .withTypeText("Priority: Medium")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("low")
                    .withTypeText("Priority: Low")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("true")
                    .withTypeText("Boolean: True")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("false")
                    .withTypeText("Boolean: False")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
        }
        
        /**
         * Add snippet completions.
         */
        private void addSnippetCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("rhema-scope")
                    .withTypeText("Complete scope template")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\nscope:\n  name: \n  description: \n  version: 1.0.0\n  tags: []\n  dependencies: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("rhema-context")
                    .withTypeText("Complete context template")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\ncontext:\n  name: \n  description: \n  scope: \n  version: 1.0.0\n  tags: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("rhema-todo")
                    .withTypeText("Complete todo item")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  - title: \n    description: \n    priority: medium\n    status: pending");
                    }));
        }
        
        /**
         * Add Rhema-specific keywords.
         */
        private void addRhemaKeywords(@NotNull CompletionResultSet result) {
            // Rhema wrapper keywords
            result.addElement(LookupElementBuilder.create("rhema")
                    .withTypeText("Rhema document wrapper")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            ":\n  version: 1.0.0\n  ");
                    }));
            
            // Scope-specific keywords
            result.addElement(LookupElementBuilder.create("type")
                    .withTypeText("Scope type")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("boundaries")
                    .withTypeText("Scope boundaries")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            ":\n    includes: []\n    excludes: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("dependencies")
                    .withTypeText("Scope dependencies")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            ":\n    parent: \n    children: []\n    peers: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("responsibilities")
                    .withTypeText("Scope responsibilities")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("tech")
                    .withTypeText("Technology stack")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            ":\n    primary_languages: []\n    frameworks: []\n    databases: []");
                    }));
            
            // Todo-specific keywords
            result.addElement(LookupElementBuilder.create("active")
                    .withTypeText("Active todos")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": {}");
                    }));
            
            result.addElement(LookupElementBuilder.create("completed")
                    .withTypeText("Completed todos")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": {}");
                    }));
            
            result.addElement(LookupElementBuilder.create("title")
                    .withTypeText("Todo title")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("priority")
                    .withTypeText("Todo priority")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("status")
                    .withTypeText("Todo status")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("created")
                    .withTypeText("Creation date")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("context")
                    .withTypeText("Todo context")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            ":\n      related_files: []\n      related_components: []\n      cross_scope_dependencies: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("acceptance_criteria")
                    .withTypeText("Acceptance criteria")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("estimated_effort")
                    .withTypeText("Estimated effort")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("outcome")
                    .withTypeText("Completion outcome")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("impact")
                    .withTypeText("Impact assessment")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("lessons_learned")
                    .withTypeText("Lessons learned")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("knowledge_updated")
                    .withTypeText("Knowledge updates")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("effort_actual")
                    .withTypeText("Actual effort")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
        }
        
        /**
         * Add Rhema enum values.
         */
        private void addRhemaEnumValues(@NotNull CompletionResultSet result) {
            // Scope types
            result.addElement(LookupElementBuilder.create("repository")
                    .withTypeText("Scope type: Repository")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("service")
                    .withTypeText("Scope type: Service")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("application")
                    .withTypeText("Scope type: Application")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("library")
                    .withTypeText("Scope type: Library")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("component")
                    .withTypeText("Scope type: Component")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            // Todo priorities
            result.addElement(LookupElementBuilder.create("critical")
                    .withTypeText("Priority: Critical")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            // Todo statuses
            result.addElement(LookupElementBuilder.create("todo")
                    .withTypeText("Status: Todo")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("in_progress")
                    .withTypeText("Status: In Progress")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("blocked")
                    .withTypeText("Status: Blocked")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("review")
                    .withTypeText("Status: Review")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("done")
                    .withTypeText("Status: Done")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
        }
    }
} 