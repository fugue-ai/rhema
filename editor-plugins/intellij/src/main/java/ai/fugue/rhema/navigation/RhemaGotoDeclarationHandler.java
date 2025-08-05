package ai.fugue.rhema.navigation;

import com.intellij.codeInsight.navigation.actions.GotoDeclarationHandler;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.psi.search.GlobalSearchScope;
import com.intellij.psi.search.PsiShortNamesCache;
import org.jetbrains.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;

/**
 * Goto declaration handler for Rhema files.
 * Provides navigation functionality for Rhema-specific elements.
 */
public class RhemaGotoDeclarationHandler implements GotoDeclarationHandler {
    
    @Nullable
    @Override
    public PsiElement[] getGotoDeclarationTargets(@Nullable PsiElement sourceElement, int offset, Editor editor) {
        if (sourceElement == null) {
            return null;
        }
        
        return findRhemaDeclarationTargets(sourceElement, editor.getProject());
    }
    
    /**
     * Find Rhema declaration targets.
     */
    private PsiElement[] findRhemaDeclarationTargets(PsiElement sourceElement, Project project) {
        List<PsiElement> targets = new ArrayList<>();
        
        // Get the text at the cursor position
        String text = sourceElement.getText();
        if (text == null || text.trim().isEmpty()) {
            return null;
        }
        
        // Check if this is a Rhema reference
        RhemaReference reference = parseRhemaReference(text, sourceElement);
        if (reference != null) {
            // Find the referenced element
            PsiElement target = findReferencedElement(reference, project);
            if (target != null) {
                targets.add(target);
            }
        }
        
        return targets.isEmpty() ? null : targets.toArray(new PsiElement[0]);
    }
    
    /**
     * Parse a Rhema reference from text.
     */
    private RhemaReference parseRhemaReference(String text, PsiElement element) {
        // Look for scope references
        if (text.contains("scope:")) {
            return new RhemaReference(RhemaReferenceType.SCOPE, extractReferenceName(text));
        }
        
        // Look for context references
        if (text.contains("context:")) {
            return new RhemaReference(RhemaReferenceType.CONTEXT, extractReferenceName(text));
        }
        
        // Look for todo references
        if (text.contains("todo:")) {
            return new RhemaReference(RhemaReferenceType.TODO, extractReferenceName(text));
        }
        
        // Look for insight references
        if (text.contains("insight:")) {
            return new RhemaReference(RhemaReferenceType.INSIGHT, extractReferenceName(text));
        }
        
        // Look for pattern references
        if (text.contains("pattern:")) {
            return new RhemaReference(RhemaReferenceType.PATTERN, extractReferenceName(text));
        }
        
        // Look for decision references
        if (text.contains("decision:")) {
            return new RhemaReference(RhemaReferenceType.DECISION, extractReferenceName(text));
        }
        
        return null;
    }
    
    /**
     * Extract reference name from text.
     */
    private String extractReferenceName(String text) {
        // Simple extraction - look for the value after the colon
        String[] parts = text.split(":");
        if (parts.length > 1) {
            return parts[1].trim();
        }
        return null;
    }
    
    /**
     * Find the referenced element.
     */
    private PsiElement findReferencedElement(RhemaReference reference, Project project) {
        // Search for Rhema files in the project
        PsiShortNamesCache cache = PsiShortNamesCache.getInstance(project);
        GlobalSearchScope scope = GlobalSearchScope.projectScope(project);
        
        // Look for files with the appropriate extension
        String[] extensions = getExtensionsForType(reference.getType());
        for (String extension : extensions) {
            PsiFile[] files = cache.getFilesByName("*" + extension, scope);
            for (PsiFile file : files) {
                if (containsReference(file, reference)) {
                    return file;
                }
            }
        }
        
        return null;
    }
    
    /**
     * Get file extensions for a reference type.
     */
    private String[] getExtensionsForType(RhemaReferenceType type) {
        switch (type) {
            case SCOPE:
                return new String[]{".scope.yml", ".rhema.yml"};
            case CONTEXT:
                return new String[]{".context.yml", ".rhema.yml"};
            case TODO:
                return new String[]{".todos.yml", ".rhema.yml"};
            case INSIGHT:
                return new String[]{".insights.yml", ".rhema.yml"};
            case PATTERN:
                return new String[]{".patterns.yml", ".rhema.yml"};
            case DECISION:
                return new String[]{".decisions.yml", ".rhema.yml"};
            default:
                return new String[]{".yml"};
        }
    }
    
    /**
     * Check if a file contains the referenced element.
     */
    private boolean containsReference(PsiFile file, RhemaReference reference) {
        String content = file.getText();
        if (content == null) {
            return false;
        }
        
        // Look for the reference name in the file
        return content.contains("name: " + reference.getName()) ||
               content.contains("name:" + reference.getName());
    }
    
    /**
     * Rhema reference types.
     */
    private enum RhemaReferenceType {
        SCOPE, CONTEXT, TODO, INSIGHT, PATTERN, DECISION
    }
    
    /**
     * Rhema reference data.
     */
    private static class RhemaReference {
        private final RhemaReferenceType type;
        private final String name;
        
        public RhemaReference(RhemaReferenceType type, String name) {
            this.type = type;
            this.name = name;
        }
        
        public RhemaReferenceType getType() {
            return type;
        }
        
        public String getName() {
            return name;
        }
    }
} 