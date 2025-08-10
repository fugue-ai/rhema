package ai.fugue.rhema.documentation;

import com.intellij.lang.documentation.AbstractDocumentationProvider;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.util.text.StringUtil;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.psi.PsiManager;
import com.intellij.psi.util.PsiTreeUtil;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Documentation provider for Rhema IntelliJ plugin.
 * Provides inline help, context-sensitive documentation, and interactive tutorials.
 */
public class RhemaDocumentationProvider extends AbstractDocumentationProvider {
    
    private static final Logger LOG = Logger.getInstance(RhemaDocumentationProvider.class);
    
    // Documentation cache for performance
    private final Map<String, String> documentationCache = new ConcurrentHashMap<>();
    
    // Tutorial system
    private final Map<String, RhemaTutorial> tutorials = new ConcurrentHashMap<>();
    
    // Context-sensitive help
    private final Map<String, String> contextHelp = new ConcurrentHashMap<>();
    
    public RhemaDocumentationProvider() {
        initializeDocumentation();
        initializeTutorials();
        initializeContextHelp();
    }
    
    /**
     * Initialize comprehensive documentation.
     */
    private void initializeDocumentation() {
        // Rhema document types documentation
        documentationCache.put("scope", createScopeDocumentation());
        documentationCache.put("context", createContextDocumentation());
        documentationCache.put("todos", createTodosDocumentation());
        documentationCache.put("insights", createInsightsDocumentation());
        documentationCache.put("patterns", createPatternsDocumentation());
        documentationCache.put("decisions", createDecisionsDocumentation());
        
        // Rhema field documentation
        documentationCache.put("name", createNameFieldDocumentation());
        documentationCache.put("description", createDescriptionFieldDocumentation());
        documentationCache.put("type", createTypeFieldDocumentation());
        documentationCache.put("priority", createPriorityFieldDocumentation());
        documentationCache.put("status", createStatusFieldDocumentation());
        documentationCache.put("tags", createTagsFieldDocumentation());
        documentationCache.put("dependencies", createDependenciesFieldDocumentation());
        documentationCache.put("boundaries", createBoundariesFieldDocumentation());
        documentationCache.put("responsibilities", createResponsibilitiesFieldDocumentation());
        documentationCache.put("tech", createTechFieldDocumentation());
        
        // Rhema enum values documentation
        documentationCache.put("repository", createRepositoryTypeDocumentation());
        documentationCache.put("service", createServiceTypeDocumentation());
        documentationCache.put("application", createApplicationTypeDocumentation());
        documentationCache.put("library", createLibraryTypeDocumentation());
        documentationCache.put("component", createComponentTypeDocumentation());
        documentationCache.put("low", createLowPriorityDocumentation());
        documentationCache.put("medium", createMediumPriorityDocumentation());
        documentationCache.put("high", createHighPriorityDocumentation());
        documentationCache.put("critical", createCriticalPriorityDocumentation());
        documentationCache.put("todo", createTodoStatusDocumentation());
        documentationCache.put("in_progress", createInProgressStatusDocumentation());
        documentationCache.put("blocked", createBlockedStatusDocumentation());
        documentationCache.put("review", createReviewStatusDocumentation());
        documentationCache.put("done", createDoneStatusDocumentation());
        
        LOG.info("Initialized " + documentationCache.size() + " documentation entries");
    }
    
    /**
     * Initialize interactive tutorials.
     */
    private void initializeTutorials() {
        // Getting started tutorial
        tutorials.put("getting_started", new RhemaTutorial(
            "Getting Started with Rhema",
            "Learn the basics of Rhema and how to use it in your projects",
            new String[]{
                "Introduction to Rhema",
                "Creating your first scope",
                "Adding context and insights",
                "Managing todos and decisions",
                "Using patterns effectively"
            }
        ));
        
        // Advanced features tutorial
        tutorials.put("advanced_features", new RhemaTutorial(
            "Advanced Rhema Features",
            "Master advanced Rhema features for complex projects",
            new String[]{
                "Cross-scope dependencies",
                "Context inheritance",
                "Pattern composition",
                "Decision tracking",
                "Git integration"
            }
        ));
        
        // Best practices tutorial
        tutorials.put("best_practices", new RhemaTutorial(
            "Rhema Best Practices",
            "Learn best practices for effective Rhema usage",
            new String[]{
                "Scope organization",
                "Context management",
                "Todo prioritization",
                "Pattern documentation",
                "Team collaboration"
            }
        ));
        
        LOG.info("Initialized " + tutorials.size() + " tutorials");
    }
    
    /**
     * Initialize context-sensitive help.
     */
    private void initializeContextHelp() {
        // Context-sensitive help for different scenarios
        contextHelp.put("scope_creation", "Create a new scope to define a bounded context for your project or component.");
        contextHelp.put("context_definition", "Define context to capture important information and insights about your project.");
        contextHelp.put("todo_management", "Manage todos to track tasks, bugs, and improvements in your project.");
        contextHelp.put("pattern_documentation", "Document patterns to capture reusable solutions and architectural decisions.");
        contextHelp.put("decision_tracking", "Track decisions to maintain a record of important architectural and design choices.");
        contextHelp.put("validation_errors", "Fix validation errors to ensure your Rhema documents are properly structured.");
        contextHelp.put("git_integration", "Use Git integration to manage Rhema files across branches and collaborate with your team.");
        
        LOG.info("Initialized " + contextHelp.size() + " context help entries");
    }
    
    @Override
    public String generateDoc(PsiElement element, @Nullable PsiElement originalElement) {
        if (element == null) {
            return null;
        }
        
        try {
            // Get documentation based on element type
            String documentation = getDocumentationForElement(element);
            if (documentation != null) {
                return formatDocumentation(documentation);
            }
            
            // Get context-sensitive help
            String contextHelp = getContextHelpForElement(element);
            if (contextHelp != null) {
                return formatContextHelp(contextHelp);
            }
            
            return null;
        } catch (Exception e) {
            LOG.error("Error generating documentation for element", e);
            return "Error generating documentation: " + e.getMessage();
        }
    }
    
    @Override
    public PsiElement getDocumentationElementForLookupItem(PsiElement element, Object object) {
        // Return the element for which to show documentation
        return element;
    }
    
    @Override
    public PsiElement getDocumentationElementForLink(PsiElement element, String link) {
        // Handle documentation links
        if (link.startsWith("rhema://")) {
            String topic = link.substring(8);
            return createDocumentationElement(element.getProject(), topic);
        }
        return element;
    }
    
    /**
     * Get documentation for a specific element.
     */
    private String getDocumentationForElement(PsiElement element) {
        String elementText = element.getText();
        if (elementText == null) {
            return null;
        }
        
        // Clean up the element text
        String cleanText = elementText.trim().replaceAll("[\"']", "");
        
        // Check documentation cache
        String documentation = documentationCache.get(cleanText);
        if (documentation != null) {
            return documentation;
        }
        
        // Check for Rhema keywords
        if (isRhemaKeyword(cleanText)) {
            return getRhemaKeywordDocumentation(cleanText);
        }
        
        // Check for Rhema field names
        if (isRhemaField(cleanText)) {
            return getRhemaFieldDocumentation(cleanText);
        }
        
        return null;
    }
    
    /**
     * Get context-sensitive help for an element.
     */
    private String getContextHelpForElement(PsiElement element) {
        // Analyze the context and provide relevant help
        String context = analyzeContext(element);
        return contextHelp.get(context);
    }
    
    /**
     * Analyze the context of an element.
     */
    private String analyzeContext(PsiElement element) {
        // Analyze the surrounding context to determine what help to show
        PsiFile file = element.getContainingFile();
        if (file != null) {
            String fileName = file.getName().toLowerCase();
            if (fileName.contains("scope")) {
                return "scope_creation";
            } else if (fileName.contains("context")) {
                return "context_definition";
            } else if (fileName.contains("todos")) {
                return "todo_management";
            } else if (fileName.contains("patterns")) {
                return "pattern_documentation";
            } else if (fileName.contains("decisions")) {
                return "decision_tracking";
            }
        }
        
        return null;
    }
    
    /**
     * Check if text is a Rhema keyword.
     */
    private boolean isRhemaKeyword(String text) {
        return text.equals("scope") || text.equals("context") || text.equals("todos") ||
               text.equals("insights") || text.equals("patterns") || text.equals("decisions") ||
               text.equals("rhema") || text.equals("active") || text.equals("completed");
    }
    
    /**
     * Check if text is a Rhema field.
     */
    private boolean isRhemaField(String text) {
        return text.equals("name") || text.equals("description") || text.equals("type") ||
               text.equals("priority") || text.equals("status") || text.equals("tags") ||
               text.equals("dependencies") || text.equals("boundaries") || text.equals("responsibilities") ||
               text.equals("tech") || text.equals("version") || text.equals("created") ||
               text.equals("updated") || text.equals("author") || text.equals("reviewer");
    }
    
    /**
     * Get Rhema keyword documentation.
     */
    private String getRhemaKeywordDocumentation(String keyword) {
        switch (keyword) {
            case "scope":
                return "A scope defines a bounded context for your project or component. It helps organize and structure your project knowledge.";
            case "context":
                return "Context captures important information, insights, and background knowledge about your project.";
            case "todos":
                return "Todos track tasks, bugs, improvements, and other work items in your project.";
            case "insights":
                return "Insights capture important learnings, observations, and knowledge gained during development.";
            case "patterns":
                return "Patterns document reusable solutions, architectural patterns, and design decisions.";
            case "decisions":
                return "Decisions track important architectural and design choices made during development.";
            case "rhema":
                return "The root element that wraps all Rhema documents and provides version information.";
            case "active":
                return "Active todos are current work items that need to be completed.";
            case "completed":
                return "Completed todos are finished work items that have been accomplished.";
            default:
                return null;
        }
    }
    
    /**
     * Get Rhema field documentation.
     */
    private String getRhemaFieldDocumentation(String field) {
        switch (field) {
            case "name":
                return "The name of the scope, context, todo, insight, pattern, or decision.";
            case "description":
                return "A detailed description explaining the purpose, context, and details.";
            case "type":
                return "The type or category of the element (e.g., repository, service, application).";
            case "priority":
                return "The priority level (low, medium, high, critical) indicating importance.";
            case "status":
                return "The current status (todo, in_progress, blocked, review, done) of a todo item.";
            case "tags":
                return "Tags for categorizing and filtering elements.";
            case "dependencies":
                return "Dependencies on other scopes, components, or external systems.";
            case "boundaries":
                return "Defines what is included and excluded from the scope.";
            case "responsibilities":
                return "The responsibilities and duties of the scope or component.";
            case "tech":
                return "Technical details including languages, frameworks, and tools used.";
            case "version":
                return "Version information for tracking changes and compatibility.";
            case "created":
                return "When the element was created.";
            case "updated":
                return "When the element was last updated.";
            case "author":
                return "Who created or is responsible for the element.";
            case "reviewer":
                return "Who reviewed or approved the element.";
            default:
                return null;
        }
    }
    
    /**
     * Format documentation for display.
     */
    private String formatDocumentation(String documentation) {
        return "<div class='rhema-documentation'>" +
               "<h3>Rhema Documentation</h3>" +
               "<p>" + documentation + "</p>" +
               "<div class='rhema-documentation-footer'>" +
               "<small>Rhema IntelliJ Plugin Documentation</small>" +
               "</div>" +
               "</div>";
    }
    
    /**
     * Format context help for display.
     */
    private String formatContextHelp(String help) {
        return "<div class='rhema-context-help'>" +
               "<h3>Context-Sensitive Help</h3>" +
               "<p>" + help + "</p>" +
               "<div class='rhema-help-footer'>" +
               "<small>Rhema IntelliJ Plugin Help</small>" +
               "</div>" +
               "</div>";
    }
    
    /**
     * Create a documentation element for links.
     */
    private PsiElement createDocumentationElement(Project project, String topic) {
        // Create a placeholder element for documentation links
        return PsiManager.getInstance(project).findFile(
            project.getBaseDir().getFileSystem().findFileByPath("documentation/" + topic + ".md")
        );
    }
    
    /**
     * Get available tutorials.
     */
    public Map<String, RhemaTutorial> getTutorials() {
        return new HashMap<>(tutorials);
    }
    
    /**
     * Get a specific tutorial.
     */
    public RhemaTutorial getTutorial(String tutorialId) {
        return tutorials.get(tutorialId);
    }
    
    /**
     * Start an interactive tutorial.
     */
    public void startTutorial(String tutorialId, Project project) {
        RhemaTutorial tutorial = tutorials.get(tutorialId);
        if (tutorial != null) {
            LOG.info("Starting tutorial: " + tutorial.getTitle());
            // Implementation would show the tutorial UI
            showTutorialUI(tutorial, project);
        }
    }
    
    /**
     * Show tutorial UI.
     */
    private void showTutorialUI(RhemaTutorial tutorial, Project project) {
        // Implementation would create and show the tutorial UI
        LOG.info("Showing tutorial UI for: " + tutorial.getTitle());
    }
    
    /**
     * Get documentation statistics.
     */
    public Map<String, Object> getDocumentationStats() {
        Map<String, Object> stats = new HashMap<>();
        stats.put("documentationEntries", documentationCache.size());
        stats.put("tutorials", tutorials.size());
        stats.put("contextHelpEntries", contextHelp.size());
        return stats;
    }
    
    // Documentation creation methods
    private String createScopeDocumentation() {
        return "A scope defines a bounded context for your project or component. " +
               "It helps organize and structure your project knowledge by establishing clear boundaries " +
               "and responsibilities. Scopes can represent repositories, services, applications, libraries, " +
               "or components within your system.";
    }
    
    private String createContextDocumentation() {
        return "Context captures important information, insights, and background knowledge about your project. " +
               "It provides the necessary understanding for making informed decisions and maintaining " +
               "consistency across your development team.";
    }
    
    private String createTodosDocumentation() {
        return "Todos track tasks, bugs, improvements, and other work items in your project. " +
               "They help maintain a clear overview of what needs to be done and what has been completed. " +
               "Todos can be prioritized and categorized for better organization.";
    }
    
    private String createInsightsDocumentation() {
        return "Insights capture important learnings, observations, and knowledge gained during development. " +
               "They help preserve valuable information that might otherwise be lost and enable " +
               "continuous learning and improvement.";
    }
    
    private String createPatternsDocumentation() {
        return "Patterns document reusable solutions, architectural patterns, and design decisions. " +
               "They provide templates and guidelines for solving common problems and maintaining " +
               "consistency across your codebase.";
    }
    
    private String createDecisionsDocumentation() {
        return "Decisions track important architectural and design choices made during development. " +
               "They provide a record of why certain decisions were made and help maintain " +
               "consistency and understanding across the team.";
    }
    
    private String createNameFieldDocumentation() {
        return "The name field provides a clear, descriptive identifier for the element. " +
               "It should be concise but descriptive enough to understand the purpose at a glance.";
    }
    
    private String createDescriptionFieldDocumentation() {
        return "The description field provides detailed information about the element's purpose, " +
               "context, and implementation details. It should be comprehensive enough to " +
               "provide full understanding of the element.";
    }
    
    private String createTypeFieldDocumentation() {
        return "The type field categorizes the element into predefined types such as repository, " +
               "service, application, library, or component. This helps organize and filter elements.";
    }
    
    private String createPriorityFieldDocumentation() {
        return "The priority field indicates the importance of the element, with values from " +
               "low to critical. This helps in planning and resource allocation.";
    }
    
    private String createStatusFieldDocumentation() {
        return "The status field tracks the current state of a todo item, from todo to done. " +
               "This provides visibility into progress and helps with project management.";
    }
    
    private String createTagsFieldDocumentation() {
        return "Tags provide additional categorization and filtering capabilities. " +
               "They can be used to group related elements or mark them for specific purposes.";
    }
    
    private String createDependenciesFieldDocumentation() {
        return "Dependencies define relationships with other scopes, components, or external systems. " +
               "This helps understand the impact of changes and manage complexity.";
    }
    
    private String createBoundariesFieldDocumentation() {
        return "Boundaries define what is included and excluded from the scope. " +
               "This helps establish clear limits and responsibilities.";
    }
    
    private String createResponsibilitiesFieldDocumentation() {
        return "Responsibilities define what the scope or component is responsible for. " +
               "This helps clarify roles and avoid duplication of effort.";
    }
    
    private String createTechFieldDocumentation() {
        return "The tech field documents the technical details including programming languages, " +
               "frameworks, databases, and tools used in the implementation.";
    }
    
    // Enum value documentation
    private String createRepositoryTypeDocumentation() {
        return "Repository scope - for version control repositories and codebases.";
    }
    
    private String createServiceTypeDocumentation() {
        return "Service scope - for microservices, APIs, and backend services.";
    }
    
    private String createApplicationTypeDocumentation() {
        return "Application scope - for end-user applications and frontend systems.";
    }
    
    private String createLibraryTypeDocumentation() {
        return "Library scope - for reusable code libraries and frameworks.";
    }
    
    private String createComponentTypeDocumentation() {
        return "Component scope - for UI components, modules, and reusable parts.";
    }
    
    private String createLowPriorityDocumentation() {
        return "Low priority - nice to have but not critical for current goals.";
    }
    
    private String createMediumPriorityDocumentation() {
        return "Medium priority - important but not urgent for current goals.";
    }
    
    private String createHighPriorityDocumentation() {
        return "High priority - important and urgent for current goals.";
    }
    
    private String createCriticalPriorityDocumentation() {
        return "Critical priority - blocking progress and requires immediate attention.";
    }
    
    private String createTodoStatusDocumentation() {
        return "Todo status - work item is planned but not yet started.";
    }
    
    private String createInProgressStatusDocumentation() {
        return "In progress status - work item is currently being worked on.";
    }
    
    private String createBlockedStatusDocumentation() {
        return "Blocked status - work item is blocked by dependencies or issues.";
    }
    
    private String createReviewStatusDocumentation() {
        return "Review status - work item is completed and ready for review.";
    }
    
    private String createDoneStatusDocumentation() {
        return "Done status - work item is completed and approved.";
    }
    
    /**
     * Tutorial class for interactive tutorials.
     */
    public static class RhemaTutorial {
        private final String title;
        private final String description;
        private final String[] steps;
        
        public RhemaTutorial(String title, String description, String[] steps) {
            this.title = title;
            this.description = description;
            this.steps = steps;
        }
        
        public String getTitle() {
            return title;
        }
        
        public String getDescription() {
            return description;
        }
        
        public String[] getSteps() {
            return steps;
        }
    }
}