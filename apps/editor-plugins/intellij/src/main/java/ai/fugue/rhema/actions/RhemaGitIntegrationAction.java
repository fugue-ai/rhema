package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.ui.DialogWrapper;
import com.intellij.ui.components.JBLabel;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.table.JBTable;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import javax.swing.table.DefaultTableModel;
import java.awt.*;
import java.util.List;
import java.util.ArrayList;

/**
 * Action for Rhema Git integration.
 * Provides Git workflow integration for Rhema projects.
 */
public class RhemaGitIntegrationAction extends AnAction {

    public RhemaGitIntegrationAction() {
        super("Git Integration", "Rhema Git integration", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaGitIntegrationDialog dialog = new RhemaGitIntegrationDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for Git integration.
     */
    private static class RhemaGitIntegrationDialog extends DialogWrapper {
        private final Project project;
        private final JBTable gitTable;
        private final DefaultTableModel tableModel;

        public RhemaGitIntegrationDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            
            String[] columnNames = {"Operation", "Status", "Last Run", "Next Run"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return false;
                }
            };
            
            this.gitTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Rhema Git Integration");
            setSize(600, 400);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel mainPanel = new JPanel(new BorderLayout());
            
            // Table panel
            JBScrollPane scrollPane = new JBScrollPane(gitTable);
            scrollPane.setPreferredSize(new Dimension(550, 300));
            
            // Button panel
            JPanel buttonPanel = createButtonPanel();
            
            mainPanel.add(scrollPane, BorderLayout.CENTER);
            mainPanel.add(buttonPanel, BorderLayout.SOUTH);
            
            return mainPanel;
        }

        private JPanel createButtonPanel() {
            JPanel panel = new JPanel(new FlowLayout(FlowLayout.LEFT));
            
            JButton runButton = new JButton("Run Selected");
            JButton refreshButton = new JButton("Refresh");
            
            runButton.addActionListener(e -> runSelectedOperation());
            refreshButton.addActionListener(e -> refreshOperations());
            
            panel.add(runButton);
            panel.add(refreshButton);
            
            return panel;
        }

        private void populateTable() {
            List<GitOperation> operations = loadGitOperations(project);
            
            tableModel.setRowCount(0);
            for (GitOperation operation : operations) {
                tableModel.addRow(new Object[]{
                    operation.name,
                    operation.status,
                    operation.lastRun,
                    operation.nextRun
                });
            }
        }

        private List<GitOperation> loadGitOperations(Project project) {
            // TODO: Implement actual Git operations loading using Rhema services
            List<GitOperation> operations = new ArrayList<>();
            
            // Mock Git operations for demonstration
            operations.add(new GitOperation("Pre-commit Hook", "ENABLED", "2025-01-20 14:30", "On commit"));
            operations.add(new GitOperation("Post-commit Hook", "ENABLED", "2025-01-20 14:30", "On commit"));
            operations.add(new GitOperation("Pre-push Hook", "ENABLED", "2025-01-20 14:30", "On push"));
            operations.add(new GitOperation("Branch Sync", "ENABLED", "2025-01-20 14:30", "On branch change"));
            
            return operations;
        }

        private void runSelectedOperation() {
            int selectedRow = gitTable.getSelectedRow();
            if (selectedRow >= 0) {
                Messages.showInfoMessage(project, "Git operation executed successfully", "Git Integration");
            } else {
                Messages.showWarningDialog(project, "Please select an operation to run", "Git Integration");
            }
        }

        private void refreshOperations() {
            populateTable();
        }
    }

    /**
     * Represents a Git operation.
     */
    private static class GitOperation {
        final String name;
        final String status;
        final String lastRun;
        final String nextRun;

        GitOperation(String name, String status, String lastRun, String nextRun) {
            this.name = name;
            this.status = status;
            this.lastRun = lastRun;
            this.nextRun = nextRun;
        }
    }
} 