package ai.fugue.rhema.actions;

import com.intellij.codeInsight.intention.IntentionAction;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.util.IncorrectOperationException;
import org.jetbrains.annotations.NotNull;

/**
 * Refactor intention for Rhema files.
 * Provides refactoring actions for Rhema elements.
 */
public class RhemaRefactorIntention implements IntentionAction {
    
    @NotNull
    @Override
    public String getText() {
        return "Rhema Refactor";
    }
    
    @NotNull
    @Override
    public String getFamilyName() {
        return "Rhema";
    }
    
    @Override
    public boolean isAvailable(@NotNull Project project, Editor editor, PsiFile file) {
        // TODO: Check if refactor is available
        // This would involve:
        // - Checking if the file is a Rhema file
        // - Checking if there are refactorable elements
        // - Determining what refactoring options are available
        
        return isRhemaFile(file);
    }
    
    @Override
    public void invoke(@NotNull Project project, Editor editor, PsiFile file) throws IncorrectOperationException {
        // TODO: Implement refactor
        // This would involve:
        // - Analyzing the selected element
        // - Providing refactoring options
        // - Applying the selected refactoring
        
        System.out.println("Rhema: Applying refactor");
    }
    
    @Override
    public boolean startInWriteAction() {
        return true;
    }
    
    /**
     * Check if a file is a Rhema file.
     */
    private boolean isRhemaFile(PsiFile file) {
        String name = file.getName();
        return name.endsWith(".rhema.yml") || 
               name.endsWith(".rhema.yaml") || 
               name.equals("rhema.yml") || 
               name.equals("rhema.yaml");
    }
} 