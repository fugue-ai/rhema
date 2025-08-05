package ai.fugue.rhema;

import ai.fugue.rhema.validation.RhemaSchemaValidator;
import ai.fugue.rhema.intellisense.RhemaCompletionContributor;
import ai.fugue.rhema.navigation.RhemaGotoDeclarationHandler;
import ai.fugue.rhema.refactoring.RhemaRenameProcessor;
import com.intellij.testFramework.fixtures.BasePlatformTestCase;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.codeInsight.completion.CompletionType;
import com.intellij.codeInsight.lookup.LookupElement;
import com.intellij.openapi.editor.Editor;
import com.intellij.psi.PsiElement;
import com.intellij.refactoring.rename.RenameRefactoring;

import java.util.List;

/**
 * Comprehensive test suite for the Rhema IntelliJ plugin.
 */
public class RhemaPluginTestSuite extends BasePlatformTestCase {
    
    @Override
    protected void setUp() throws Exception {
        super.setUp();
    }
    
    @Override
    protected void tearDown() throws Exception {
        super.tearDown();
    }
    
    /**
     * Test schema validation for scope documents.
     */
    public void testScopeValidation() {
        // Create a valid scope document
        String validScope = """
            rhema:
              version: 1.0.0
              scope:
                type: repository
                name: test-scope
                description: A test scope
                boundaries:
                  includes: ["src/**"]
                  excludes: ["**/*.test.*"]
                dependencies:
                  parent: parent-scope
                  children: []
                  peers: []
                responsibilities:
                  - Handle user authentication
                  - Manage data persistence
                tech:
                  primary_languages: ["Rust", "TypeScript"]
                  frameworks: ["Actix", "React"]
                  databases: ["PostgreSQL"]
            """;
        
        PsiFile file = myFixture.configureByText("test.scope.yml", validScope);
        RhemaSchemaValidator validator = new RhemaSchemaValidator();
        RhemaSchemaValidator.ValidationResult result = validator.validateFile(file);
        
        assertFalse("Valid scope should not have errors", result.hasErrors());
    }
    
    /**
     * Test schema validation for invalid scope documents.
     */
    public void testInvalidScopeValidation() {
        // Create an invalid scope document (missing required fields)
        String invalidScope = """
            rhema:
              version: 1.0.0
              scope:
                name: test-scope
                # Missing type and description
            """;
        
        PsiFile file = myFixture.configureByText("test.scope.yml", invalidScope);
        RhemaSchemaValidator validator = new RhemaSchemaValidator();
        RhemaSchemaValidator.ValidationResult result = validator.validateFile(file);
        
        assertTrue("Invalid scope should have errors", result.hasErrors());
        assertTrue("Should have error for missing type", 
            result.getErrors().stream().anyMatch(e -> e.getMessage().contains("type")));
        assertTrue("Should have error for missing description", 
            result.getErrors().stream().anyMatch(e -> e.getMessage().contains("description")));
    }
    
    /**
     * Test schema validation for todos documents.
     */
    public void testTodosValidation() {
        // Create a valid todos document
        String validTodos = """
            active:
              task1:
                title: Implement feature
                description: Add new functionality
                priority: high
                status: in_progress
                created: 2024-01-01
                context:
                  related_files: ["src/main.rs"]
                  related_components: ["auth"]
                  cross_scope_dependencies: []
                acceptance_criteria:
                  - Feature works correctly
                  - Tests pass
                estimated_effort: 2 days
                tags: ["feature", "auth"]
            completed:
              task2:
                title: Fix bug
                description: Resolved critical issue
                completed: 2024-01-02
                outcome: Bug fixed successfully
                impact:
                  - Improved stability
                  - Better user experience
                lessons_learned:
                  - Need better error handling
                knowledge_updated:
                  - Updated error handling guide
                effort_actual: 1 day
                tags: ["bugfix", "stability"]
            """;
        
        PsiFile file = myFixture.configureByText("test.todos.yml", validTodos);
        RhemaSchemaValidator validator = new RhemaSchemaValidator();
        RhemaSchemaValidator.ValidationResult result = validator.validateFile(file);
        
        assertFalse("Valid todos should not have errors", result.hasErrors());
    }
    
    /**
     * Test schema validation for invalid todos documents.
     */
    public void testInvalidTodosValidation() {
        // Create an invalid todos document (missing required fields)
        String invalidTodos = """
            active:
              task1:
                title: Test task
                # Missing priority, status, created
            """;
        
        PsiFile file = myFixture.configureByText("test.todos.yml", invalidTodos);
        RhemaSchemaValidator validator = new RhemaSchemaValidator();
        RhemaSchemaValidator.ValidationResult result = validator.validateFile(file);
        
        assertTrue("Invalid todos should have errors", result.hasErrors());
        assertTrue("Should have error for missing priority", 
            result.getErrors().stream().anyMatch(e -> e.getMessage().contains("priority")));
        assertTrue("Should have error for missing status", 
            result.getErrors().stream().anyMatch(e -> e.getMessage().contains("status")));
        assertTrue("Should have error for missing created", 
            result.getErrors().stream().anyMatch(e -> e.getMessage().contains("created")));
    }
    
    /**
     * Test completion for Rhema keywords.
     */
    public void testRhemaKeywordCompletion() {
        // Create a Rhema file and test completion
        String rhemaContent = """
            rhema:
              version: 1.0.0
              scope:
                name: test-scope
                type: 
            """;
        
        PsiFile file = myFixture.configureByText("test.scope.yml", rhemaContent);
        
        // Position cursor after "type: "
        myFixture.getEditor().getCaretModel().moveToOffset(rhemaContent.length());
        
        // Trigger completion
        myFixture.complete(CompletionType.BASIC);
        List<LookupElement> elements = myFixture.getLookupElements();
        
        assertNotNull("Completion elements should not be null", elements);
        assertTrue("Should have completion elements", elements.size() > 0);
        
        // Check for scope type completions
        boolean hasRepository = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("repository"));
        boolean hasService = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("service"));
        boolean hasApplication = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("application"));
        
        assertTrue("Should have repository completion", hasRepository);
        assertTrue("Should have service completion", hasService);
        assertTrue("Should have application completion", hasApplication);
    }
    
    /**
     * Test completion for Rhema field names.
     */
    public void testRhemaFieldCompletion() {
        // Create a Rhema file and test field completion
        String rhemaContent = """
            rhema:
              version: 1.0.0
              scope:
                name: test-scope
                type: repository
                
            """;
        
        PsiFile file = myFixture.configureByText("test.scope.yml", rhemaContent);
        
        // Position cursor at the end
        myFixture.getEditor().getCaretModel().moveToOffset(rhemaContent.length());
        
        // Trigger completion
        myFixture.complete(CompletionType.BASIC);
        List<LookupElement> elements = myFixture.getLookupElements();
        
        assertNotNull("Completion elements should not be null", elements);
        assertTrue("Should have completion elements", elements.size() > 0);
        
        // Check for field completions
        boolean hasDescription = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("description"));
        boolean hasBoundaries = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("boundaries"));
        boolean hasDependencies = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("dependencies"));
        
        assertTrue("Should have description completion", hasDescription);
        assertTrue("Should have boundaries completion", hasBoundaries);
        assertTrue("Should have dependencies completion", hasDependencies);
    }
    
    /**
     * Test completion for todo status values.
     */
    public void testTodoStatusCompletion() {
        // Create a todos file and test status completion
        String todosContent = """
            active:
              task1:
                title: Test task
                priority: high
                status: 
            """;
        
        PsiFile file = myFixture.configureByText("test.todos.yml", todosContent);
        
        // Position cursor after "status: "
        myFixture.getEditor().getCaretModel().moveToOffset(todosContent.length());
        
        // Trigger completion
        myFixture.complete(CompletionType.BASIC);
        List<LookupElement> elements = myFixture.getLookupElements();
        
        assertNotNull("Completion elements should not be null", elements);
        assertTrue("Should have completion elements", elements.size() > 0);
        
        // Check for status completions
        boolean hasTodo = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("todo"));
        boolean hasInProgress = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("in_progress"));
        boolean hasDone = elements.stream()
            .anyMatch(e -> e.getLookupString().equals("done"));
        
        assertTrue("Should have todo completion", hasTodo);
        assertTrue("Should have in_progress completion", hasInProgress);
        assertTrue("Should have done completion", hasDone);
    }
    
    /**
     * Test navigation to Rhema elements.
     */
    public void testRhemaNavigation() {
        // Create a scope file
        String scopeContent = """
            rhema:
              version: 1.0.0
              scope:
                type: repository
                name: main-scope
                description: Main application scope
            """;
        
        PsiFile scopeFile = myFixture.configureByText("main.scope.yml", scopeContent);
        
        // Create a context file that references the scope
        String contextContent = """
            rhema:
              version: 1.0.0
              context:
                name: auth-context
                description: Authentication context
                scope: main-scope
            """;
        
        PsiFile contextFile = myFixture.configureByText("auth.context.yml", contextContent);
        
        // Position cursor on the scope reference
        myFixture.getEditor().getCaretModel().moveToOffset(contextContent.indexOf("main-scope"));
        
        // Test goto declaration
        PsiElement target = myFixture.getReferenceAtCaretPosition();
        assertNotNull("Should find reference", target);
        
        // Navigate to declaration
        myFixture.performEditorAction("GotoDeclaration");
        
        // Verify we're in the scope file
        assertEquals("Should navigate to scope file", scopeFile, myFixture.getFile());
    }
    
    /**
     * Test refactoring of Rhema elements.
     */
    public void testRhemaRefactoring() {
        // Create a scope file
        String scopeContent = """
            rhema:
              version: 1.0.0
              scope:
                type: repository
                name: old-scope-name
                description: Test scope
            """;
        
        PsiFile scopeFile = myFixture.configureByText("test.scope.yml", scopeContent);
        
        // Create a context file that references the scope
        String contextContent = """
            rhema:
              version: 1.0.0
              context:
                name: test-context
                description: Test context
                scope: old-scope-name
            """;
        
        PsiFile contextFile = myFixture.configureByText("test.context.yml", contextContent);
        
        // Position cursor on the scope name
        myFixture.getEditor().getCaretModel().moveToOffset(scopeContent.indexOf("old-scope-name"));
        
        // Test rename refactoring
        myFixture.performEditorAction("RenameElement");
        
        // Type new name
        myFixture.type("new-scope-name");
        
        // Apply refactoring
        myFixture.performEditorAction("RefactorRename");
        
        // Verify the scope name was changed
        String newScopeContent = scopeFile.getText();
        assertTrue("Scope name should be updated", newScopeContent.contains("new-scope-name"));
        
        // Verify the reference was updated
        String newContextContent = contextFile.getText();
        assertTrue("Context reference should be updated", newContextContent.contains("new-scope-name"));
    }
    
    /**
     * Test file type detection.
     */
    public void testRhemaFileTypeDetection() {
        // Test scope file
        PsiFile scopeFile = myFixture.configureByText("test.scope.yml", "scope: test");
        assertTrue("Should detect scope file", isRhemaFile(scopeFile));
        
        // Test context file
        PsiFile contextFile = myFixture.configureByText("test.context.yml", "context: test");
        assertTrue("Should detect context file", isRhemaFile(contextFile));
        
        // Test todos file
        PsiFile todosFile = myFixture.configureByText("test.todos.yml", "todos: test");
        assertTrue("Should detect todos file", isRhemaFile(todosFile));
        
        // Test regular YAML file (should not be detected as Rhema)
        PsiFile regularFile = myFixture.configureByText("test.yml", "key: value");
        assertFalse("Should not detect regular YAML as Rhema", isRhemaFile(regularFile));
    }
    
    /**
     * Test document type detection.
     */
    public void testDocumentTypeDetection() {
        RhemaSchemaValidator validator = new RhemaSchemaValidator();
        
        // Test scope detection
        String scopeContent = """
            rhema:
              version: 1.0.0
              scope:
                name: test
            """;
        RhemaSchemaValidator.DocumentType scopeType = validator.detectDocumentType(scopeContent);
        assertEquals("Should detect scope type", RhemaSchemaValidator.DocumentType.SCOPE, scopeType);
        
        // Test todos detection
        String todosContent = """
            active:
              task1:
                title: test
            """;
        RhemaSchemaValidator.DocumentType todosType = validator.detectDocumentType(todosContent);
        assertEquals("Should detect todos type", RhemaSchemaValidator.DocumentType.TODOS, todosType);
        
        // Test unknown type
        String unknownContent = "key: value";
        RhemaSchemaValidator.DocumentType unknownType = validator.detectDocumentType(unknownContent);
        assertNull("Should not detect unknown type", unknownType);
    }
    
    /**
     * Helper method to check if a file is a Rhema file.
     */
    private boolean isRhemaFile(PsiFile file) {
        String fileName = file.getName().toLowerCase();
        return fileName.endsWith(".rhema.yml") || 
               fileName.endsWith(".scope.yml") || 
               fileName.endsWith(".context.yml") ||
               fileName.endsWith(".todos.yml") ||
               fileName.endsWith(".insights.yml") ||
               fileName.endsWith(".patterns.yml") ||
               fileName.endsWith(".decisions.yml");
    }
} 