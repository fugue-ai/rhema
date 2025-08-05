package ai.fugue.rhema.ui;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import org.jetbrains.annotations.NotNull;

/**
 * Action to show the Rhema scope tree view.
 * Displays a tree view of Rhema scopes in the project.
 */
public class RhemaShowTreeAction extends AnAction {
    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project != null) {
            Messages.showInfoMessage(project, "Rhema scope tree view will be implemented in a future version.", "Rhema Scope Tree");
        }
    }
}