package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.ui.DialogWrapper;
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
 * Action to manage Rhema patterns.
 * Provides a comprehensive interface for managing patterns in Rhema projects.
 */
public class RhemaManagePatternsAction extends AnAction {

    public RhemaManagePatternsAction() {
        super("Manage Patterns", "Manage Rhema patterns", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaPatternsManagerDialog dialog = new RhemaPatternsManagerDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for managing patterns.
     */
    private static class RhemaPatternsManagerDialog extends DialogWrapper {
        private final Project project;
        private final JBTable patternsTable;
        private final DefaultTableModel tableModel;
        private final List<PatternItem> patterns;

        public RhemaPatternsManagerDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            this.patterns = loadPatterns(project);
            
            String[] columnNames = {"Name", "Type", "Description", "Usage Count"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return false;
                }
            };
            
            this.patternsTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Manage Rhema Patterns");
            setSize(700, 500);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel mainPanel = new JPanel(new BorderLayout());
            
            // Table panel
            JBScrollPane scrollPane = new JBScrollPane(patternsTable);
            scrollPane.setPreferredSize(new Dimension(650, 400));
            
            // Button panel
            JPanel buttonPanel = createButtonPanel();
            
            mainPanel.add(scrollPane, BorderLayout.CENTER);
            mainPanel.add(buttonPanel, BorderLayout.SOUTH);
            
            return mainPanel;
        }

        private JPanel createButtonPanel() {
            JPanel panel = new JPanel(new FlowLayout(FlowLayout.LEFT));
            
            JButton refreshButton = new JButton("Refresh");
            JButton exportButton = new JButton("Export");
            
            refreshButton.addActionListener(e -> refreshPatterns());
            exportButton.addActionListener(e -> exportPatterns());
            
            panel.add(refreshButton);
            panel.add(exportButton);
            
            return panel;
        }

        private List<PatternItem> loadPatterns(Project project) {
            // TODO: Implement actual patterns loading using Rhema services
            List<PatternItem> patterns = new ArrayList<>();
            
            // Mock patterns for demonstration
            patterns.add(new PatternItem("Plugin Architecture", "ARCHITECTURE", "Standard plugin architecture pattern", 5));
            patterns.add(new PatternItem("Action Management", "UI", "Pattern for managing IntelliJ actions", 12));
            patterns.add(new PatternItem("Dialog Creation", "UI", "Standard dialog creation pattern", 8));
            patterns.add(new PatternItem("Service Integration", "INTEGRATION", "Pattern for integrating with IntelliJ services", 3));
            
            return patterns;
        }

        private void populateTable() {
            tableModel.setRowCount(0);
            for (PatternItem pattern : patterns) {
                tableModel.addRow(new Object[]{
                    pattern.name,
                    pattern.type,
                    pattern.description,
                    pattern.usageCount
                });
            }
        }

        private void refreshPatterns() {
            patterns.clear();
            patterns.addAll(loadPatterns(project));
            populateTable();
        }

        private void exportPatterns() {
            Messages.showInfoMessage(project, "Pattern export functionality will be implemented in future versions", "Export Patterns");
        }
    }

    /**
     * Represents a pattern item.
     */
    private static class PatternItem {
        final String name;
        final String type;
        final String description;
        final int usageCount;

        PatternItem(String name, String type, String description, int usageCount) {
            this.name = name;
            this.type = type;
            this.description = description;
            this.usageCount = usageCount;
        }
    }
} 