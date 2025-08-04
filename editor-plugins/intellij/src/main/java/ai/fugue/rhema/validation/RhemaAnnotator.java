package ai.fugue.rhema.validation;

import com.intellij.lang.annotation.AnnotationHolder;
import com.intellij.lang.annotation.Annotator;
import com.intellij.psi.PsiElement;
import org.jetbrains.annotations.NotNull;

/**
 * Annotator for Rhema YAML files.
 * Provides validation and error highlighting for Rhema-specific syntax.
 */
public class RhemaAnnotator implements Annotator {
    
    @Override
    public void annotate(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        // TODO: Implement Rhema-specific validation
        // This would include:
        // - Schema validation
        // - Syntax validation
        // - Semantic validation
        // - Cross-reference validation
        
        validateRhemaElement(element, holder);
    }
    
    /**
     * Validate a Rhema element.
     */
    private void validateRhemaElement(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        try {
            String text = element.getText();
            if (text == null || text.trim().isEmpty()) {
                return;
            }
            
            // Validate Rhema-specific syntax
            validateRhemaSyntax(element, holder);
            
            // Validate YAML structure
            validateYamlStructure(element, holder);
            
            // Validate Rhema schema
            validateRhemaSchema(element, holder);
            
        } catch (Exception e) {
            // Log validation errors but don't crash the IDE
            holder.createErrorAnnotation(element, "Rhema validation error: " + e.getMessage());
        }
    }
    
    /**
     * Validate Rhema-specific syntax.
     */
    private void validateRhemaSyntax(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Validate required fields
        validateRequiredFields(element, holder);
        
        // Validate field types
        validateFieldTypes(element, holder);
        
        // Validate value constraints
        validateValueConstraints(element, holder);
        
        // Validate cross-references
        validateCrossReferences(element, holder);
    }
    
    /**
     * Validate YAML structure.
     */
    private void validateYamlStructure(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Check for basic YAML syntax issues
        if (text.contains(":") && !text.contains(" ")) {
            holder.createWarningAnnotation(element, "YAML key-value pairs should have a space after the colon");
        }
        
        // Check for proper indentation
        if (text.startsWith(" ") && !text.startsWith("  ")) {
            holder.createWarningAnnotation(element, "YAML should use consistent indentation (2 spaces recommended)");
        }
    }
    
    /**
     * Validate Rhema schema.
     */
    private void validateRhemaSchema(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Validate scope structure
        if (text.contains("scope:")) {
            validateScopeStructure(element, holder);
        }
        
        // Validate context structure
        if (text.contains("context:")) {
            validateContextStructure(element, holder);
        }
        
        // Validate todos structure
        if (text.contains("todos:")) {
            validateTodosStructure(element, holder);
        }
    }
    
    /**
     * Validate required fields.
     */
    private void validateRequiredFields(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Check for required fields based on document type
        if (text.contains("scope:") && !text.contains("name:")) {
            holder.createErrorAnnotation(element, "Scope documents must contain a 'name' field");
        }
        
        if (text.contains("context:") && !text.contains("name:")) {
            holder.createErrorAnnotation(element, "Context documents must contain a 'name' field");
        }
    }
    
    /**
     * Validate field types.
     */
    private void validateFieldTypes(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Validate version format
        if (text.contains("version:") && !text.matches(".*version:\\s*\\d+\\.\\d+\\.\\d+.*")) {
            holder.createWarningAnnotation(element, "Version should follow semantic versioning format (e.g., 1.0.0)");
        }
    }
    
    /**
     * Validate value constraints.
     */
    private void validateValueConstraints(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Validate name constraints
        if (text.contains("name:") && text.matches(".*name:\\s*[A-Z].*")) {
            holder.createWarningAnnotation(element, "Names should be lowercase with hyphens or underscores");
        }
    }
    
    /**
     * Validate cross-references.
     */
    private void validateCrossReferences(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        // TODO: Implement cross-reference validation
        // This would check if referenced scopes, contexts, etc. exist
    }
    
    /**
     * Validate scope structure.
     */
    private void validateScopeStructure(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Check for required scope fields
        if (text.contains("scope:") && !text.contains("description:")) {
            holder.createWarningAnnotation(element, "Scope should include a description");
        }
    }
    
    /**
     * Validate context structure.
     */
    private void validateContextStructure(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Check for required context fields
        if (text.contains("context:") && !text.contains("description:")) {
            holder.createWarningAnnotation(element, "Context should include a description");
        }
    }
    
    /**
     * Validate todos structure.
     */
    private void validateTodosStructure(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        String text = element.getText();
        
        // Check for required todos fields
        if (text.contains("todos:") && !text.contains("items:")) {
            holder.createWarningAnnotation(element, "Todos should include items list");
        }
    }
} 