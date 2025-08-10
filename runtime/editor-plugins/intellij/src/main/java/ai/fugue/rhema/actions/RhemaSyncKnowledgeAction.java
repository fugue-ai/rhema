package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.ui.DialogWrapper;
import com.intellij.ui.components.JBLabel;
import com.intellij.ui.components.JBProgressBar;
import com.intellij.util.ui.FormBuilder;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import java.awt.*;

/**
 * Action to sync Rhema knowledge.
 * Synchronizes knowledge base with current project state.
 */
public class RhemaSyncKnowledgeAction extends AnAction {

    public RhemaSyncKnowledgeAction() {
        super("Sync Knowledge", "Sync Rhema knowledge", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaSyncKnowledgeDialog dialog = new RhemaSyncKnowledgeDialog(project);
        if (dialog.showAndGet()) {
            performSync(project);
        }
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    private void performSync(Project project) {
        // TODO: Implement actual knowledge sync using Rhema services
        Messages.showInfoMessage(project, "Knowledge synchronization completed successfully", "Sync Knowledge");
    }

    /**
     * Dialog for knowledge synchronization.
     */
    private static class RhemaSyncKnowledgeDialog extends DialogWrapper {
        private final Project project;
        private final JBProgressBar progressBar;
        private final JBLabel statusLabel;

        public RhemaSyncKnowledgeDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            
            this.progressBar = new JBProgressBar(0, 100);
            this.statusLabel = new JBLabel("Ready to sync knowledge base...");
            
            setTitle("Sync Rhema Knowledge");
            setSize(400, 150);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel dialogPanel = FormBuilder.createFormBuilder()
                    .addComponent(statusLabel)
                    .addComponent(progressBar)
                    .addComponentFillVertically(new JPanel(), 0)
                    .getPanel();
            dialogPanel.setPreferredSize(new Dimension(350, 100));
            return dialogPanel;
        }

        @Override
        protected void doOKAction() {
            // Simulate sync process
            progressBar.setIndeterminate(true);
            statusLabel.setText("Synchronizing knowledge base...");
            
            // In a real implementation, this would be done asynchronously
            Timer timer = new Timer(2000, e -> {
                progressBar.setIndeterminate(false);
                progressBar.setValue(100);
                statusLabel.setText("Sync completed successfully!");
                close(OK_EXIT_CODE);
            });
            timer.setRepeats(false);
            timer.start();
        }
    }
} 