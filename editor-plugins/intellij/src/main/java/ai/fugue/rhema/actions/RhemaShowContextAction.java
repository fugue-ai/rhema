package ai.fugue.rhema.actions;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.actionSystem.CommonDataKeys;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import org.jetbrains.annotations.NotNull;

/**
 * Action to show Rhema context information.
 * Displays context details for the current project.
 */
public class RhemaShowContextAction extends AnAction {
    
    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getRequiredData(CommonDataKeys.PROJECT);
        
        try {
            // Get the project service
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Get context information
            String contextInfo = getRhemaContextInfo(projectService);
            
            // Show context information
            Messages.showInfoMessage(
                project,
                contextInfo,
                "Rhema Context Information"
            );
            
        } catch (Exception ex) {
            Messages.showErrorDialog(
                project,
                "Failed to show Rhema context: " + ex.getMessage(),
                "Context Error"
            );
        }
    }
    
    @Override
    public void update(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        e.getPresentation().setEnabledAndVisible(project != null);
    }
    
    /**
     * Get Rhema context information.
     */
    private String getRhemaContextInfo(RhemaProjectService projectService) {
        // TODO: Implement Rhema context information gathering
        // This would involve:
        // - Parsing Rhema files
        // - Extracting context information
        // - Formatting the information for display
        
        StringBuilder info = new StringBuilder();
        info.append("Rhema Context Information\n\n");
        
        // Add basic project information
        info.append("Project: ").append(projectService.getProject().getName()).append("\n");
        info.append("Rhema Files: ").append(projectService.getRhemaFiles().size()).append("\n");
        
        // Add context details
        info.append("\nContext Details:\n");
        info.append("- Scopes: 0\n");
        info.append("- Todos: 0\n");
        info.append("- Insights: 0\n");
        info.append("- Patterns: 0\n");
        info.append("- Decisions: 0\n");
        
        return info.toString();
    }
} 