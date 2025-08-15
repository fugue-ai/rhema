package ai.fugue.rhema.actions;

import com.intellij.codeInsight.intention.IntentionAction;
import com.intellij.openapi.editor.Editor;
import com.intellij.openapi.project.Project;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import com.intellij.util.IncorrectOperationException;
import org.jetbrains.annotations.NotNull;

/**
 * Quick fix intention for Rhema files.
 * Provides quick fixes for common Rhema issues.
 */
public class RhemaQuickFixIntention implements IntentionAction {
    
    @NotNull
    @Override
    public String getText() {
        return "Rhema Quick Fix";
    }
    
    @NotNull
    @Override
    public String getFamilyName() {
        return "Rhema";
    }
    
    @Override
    public boolean isAvailable(@NotNull Project project, Editor editor, PsiFile file) {
        // TODO: Check if quick fix is available
        // This would involve:
        // - Checking if the file is a Rhema file
        // - Checking if there are issues that can be fixed
        // - Determining what fixes are available
        
        return isRhemaFile(file);
    }
    
    @Override
    public void invoke(@NotNull Project project, Editor editor, PsiFile file) throws IncorrectOperationException {
        // TODO: Implement quick fix
        // This would involve:
        // - Analyzing the issue
        // - Applying the appropriate fix
        // - Updating the document
        
        System.out.println("Rhema: Applying quick fix");
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