package ai.fugue.rhema.refactoring;

import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.psi.search.GlobalSearchScope;
import com.intellij.psi.search.PsiShortNamesCache;
import com.intellij.refactoring.rename.RenamePsiElementProcessor;
import com.intellij.refactoring.rename.RenameRefactoring;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.ArrayList;
import java.util.List;

/**
 * Rename processor for Rhema elements.
 * Handles renaming of Rhema-specific elements.
 */
public class RhemaRenameProcessor extends RenamePsiElementProcessor {
    
    @Override
    public boolean canProcessElement(@NotNull PsiElement element) {
        return isRhemaElement(element);
    }
    
    @Override
    public void prepareRenaming(@NotNull PsiElement element, @NotNull String newName, @NotNull RenameRefactoring refactoring) {
        // Find all references to this Rhema element
        List<PsiElement> references = findRhemaReferences(element);
        
        // Add all references to the refactoring
        for (PsiElement reference : references) {
            refactoring.addElement(reference);
        }
        
        super.prepareRenaming(element, newName, refactoring);
    }
    
    @Override
    public String getQualifiedNameAfterRename(@NotNull PsiElement element, @NotNull String newName, boolean nonCode) {
        // For Rhema elements, the qualified name is just the new name
        return newName;
    }
    
    @Override
    public PsiElement substituteElementToRename(@NotNull PsiElement element, @Nullable Editor editor) {
        // For Rhema elements, we want to rename the name field
        return findNameField(element);
    }
    
    /**
     * Check if an element is a Rhema element.
     */
    private boolean isRhemaElement(PsiElement element) {
        // Check if this is a Rhema file
        if (element instanceof PsiFile) {
            PsiFile file = (PsiFile) element;
            return isRhemaFile(file);
        }
        
        // Check if this element contains Rhema keywords
        String text = element.getText();
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
     * Check if a file is a Rhema file.
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
     * Find all references to a Rhema element.
     */
    private List<PsiElement> findRhemaReferences(PsiElement element) {
        List<PsiElement> references = new ArrayList<>();
        
        if (element instanceof PsiFile) {
            PsiFile file = (PsiFile) element;
            String elementName = extractElementName(file);
            
            if (elementName != null) {
                // Search for references to this element in other Rhema files
                references.addAll(findReferencesToElement(elementName, file.getProject()));
            }
        }
        
        return references;
    }
    
    /**
     * Extract the name of a Rhema element from a file.
     */
    private String extractElementName(PsiFile file) {
        String content = file.getText();
        if (content == null) {
            return null;
        }
        
        // Look for name field in the content
        String[] lines = content.split("\n");
        for (String line : lines) {
            line = line.trim();
            if (line.startsWith("name:")) {
                String[] parts = line.split(":");
                if (parts.length > 1) {
                    return parts[1].trim();
                }
            }
        }
        
        return null;
    }
    
    /**
     * Find references to an element in the project.
     */
    private List<PsiElement> findReferencesToElement(String elementName, Project project) {
        List<PsiElement> references = new ArrayList<>();
        
        // Search for Rhema files in the project
        PsiShortNamesCache cache = PsiShortNamesCache.getInstance(project);
        GlobalSearchScope scope = GlobalSearchScope.projectScope(project);
        
        // Look for files with Rhema extensions
        String[] extensions = {".rhema.yml", ".scope.yml", ".context.yml", ".todos.yml", ".insights.yml", ".patterns.yml", ".decisions.yml"};
        
        for (String extension : extensions) {
            PsiFile[] files = cache.getFilesByName("*" + extension, scope);
            for (PsiFile file : files) {
                if (containsReference(file, elementName)) {
                    references.add(file);
                }
            }
        }
        
        return references;
    }
    
    /**
     * Check if a file contains a reference to an element.
     */
    private boolean containsReference(PsiFile file, String elementName) {
        String content = file.getText();
        if (content == null) {
            return false;
        }
        
        // Look for the element name in the file
        return content.contains("name: " + elementName) ||
               content.contains("name:" + elementName) ||
               content.contains("scope: " + elementName) ||
               content.contains("scope:" + elementName) ||
               content.contains("context: " + elementName) ||
               content.contains("context:" + elementName);
    }
    
    /**
     * Find the name field in a Rhema element.
     */
    private PsiElement findNameField(PsiElement element) {
        // For Rhema elements, we want to find the name field
        if (element instanceof PsiFile) {
            PsiFile file = (PsiFile) element;
            String content = file.getText();
            
            if (content != null) {
                String[] lines = content.split("\n");
                for (int i = 0; i < lines.length; i++) {
                    String line = lines[i];
                    if (line.trim().startsWith("name:")) {
                        // Return the file itself, but we'll focus on the name field
                        return file;
                    }
                }
            }
        }
        
        return element;
    }
} 