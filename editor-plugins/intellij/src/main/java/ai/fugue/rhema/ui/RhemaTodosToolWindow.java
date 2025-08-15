package ai.fugue.rhema.ui;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.project.Project;
import com.intellij.ui.components.JBList;
import com.intellij.ui.components.JBScrollPane;

import javax.swing.*;
import java.awt.*;
import java.util.ArrayList;
import java.util.List;

/**
 * Tool window for displaying Rhema todos.
 * Shows todos in a list view with management capabilities.
 */
public class RhemaTodosToolWindow {
    
    private final Project project;
    private final JPanel content;
    private final JBList<String> todosList;
    private final DefaultListModel<String> listModel;
    
    public RhemaTodosToolWindow(Project project) {
        this.project = project;
        this.content = new JPanel(new BorderLayout());
        this.listModel = new DefaultListModel<>();
        this.todosList = new JBList<>(listModel);
        
        initializeUI();
        loadTodos();
    }
    
    /**
     * Initialize the UI components.
     */
    private void initializeUI() {
        // Set up the list
        todosList.setSelectionMode(ListSelectionModel.SINGLE_SELECTION);
        
        // Create scroll pane
        JBScrollPane scrollPane = new JBScrollPane(todosList);
        content.add(scrollPane, BorderLayout.CENTER);
        
        // Add toolbar
        JToolBar toolBar = createToolBar();
        content.add(toolBar, BorderLayout.NORTH);
    }
    
    /**
     * Create the toolbar.
     */
    private JToolBar createToolBar() {
        JToolBar toolBar = new JToolBar();
        toolBar.setFloatable(false);
        
        // Add refresh button
        JButton refreshButton = new JButton("Refresh");
        refreshButton.addActionListener(e -> loadTodos());
        toolBar.add(refreshButton);
        
        // Add add todo button
        JButton addButton = new JButton("Add Todo");
        addButton.addActionListener(e -> addTodo());
        toolBar.add(addButton);
        
        // Add remove todo button
        JButton removeButton = new JButton("Remove Todo");
        removeButton.addActionListener(e -> removeTodo());
        toolBar.add(removeButton);
        
        return toolBar;
    }
    
    /**
     * Load todos from the project.
     */
    private void loadTodos() {
        try {
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Clear existing todos
            listModel.clear();
            
            // TODO: Load actual todos from Rhema files
            // For now, add placeholder todos
            listModel.addElement("Implement Rhema plugin core functionality");
            listModel.addElement("Add comprehensive validation");
            listModel.addElement("Implement IntelliSense features");
            listModel.addElement("Create UI components");
            listModel.addElement("Add testing framework");
            
        } catch (Exception e) {
            // Handle error
            listModel.clear();
            listModel.addElement("Error loading todos: " + e.getMessage());
        }
    }
    
    /**
     * Add a new todo.
     */
    private void addTodo() {
        String todoText = JOptionPane.showInputDialog(
            content,
            "Enter todo text:",
            "Add Todo",
            JOptionPane.PLAIN_MESSAGE
        );
        
        if (todoText != null && !todoText.trim().isEmpty()) {
            listModel.addElement(todoText.trim());
        }
    }
    
    /**
     * Remove the selected todo.
     */
    private void removeTodo() {
        int selectedIndex = todosList.getSelectedIndex();
        if (selectedIndex >= 0) {
            listModel.remove(selectedIndex);
        }
    }
    
    /**
     * Get the content panel.
     */
    public JComponent getContent() {
        return content;
    }
} 