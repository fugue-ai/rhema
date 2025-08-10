package ai.fugue.rhema.actions;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.actionSystem.CommonDataKeys;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import org.jetbrains.annotations.NotNull;

/**
 * Action to initialize a new Rhema scope.
 * Allows users to create a new Rhema configuration in their project.
 */
public class RhemaInitializeAction extends AnAction {
    
    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getRequiredData(CommonDataKeys.PROJECT);
        
        try {
            // Get the project service
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Show initialization dialog
            String scopeName = Messages.showInputDialog(
                project,
                "Enter the name for your new Rhema scope:",
                "Initialize Rhema Scope",
                Messages.getQuestionIcon(),
                "my-scope",
                null
            );
            
            if (scopeName != null && !scopeName.trim().isEmpty()) {
                // Initialize the Rhema scope
                initializeRhemaScope(project, scopeName.trim());
                
                Messages.showInfoMessage(
                    project,
                    "Rhema scope '" + scopeName + "' initialized successfully!",
                    "Rhema Scope Initialized"
                );
            }
            
        } catch (Exception ex) {
            Messages.showErrorDialog(
                project,
                "Failed to initialize Rhema scope: " + ex.getMessage(),
                "Initialization Error"
            );
        }
    }
    
    @Override
    public void update(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        e.getPresentation().setEnabledAndVisible(project != null);
    }
    
    /**
     * Initialize a new Rhema scope.
     */
    private void initializeRhemaScope(Project project, String scopeName) {
        // TODO: Implement Rhema scope initialization
        // This would involve:
        // - Creating a new rhema.yml file
        // - Setting up the basic scope structure
        // - Configuring the scope with the given name
        
        // For now, we'll just log the action
        System.out.println("Initializing Rhema scope: " + scopeName + " in project: " + project.getName());
    }
} 