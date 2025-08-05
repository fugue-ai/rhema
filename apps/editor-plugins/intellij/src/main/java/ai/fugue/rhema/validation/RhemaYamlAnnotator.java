package ai.fugue.rhema.validation;

import com.intellij.lang.annotation.AnnotationHolder;
import com.intellij.lang.annotation.Annotator;
import com.intellij.psi.PsiElement;
import org.jetbrains.annotations.NotNull;

/**
 * Annotator for YAML files with Rhema-specific validation.
 * Provides validation for Rhema keywords and syntax in regular YAML files.
 */
public class RhemaYamlAnnotator implements Annotator {
    
    @Override
    public void annotate(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        // TODO: Implement Rhema-specific validation for YAML files
        // This would include:
        // - Rhema keyword validation
        // - Rhema schema validation
        // - Rhema syntax validation
        
        validateRhemaYamlElement(element, holder);
    }
    
    /**
     * Validate a Rhema element in YAML.
     */
    private void validateRhemaYamlElement(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        try {
            // TODO: Implement Rhema validation for YAML files
            // This would involve detecting Rhema keywords and validating them
            
            // Example validation logic:
            String text = element.getText();
            if (text != null && isRhemaKeyword(text)) {
                // Validate Rhema-specific syntax in YAML
                validateRhemaYamlSyntax(element, holder);
            }
            
        } catch (Exception e) {
            // Log validation errors but don't crash the IDE
            holder.createErrorAnnotation(element, "Rhema YAML validation error: " + e.getMessage());
        }
    }
    
    /**
     * Check if a text contains Rhema keywords.
     */
    private boolean isRhemaKeyword(String text) {
        return text != null && (
            text.contains("rhema") ||
            text.contains("scope") ||
            text.contains("context") ||
            text.contains("todos") ||
            text.contains("insights") ||
            text.contains("patterns") ||
            text.contains("decisions")
        );
    }
    
    /**
     * Validate Rhema-specific syntax in YAML.
     */
    private void validateRhemaYamlSyntax(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        // TODO: Implement Rhema syntax validation for YAML files
        // This would include validation for:
        // - Rhema keyword usage
        // - Rhema field structure
        // - Rhema value constraints
    }
} 