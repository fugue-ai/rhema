package ai.fugue.rhema.actions;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.actionSystem.CommonDataKeys;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import org.jetbrains.annotations.NotNull;

/**
 * Action to execute Rhema queries.
 * Allows users to run queries against their Rhema context.
 */
public class RhemaExecuteQueryAction extends AnAction {
    
    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getRequiredData(CommonDataKeys.PROJECT);
        
        try {
            // Get the project service
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Show query input dialog
            String query = Messages.showInputDialog(
                project,
                "Enter your Rhema query:",
                "Execute Rhema Query",
                Messages.getQuestionIcon(),
                "",
                null
            );
            
            if (query != null && !query.trim().isEmpty()) {
                // Execute the query
                String result = executeRhemaQuery(projectService, query.trim());
                
                // Show query results
                Messages.showInfoMessage(
                    project,
                    result,
                    "Query Results"
                );
            }
            
        } catch (Exception ex) {
            Messages.showErrorDialog(
                project,
                "Failed to execute Rhema query: " + ex.getMessage(),
                "Query Error"
            );
        }
    }
    
    @Override
    public void update(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        e.getPresentation().setEnabledAndVisible(project != null);
    }
    
    /**
     * Execute a Rhema query.
     */
    private String executeRhemaQuery(RhemaProjectService projectService, String query) {
        // TODO: Implement Rhema query execution
        // This would involve:
        // - Parsing the query
        // - Executing against Rhema context
        // - Returning formatted results
        
        StringBuilder result = new StringBuilder();
        result.append("Query: ").append(query).append("\n\n");
        result.append("Results:\n");
        result.append("- No results found (query execution not implemented yet)\n");
        result.append("\nThis feature will be implemented in a future version.");
        
        return result.toString();
    }
} 