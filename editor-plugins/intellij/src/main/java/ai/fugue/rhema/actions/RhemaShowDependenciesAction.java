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
 * Action to show Rhema dependencies.
 * Displays dependency relationships between Rhema components.
 */
public class RhemaShowDependenciesAction extends AnAction {

    public RhemaShowDependenciesAction() {
        super("Show Dependencies", "Show Rhema dependencies", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaDependenciesDialog dialog = new RhemaDependenciesDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for displaying dependencies.
     */
    private static class RhemaDependenciesDialog extends DialogWrapper {
        private final Project project;
        private final JBTable dependenciesTable;
        private final DefaultTableModel tableModel;

        public RhemaDependenciesDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            
            String[] columnNames = {"Component", "Dependency", "Type", "Status"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return false;
                }
            };
            
            this.dependenciesTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Rhema Dependencies");
            setSize(700, 500);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JBScrollPane scrollPane = new JBScrollPane(dependenciesTable);
            scrollPane.setPreferredSize(new Dimension(650, 400));
            return scrollPane;
        }

        private void populateTable() {
            List<DependencyItem> dependencies = loadDependencies(project);
            
            tableModel.setRowCount(0);
            for (DependencyItem dependency : dependencies) {
                tableModel.addRow(new Object[]{
                    dependency.component,
                    dependency.dependency,
                    dependency.type,
                    dependency.status
                });
            }
        }

        private List<DependencyItem> loadDependencies(Project project) {
            // TODO: Implement actual dependency loading using Rhema services
            List<DependencyItem> dependencies = new ArrayList<>();
            
            // Mock dependencies for demonstration
            dependencies.add(new DependencyItem("IntelliJ Plugin", "IntelliJ Platform SDK", "REQUIRED", "SATISFIED"));
            dependencies.add(new DependencyItem("Language Support", "YAML Parser", "REQUIRED", "SATISFIED"));
            dependencies.add(new DependencyItem("Git Integration", "Git4Idea", "OPTIONAL", "SATISFIED"));
            dependencies.add(new DependencyItem("Terminal Integration", "Terminal Plugin", "OPTIONAL", "SATISFIED"));
            dependencies.add(new DependencyItem("Rhema CLI", "Rhema Core", "REQUIRED", "SATISFIED"));
            
            return dependencies;
        }
    }

    /**
     * Represents a dependency item.
     */
    private static class DependencyItem {
        final String component;
        final String dependency;
        final String type;
        final String status;

        DependencyItem(String component, String dependency, String type, String status) {
            this.component = component;
            this.dependency = dependency;
            this.type = type;
            this.status = status;
        }
    }
} 