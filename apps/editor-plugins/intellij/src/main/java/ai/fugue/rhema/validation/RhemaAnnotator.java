package ai.fugue.rhema.validation;

import com.intellij.lang.annotation.AnnotationHolder;
import com.intellij.lang.annotation.Annotator;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import org.jetbrains.annotations.NotNull;

/**
 * Annotator for Rhema YAML files.
 * Provides validation and error highlighting for Rhema-specific syntax.
 */
public class RhemaAnnotator implements Annotator {
    
    private final RhemaSchemaValidator schemaValidator = new RhemaSchemaValidator();
    
    @Override
    public void annotate(@NotNull PsiElement element, @NotNull AnnotationHolder holder) {
        // Only validate at file level to avoid performance issues
        if (element instanceof PsiFile) {
            PsiFile file = (PsiFile) element;
            if (isRhemaFile(file)) {
                validateRhemaFile(file, holder);
            }
        }
    }
    
    /**
     * Check if the file is a Rhema file.
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
    
    /**
     * Validate a Rhema file using the schema validator.
     */
    private void validateRhemaFile(PsiFile file, AnnotationHolder holder) {
        RhemaSchemaValidator.ValidationResult result = schemaValidator.validateFile(file);
        
        // Add error annotations
        for (RhemaSchemaValidator.ValidationError error : result.getErrors()) {
            holder.createErrorAnnotation(file, error.getMessage());
        }
        
        // Add warning annotations
        for (RhemaSchemaValidator.ValidationWarning warning : result.getWarnings()) {
            holder.createWarningAnnotation(file, warning.getMessage());
        }
    }
    

} 