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
 * Action to manage Rhema todos.
 * Provides a comprehensive interface for managing todos in Rhema projects.
 */
public class RhemaManageTodosAction extends AnAction {

    public RhemaManageTodosAction() {
        super("Manage Todos", "Manage Rhema todos", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaTodosManagerDialog dialog = new RhemaTodosManagerDialog(project);
        dialog.show();
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    /**
     * Dialog for managing todos.
     */
    private static class RhemaTodosManagerDialog extends DialogWrapper {
        private final Project project;
        private final JBTable todosTable;
        private final DefaultTableModel tableModel;
        private final List<TodoItem> todos;

        public RhemaTodosManagerDialog(@NotNull Project project) {
            super(project);
            this.project = project;
            this.todos = loadTodos(project);
            
            String[] columnNames = {"Priority", "Status", "Title", "Description", "Due Date"};
            this.tableModel = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return column != 0; // Priority is not editable
                }
            };
            
            this.todosTable = new JBTable(tableModel);
            populateTable();
            
            setTitle("Manage Rhema Todos");
            setSize(800, 600);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel mainPanel = new JPanel(new BorderLayout());
            
            // Table panel
            JBScrollPane scrollPane = new JBScrollPane(todosTable);
            scrollPane.setPreferredSize(new Dimension(750, 400));
            
            // Button panel
            JPanel buttonPanel = createButtonPanel();
            
            mainPanel.add(scrollPane, BorderLayout.CENTER);
            mainPanel.add(buttonPanel, BorderLayout.SOUTH);
            
            return mainPanel;
        }

        private JPanel createButtonPanel() {
            JPanel panel = new JPanel(new FlowLayout(FlowLayout.LEFT));
            
            JButton addButton = new JButton("Add Todo");
            JButton editButton = new JButton("Edit Todo");
            JButton deleteButton = new JButton("Delete Todo");
            JButton refreshButton = new JButton("Refresh");
            
            addButton.addActionListener(e -> addTodo());
            editButton.addActionListener(e -> editTodo());
            deleteButton.addActionListener(e -> deleteTodo());
            refreshButton.addActionListener(e -> refreshTodos());
            
            panel.add(addButton);
            panel.add(editButton);
            panel.add(deleteButton);
            panel.add(refreshButton);
            
            return panel;
        }

        private List<TodoItem> loadTodos(Project project) {
            // TODO: Implement actual todo loading using Rhema services
            List<TodoItem> todos = new ArrayList<>();
            
            // Mock todos for demonstration
            todos.add(new TodoItem("HIGH", "PENDING", "Complete IntelliJ Plugin", "Finish the remaining 2% of IntelliJ plugin implementation", "2025-02-24"));
            todos.add(new TodoItem("MEDIUM", "IN_PROGRESS", "Performance Monitoring", "Implement real-time performance monitoring and analytics", "2025-03-01"));
            todos.add(new TodoItem("LOW", "COMPLETED", "Documentation Update", "Update plugin documentation with latest features", "2025-01-15"));
            
            return todos;
        }

        private void populateTable() {
            tableModel.setRowCount(0);
            for (TodoItem todo : todos) {
                tableModel.addRow(new Object[]{
                    todo.priority,
                    todo.status,
                    todo.title,
                    todo.description,
                    todo.dueDate
                });
            }
        }

        private void addTodo() {
            RhemaAddTodoDialog dialog = new RhemaAddTodoDialog(project);
            if (dialog.showAndGet()) {
                TodoItem newTodo = dialog.getTodoItem();
                todos.add(newTodo);
                populateTable();
            }
        }

        private void editTodo() {
            int selectedRow = todosTable.getSelectedRow();
            if (selectedRow >= 0) {
                TodoItem todo = todos.get(selectedRow);
                RhemaEditTodoDialog dialog = new RhemaEditTodoDialog(project, todo);
                if (dialog.showAndGet()) {
                    TodoItem updatedTodo = dialog.getTodoItem();
                    todos.set(selectedRow, updatedTodo);
                    populateTable();
                }
            } else {
                Messages.showWarningDialog(project, "Please select a todo to edit", "Edit Todo");
            }
        }

        private void deleteTodo() {
            int selectedRow = todosTable.getSelectedRow();
            if (selectedRow >= 0) {
                int result = Messages.showYesNoDialog(project, 
                    "Are you sure you want to delete this todo?", 
                    "Delete Todo", 
                    Messages.getQuestionIcon());
                
                if (result == Messages.YES) {
                    todos.remove(selectedRow);
                    populateTable();
                }
            } else {
                Messages.showWarningDialog(project, "Please select a todo to delete", "Delete Todo");
            }
        }

        private void refreshTodos() {
            todos.clear();
            todos.addAll(loadTodos(project));
            populateTable();
        }
    }

    /**
     * Dialog for adding a new todo.
     */
    private static class RhemaAddTodoDialog extends DialogWrapper {
        private final JBTextField titleField;
        private final JBTextArea descriptionArea;
        private final JComboBox<String> priorityCombo;
        private final JComboBox<String> statusCombo;
        private final JBTextField dueDateField;

        public RhemaAddTodoDialog(@NotNull Project project) {
            super(project);
            setTitle("Add New Todo");
            
            titleField = new JBTextField(30);
            descriptionArea = new JBTextArea(5, 30);
            priorityCombo = new JComboBox<>(new String[]{"LOW", "MEDIUM", "HIGH", "CRITICAL"});
            statusCombo = new JComboBox<>(new String[]{"PENDING", "IN_PROGRESS", "COMPLETED", "CANCELLED"});
            dueDateField = new JBTextField(15);
            
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel dialogPanel = FormBuilder.createFormBuilder()
                    .addLabeledComponent(new JBLabel("Title:"), titleField)
                    .addLabeledComponent(new JBLabel("Description:"), new JBScrollPane(descriptionArea))
                    .addLabeledComponent(new JBLabel("Priority:"), priorityCombo)
                    .addLabeledComponent(new JBLabel("Status:"), statusCombo)
                    .addLabeledComponent(new JBLabel("Due Date (YYYY-MM-DD):"), dueDateField)
                    .addComponentFillVertically(new JPanel(), 0)
                    .getPanel();
            dialogPanel.setPreferredSize(new Dimension(400, 300));
            return dialogPanel;
        }

        public TodoItem getTodoItem() {
            return new TodoItem(
                (String) priorityCombo.getSelectedItem(),
                (String) statusCombo.getSelectedItem(),
                titleField.getText(),
                descriptionArea.getText(),
                dueDateField.getText()
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
     * Dialog for editing an existing todo.
     */
    private static class RhemaEditTodoDialog extends DialogWrapper {
        private final JBTextField titleField;
        private final JBTextArea descriptionArea;
        private final JComboBox<String> priorityCombo;
        private final JComboBox<String> statusCombo;
        private final JBTextField dueDateField;

        public RhemaEditTodoDialog(@NotNull Project project, TodoItem todo) {
            super(project);
            setTitle("Edit Todo");
            
            titleField = new JBTextField(30);
            descriptionArea = new JBTextArea(5, 30);
            priorityCombo = new JComboBox<>(new String[]{"LOW", "MEDIUM", "HIGH", "CRITICAL"});
            statusCombo = new JComboBox<>(new String[]{"PENDING", "IN_PROGRESS", "COMPLETED", "CANCELLED"});
            dueDateField = new JBTextField(15);
            
            // Populate fields with existing data
            titleField.setText(todo.title);
            descriptionArea.setText(todo.description);
            priorityCombo.setSelectedItem(todo.priority);
            statusCombo.setSelectedItem(todo.status);
            dueDateField.setText(todo.dueDate);
            
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel dialogPanel = FormBuilder.createFormBuilder()
                    .addLabeledComponent(new JBLabel("Title:"), titleField)
                    .addLabeledComponent(new JBLabel("Description:"), new JBScrollPane(descriptionArea))
                    .addLabeledComponent(new JBLabel("Priority:"), priorityCombo)
                    .addLabeledComponent(new JBLabel("Status:"), statusCombo)
                    .addLabeledComponent(new JBLabel("Due Date (YYYY-MM-DD):"), dueDateField)
                    .addComponentFillVertically(new JPanel(), 0)
                    .getPanel();
            dialogPanel.setPreferredSize(new Dimension(400, 300));
            return dialogPanel;
        }

        public TodoItem getTodoItem() {
            return new TodoItem(
                (String) priorityCombo.getSelectedItem(),
                (String) statusCombo.getSelectedItem(),
                titleField.getText(),
                descriptionArea.getText(),
                dueDateField.getText()
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
     * Represents a todo item.
     */
    private static class TodoItem {
        final String priority;
        final String status;
        final String title;
        final String description;
        final String dueDate;

        TodoItem(String priority, String status, String title, String description, String dueDate) {
            this.priority = priority;
            this.status = status;
            this.title = title;
            this.description = description;
            this.dueDate = dueDate;
        }
    }
} 