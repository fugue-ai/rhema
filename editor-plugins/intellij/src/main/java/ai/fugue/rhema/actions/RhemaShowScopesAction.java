package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import org.jetbrains.annotations.NotNull;

/**
 * Action to show Rhema scopes.
 * Displays all Rhema scopes in the current project.
 */
public class RhemaShowScopesAction extends AnAction {
    
    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Rhema Show Scopes");
            return;
        }
        
        showScopes(project);
    }
    
    @Override
    public void update(@NotNull AnActionEvent e) {
        // Enable the action only when a project is available
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }
    
    /**
     * Show Rhema scopes for the project.
     */
    private void showScopes(@NotNull Project project) {
        try {
            // TODO: Implement scope discovery and display
            // This would include:
            // - Scanning for scope files in the project
            // - Parsing scope definitions
            // - Displaying scope hierarchy
            // - Showing scope relationships
            
            Messages.showInfoMessage(
                project,
                "Rhema scopes functionality will be implemented in the next iteration.\n" +
                "This will show all scopes in the project with their relationships.",
                "Rhema Show Scopes"
            );
            
        } catch (Exception ex) {
            Messages.showErrorDialog(
                project,
                "Error showing scopes: " + ex.getMessage(),
                "Rhema Show Scopes Error"
            );
        }
    }
} 