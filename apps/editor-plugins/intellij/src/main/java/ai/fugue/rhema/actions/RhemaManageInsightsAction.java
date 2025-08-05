package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.ui.DialogWrapper;
import com.intellij.openapi.ui.ValidationInfo;
import com.intellij.ui.components.JBTextField;
import com.intellij.ui.components.JBTextArea;
import com.intellij.ui.components.JBLabel;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.table.JBTable;
import com.intellij.util.ui.FormBuilder;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import javax.swing.table.DefaultTableModel;
import java.awt.*;
import java.util.List;
import java.util.ArrayList;

/**
 * Action to manage Rhema insights.
 * Provides a comprehensive interface for managing insights in Rhema projects.
 */
public class RhemaManageInsightsAction extends AnAction {

    public RhemaManageInsightsAction() {
        super("Manage Insights", "Manage Rhema insights", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaInsightsManagerDialog dialog = new RhemaInsightsManagerDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for managing insights.
     */
    private static class RhemaInsightsManagerDialog extends DialogWrapper {
        private final Project project;
        private final JBTable insightsTable;
        private final DefaultTableModel tableModel;
        private final List<InsightItem> insights;

        public RhemaInsightsManagerDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            this.insights = loadInsights(project);
            
            String[] columnNames = {"Category", "Title", "Description", "Confidence", "Date"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return column != 0; // Category is not editable
                }
            };
            
            this.insightsTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Manage Rhema Insights");
            setSize(800, 600);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel mainPanel = new JPanel(new BorderLayout());
            
            // Table panel
            JBScrollPane scrollPane = new JBScrollPane(insightsTable);
            scrollPane.setPreferredSize(new Dimension(750, 400));
            
            // Button panel
            JPanel buttonPanel = createButtonPanel();
            
            mainPanel.add(scrollPane, BorderLayout.CENTER);
            mainPanel.add(buttonPanel, BorderLayout.SOUTH);
            
            return mainPanel;
        }

        private JPanel createButtonPanel() {
            JPanel panel = new JPanel(new FlowLayout(FlowLayout.LEFT));
            
            JButton addButton = new JButton("Add Insight");
            JButton editButton = new JButton("Edit Insight");
            JButton deleteButton = new JButton("Delete Insight");
            JButton refreshButton = new JButton("Refresh");
            
            addButton.addActionListener(e -> addInsight());
            editButton.addActionListener(e -> editInsight());
            deleteButton.addActionListener(e -> deleteInsight());
            refreshButton.addActionListener(e -> refreshInsights());
            
            panel.add(addButton);
            panel.add(editButton);
            panel.add(deleteButton);
            panel.add(refreshButton);
            
            return panel;
        }

        private List<InsightItem> loadInsights(Project project) {
            // TODO: Implement actual insights loading using Rhema services
            List<InsightItem> insights = new ArrayList<>();
            
            // Mock insights for demonstration
            insights.add(new InsightItem("PERFORMANCE", "Plugin Performance", "IntelliJ plugin shows excellent performance metrics", "HIGH", "2025-01-20"));
            insights.add(new InsightItem("ARCHITECTURE", "Modular Design", "Plugin architecture follows best practices for extensibility", "MEDIUM", "2025-01-18"));
            insights.add(new InsightItem("USER_EXPERIENCE", "Intuitive Interface", "User interface provides clear and accessible functionality", "HIGH", "2025-01-15"));
            
            return insights;
        }

        private void populateTable() {
            tableModel.setRowCount(0);
            for (InsightItem insight : insights) {
                tableModel.addRow(new Object[]{
                    insight.category,
                    insight.title,
                    insight.description,
                    insight.confidence,
                    insight.date
                });
            }
        }

        private void addInsight() {
            RhemaAddInsightDialog dialog = new RhemaAddInsightDialog(project);
            if (dialog.showAndGet()) {
                InsightItem newInsight = dialog.getInsightItem();
                insights.add(newInsight);
                populateTable();
            }
        }

        private void editInsight() {
            int selectedRow = insightsTable.getSelectedRow();
            if (selectedRow >= 0) {
                InsightItem insight = insights.get(selectedRow);
                RhemaEditInsightDialog dialog = new RhemaEditInsightDialog(project, insight);
                if (dialog.showAndGet()) {
                    InsightItem updatedInsight = dialog.getInsightItem();
                    insights.set(selectedRow, updatedInsight);
                    populateTable();
                }
            } else {
                Messages.showWarningDialog(project, "Please select an insight to edit", "Edit Insight");
            }
        }

        private void deleteInsight() {
            int selectedRow = insightsTable.getSelectedRow();
            if (selectedRow >= 0) {
                int result = Messages.showYesNoDialog(project, 
                    "Are you sure you want to delete this insight?", 
                    "Delete Insight", 
                    Messages.getQuestionIcon());
                
                if (result == Messages.YES) {
                    insights.remove(selectedRow);
                    populateTable();
                }
            } else {
                Messages.showWarningDialog(project, "Please select an insight to delete", "Delete Insight");
            }
        }

        private void refreshInsights() {
            insights.clear();
            insights.addAll(loadInsights(project));
            populateTable();
        }
    }

    /**
     * Dialog for adding a new insight.
     */
    private static class RhemaAddInsightDialog extends DialogWrapper {
        private final JBTextField titleField;
        private final JBTextArea descriptionArea;
        private final JComboBox<String> categoryCombo;
        private final JComboBox<String> confidenceCombo;

        public RhemaAddInsightDialog(@NotNull Project project) {
            super(project);
            setTitle("Add New Insight");
            
            titleField = new JBTextField(30);
            descriptionArea = new JBTextArea(5, 30);
            categoryCombo = new JComboBox<>(new String[]{"PERFORMANCE", "ARCHITECTURE", "USER_EXPERIENCE", "SECURITY", "MAINTAINABILITY"});
            confidenceCombo = new JComboBox<>(new String[]{"LOW", "MEDIUM", "HIGH"});
            
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel dialogPanel = FormBuilder.createFormBuilder()
                    .addLabeledComponent(new JBLabel("Title:"), titleField)
                    .addLabeledComponent(new JBLabel("Description:"), new JBScrollPane(descriptionArea))
                    .addLabeledComponent(new JBLabel("Category:"), categoryCombo)
                    .addLabeledComponent(new JBLabel("Confidence:"), confidenceCombo)
                    .addComponentFillVertically(new JPanel(), 0)
                    .getPanel();
            dialogPanel.setPreferredSize(new Dimension(400, 300));
            return dialogPanel;
        }

        public InsightItem getInsightItem() {
            return new InsightItem(
                (String) categoryCombo.getSelectedItem(),
                titleField.getText(),
                descriptionArea.getText(),
                (String) confidenceCombo.getSelectedItem(),
                java.time.LocalDate.now().toString()
            );
        }

        @Override
        protected ValidationInfo doValidate() {
            if (titleField.getText().trim().isEmpty()) {
                return new ValidationInfo("Title cannot be empty", titleField);
            }
            return null;
        }
    }

    /**
     * Dialog for editing an existing insight.
     */
    private static class RhemaEditInsightDialog extends DialogWrapper {
        private final JBTextField titleField;
        private final JBTextArea descriptionArea;
        private final JComboBox<String> categoryCombo;
        private final JComboBox<String> confidenceCombo;

        public RhemaEditInsightDialog(@NotNull Project project, InsightItem insight) {
            super(project);
            setTitle("Edit Insight");
            
            titleField = new JBTextField(30);
            descriptionArea = new JBTextArea(5, 30);
            categoryCombo = new JComboBox<>(new String[]{"PERFORMANCE", "ARCHITECTURE", "USER_EXPERIENCE", "SECURITY", "MAINTAINABILITY"});
            confidenceCombo = new JComboBox<>(new String[]{"LOW", "MEDIUM", "HIGH"});
            
            // Populate fields with existing data
            titleField.setText(insight.title);
            descriptionArea.setText(insight.description);
            categoryCombo.setSelectedItem(insight.category);
            confidenceCombo.setSelectedItem(insight.confidence);
            
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel dialogPanel = FormBuilder.createFormBuilder()
                    .addLabeledComponent(new JBLabel("Title:"), titleField)
                    .addLabeledComponent(new JBLabel("Description:"), new JBScrollPane(descriptionArea))
                    .addLabeledComponent(new JBLabel("Category:"), categoryCombo)
                    .addLabeledComponent(new JBLabel("Confidence:"), confidenceCombo)
                    .addComponentFillVertically(new JPanel(), 0)
                    .getPanel();
            dialogPanel.setPreferredSize(new Dimension(400, 300));
            return dialogPanel;
        }

        public InsightItem getInsightItem() {
            return new InsightItem(
                (String) categoryCombo.getSelectedItem(),
                titleField.getText(),
                descriptionArea.getText(),
                (String) confidenceCombo.getSelectedItem(),
                java.time.LocalDate.now().toString()
            );
        }

        @Override
        protected ValidationInfo doValidate() {
            if (titleField.getText().trim().isEmpty()) {
                return new ValidationInfo("Title cannot be empty", titleField);
            }
            return null;
        }
    }

    /**
     * Represents an insight item.
     */
    private static class InsightItem {
        final String category;
        final String title;
        final String description;
        final String confidence;
        final String date;

        InsightItem(String category, String title, String description, String confidence, String date) {
            this.category = category;
            this.title = title;
            this.description = description;
            this.confidence = confidence;
            this.date = date;
        }
    }
} 