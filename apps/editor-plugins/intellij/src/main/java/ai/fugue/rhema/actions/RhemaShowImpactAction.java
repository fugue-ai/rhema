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
 * Action to show Rhema impact analysis.
 * Displays impact analysis for Rhema changes and modifications.
 */
public class RhemaShowImpactAction extends AnAction {

    public RhemaShowImpactAction() {
        super("Show Impact", "Show Rhema impact analysis", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaImpactDialog dialog = new RhemaImpactDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for displaying impact analysis.
     */
    private static class RhemaImpactDialog extends DialogWrapper {
        private final Project project;
        private final JBTable impactTable;
        private final DefaultTableModel tableModel;

        public RhemaImpactDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            
            String[] columnNames = {"Component", "Change Type", "Impact Level", "Affected Files", "Risk"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return false;
                }
            };
            
            this.impactTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Rhema Impact Analysis");
            setSize(800, 500);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JBScrollPane scrollPane = new JBScrollPane(impactTable);
            scrollPane.setPreferredSize(new Dimension(750, 400));
            return scrollPane;
        }

        private void populateTable() {
            List<ImpactItem> impacts = loadImpactAnalysis(project);
            
            tableModel.setRowCount(0);
            for (ImpactItem impact : impacts) {
                tableModel.addRow(new Object[]{
                    impact.component,
                    impact.changeType,
                    impact.impactLevel,
                    impact.affectedFiles,
                    impact.risk
                });
            }
        }

        private List<ImpactItem> loadImpactAnalysis(Project project) {
            // TODO: Implement actual impact analysis using Rhema services
            List<ImpactItem> impacts = new ArrayList<>();
            
            // Mock impact analysis for demonstration
            impacts.add(new ImpactItem("IntelliJ Plugin", "Feature Addition", "LOW", "5 files", "MINIMAL"));
            impacts.add(new ImpactItem("Language Support", "Schema Update", "MEDIUM", "12 files", "LOW"));
            impacts.add(new ImpactItem("Git Integration", "Hook Addition", "HIGH", "25 files", "MEDIUM"));
            impacts.add(new ImpactItem("Performance", "Optimization", "LOW", "8 files", "MINIMAL"));
            
            return impacts;
        }
    }

    /**
     * Represents an impact item.
     */
    private static class ImpactItem {
        final String component;
        final String changeType;
        final String impactLevel;
        final String affectedFiles;
        final String risk;

        ImpactItem(String component, String changeType, String impactLevel, String affectedFiles, String risk) {
            this.component = component;
            this.changeType = changeType;
            this.impactLevel = impactLevel;
            this.affectedFiles = affectedFiles;
            this.risk = risk;
        }
    }
} 