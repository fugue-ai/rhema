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
 * Action to manage Rhema decisions.
 * Provides a comprehensive interface for managing decisions in Rhema projects.
 */
public class RhemaManageDecisionsAction extends AnAction {

    public RhemaManageDecisionsAction() {
        super("Manage Decisions", "Manage Rhema decisions", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaDecisionsManagerDialog dialog = new RhemaDecisionsManagerDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for managing decisions.
     */
    private static class RhemaDecisionsManagerDialog extends DialogWrapper {
        private final Project project;
        private final JBTable decisionsTable;
        private final DefaultTableModel tableModel;
        private final List<DecisionItem> decisions;

        public RhemaDecisionsManagerDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            this.decisions = loadDecisions(project);
            
            String[] columnNames = {"Date", "Decision", "Rationale", "Status"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return false;
                }
            };
            
            this.decisionsTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Manage Rhema Decisions");
            setSize(800, 500);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel mainPanel = new JPanel(new BorderLayout());
            
            // Table panel
            JBScrollPane scrollPane = new JBScrollPane(decisionsTable);
            scrollPane.setPreferredSize(new Dimension(750, 400));
            
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
            
            refreshButton.addActionListener(e -> refreshDecisions());
            exportButton.addActionListener(e -> exportDecisions());
            
            panel.add(refreshButton);
            panel.add(exportButton);
            
            return panel;
        }

        private List<DecisionItem> loadDecisions(Project project) {
            // TODO: Implement actual decisions loading using Rhema services
            List<DecisionItem> decisions = new ArrayList<>();
            
            // Mock decisions for demonstration
            decisions.add(new DecisionItem("2025-01-20", "Use IntelliJ Platform SDK", "Provides comprehensive IDE integration capabilities", "IMPLEMENTED"));
            decisions.add(new DecisionItem("2025-01-18", "Modular Architecture", "Separate components for different functionality areas", "IMPLEMENTED"));
            decisions.add(new DecisionItem("2025-01-15", "Java Implementation", "Java provides best IntelliJ integration and performance", "IMPLEMENTED"));
            
            return decisions;
        }

        private void populateTable() {
            tableModel.setRowCount(0);
            for (DecisionItem decision : decisions) {
                tableModel.addRow(new Object[]{
                    decision.date,
                    decision.decision,
                    decision.rationale,
                    decision.status
                });
            }
        }

        private void refreshDecisions() {
            decisions.clear();
            decisions.addAll(loadDecisions(project));
            populateTable();
        }

        private void exportDecisions() {
            Messages.showInfoMessage(project, "Decision export functionality will be implemented in future versions", "Export Decisions");
        }
    }

    /**
     * Represents a decision item.
     */
    private static class DecisionItem {
        final String date;
        final String decision;
        final String rationale;
        final String status;

        DecisionItem(String date, String decision, String rationale, String status) {
            this.date = date;
            this.decision = decision;
            this.rationale = rationale;
            this.status = status;
        }
    }
} 