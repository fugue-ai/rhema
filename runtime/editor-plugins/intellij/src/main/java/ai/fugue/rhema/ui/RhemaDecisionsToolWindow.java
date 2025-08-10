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
 * Tool window for displaying Rhema decisions.
 * Shows decisions in a list view with management capabilities.
 */
public class RhemaDecisionsToolWindow {
    
    private final Project project;
    private final JPanel content;
    private final JBList<String> decisionsList;
    private final DefaultListModel<String> listModel;
    
    public RhemaDecisionsToolWindow(Project project) {
        this.project = project;
        this.content = new JPanel(new BorderLayout());
        this.listModel = new DefaultListModel<>();
        this.decisionsList = new JBList<>(listModel);
        
        initializeUI();
        loadDecisions();
    }
    
    /**
     * Initialize the UI components.
     */
    private void initializeUI() {
        // Set up the list
        decisionsList.setSelectionMode(ListSelectionModel.SINGLE_SELECTION);
        
        // Create scroll pane
        JBScrollPane scrollPane = new JBScrollPane(decisionsList);
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
        refreshButton.addActionListener(e -> loadDecisions());
        toolBar.add(refreshButton);
        
        // Add add decision button
        JButton addButton = new JButton("Add Decision");
        addButton.addActionListener(e -> addDecision());
        toolBar.add(addButton);
        
        // Add remove decision button
        JButton removeButton = new JButton("Remove Decision");
        removeButton.addActionListener(e -> removeDecision());
        toolBar.add(removeButton);
        
        return toolBar;
    }
    
    /**
     * Load decisions from the project.
     */
    private void loadDecisions() {
        try {
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Clear existing decisions
            listModel.clear();
            
            // TODO: Load actual decisions from Rhema files
            // For now, add placeholder decisions
            listModel.addElement("Use IntelliJ Platform for plugin development");
            listModel.addElement("Implement modular architecture for maintainability");
            listModel.addElement("Use Java for core functionality");
            listModel.addElement("Provide comprehensive UI components");
            listModel.addElement("Include extensive validation and error handling");
            
        } catch (Exception e) {
            // Handle error
            listModel.clear();
            listModel.addElement("Error loading decisions: " + e.getMessage());
        }
    }
    
    /**
     * Add a new decision.
     */
    private void addDecision() {
        String decisionText = JOptionPane.showInputDialog(
            content,
            "Enter decision text:",
            "Add Decision",
            JOptionPane.PLAIN_MESSAGE
        );
        
        if (decisionText != null && !decisionText.trim().isEmpty()) {
            listModel.addElement(decisionText.trim());
        }
    }
    
    /**
     * Remove the selected decision.
     */
    private void removeDecision() {
        int selectedIndex = decisionsList.getSelectedIndex();
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