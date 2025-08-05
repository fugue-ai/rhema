package ai.fugue.rhema.ui;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.project.Project;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.treeStructure.Tree;

import javax.swing.*;
import javax.swing.tree.DefaultMutableTreeNode;
import javax.swing.tree.DefaultTreeModel;
import java.awt.*;

/**
 * Tool window for displaying Rhema patterns.
 * Shows patterns in a tree view with management capabilities.
 */
public class RhemaPatternsToolWindow {
    
    private final Project project;
    private final JPanel content;
    private final Tree patternsTree;
    
    public RhemaPatternsToolWindow(Project project) {
        this.project = project;
        this.content = new JPanel(new BorderLayout());
        this.patternsTree = new Tree();
        
        initializeUI();
        loadPatterns();
    }
    
    /**
     * Initialize the UI components.
     */
    private void initializeUI() {
        // Set up the tree
        patternsTree.setRootVisible(false);
        patternsTree.setShowsRootHandles(true);
        
        // Create scroll pane
        JBScrollPane scrollPane = new JBScrollPane(patternsTree);
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
        refreshButton.addActionListener(e -> loadPatterns());
        toolBar.add(refreshButton);
        
        // Add add pattern button
        JButton addButton = new JButton("Add Pattern");
        addButton.addActionListener(e -> addPattern());
        toolBar.add(addButton);
        
        // Add expand all button
        JButton expandAllButton = new JButton("Expand All");
        expandAllButton.addActionListener(e -> expandAll());
        toolBar.add(expandAllButton);
        
        // Add collapse all button
        JButton collapseAllButton = new JButton("Collapse All");
        collapseAllButton.addActionListener(e -> collapseAll());
        toolBar.add(collapseAllButton);
        
        return toolBar;
    }
    
    /**
     * Load patterns from the project.
     */
    private void loadPatterns() {
        try {
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Create root node
            DefaultMutableTreeNode root = new DefaultMutableTreeNode("Rhema Patterns");
            
            // TODO: Load actual patterns from Rhema files
            // For now, add placeholder patterns
            DefaultMutableTreeNode architecturalPatterns = new DefaultMutableTreeNode("Architectural Patterns");
            architecturalPatterns.add(new DefaultMutableTreeNode("MVC Pattern"));
            architecturalPatterns.add(new DefaultMutableTreeNode("Repository Pattern"));
            architecturalPatterns.add(new DefaultMutableTreeNode("Factory Pattern"));
            root.add(architecturalPatterns);
            
            DefaultMutableTreeNode designPatterns = new DefaultMutableTreeNode("Design Patterns");
            designPatterns.add(new DefaultMutableTreeNode("Singleton Pattern"));
            designPatterns.add(new DefaultMutableTreeNode("Observer Pattern"));
            designPatterns.add(new DefaultMutableTreeNode("Strategy Pattern"));
            root.add(designPatterns);
            
            DefaultMutableTreeNode codePatterns = new DefaultMutableTreeNode("Code Patterns");
            codePatterns.add(new DefaultMutableTreeNode("Error Handling"));
            codePatterns.add(new DefaultMutableTreeNode("Logging"));
            codePatterns.add(new DefaultMutableTreeNode("Configuration"));
            root.add(codePatterns);
            
            // Set the tree model
            DefaultTreeModel model = new DefaultTreeModel(root);
            patternsTree.setModel(model);
            
        } catch (Exception e) {
            // Handle error
            DefaultMutableTreeNode errorNode = new DefaultMutableTreeNode("Error loading patterns: " + e.getMessage());
            DefaultTreeModel model = new DefaultTreeModel(errorNode);
            patternsTree.setModel(model);
        }
    }
    
    /**
     * Add a new pattern.
     */
    private void addPattern() {
        String patternName = JOptionPane.showInputDialog(
            content,
            "Enter pattern name:",
            "Add Pattern",
            JOptionPane.PLAIN_MESSAGE
        );
        
        if (patternName != null && !patternName.trim().isEmpty()) {
            // TODO: Add pattern to the project
            JOptionPane.showMessageDialog(
                content,
                "Pattern added: " + patternName,
                "Pattern Added",
                JOptionPane.INFORMATION_MESSAGE
            );
        }
    }
    
    /**
     * Expand all nodes in the tree.
     */
    private void expandAll() {
        for (int i = 0; i < patternsTree.getRowCount(); i++) {
            patternsTree.expandRow(i);
        }
    }
    
    /**
     * Collapse all nodes in the tree.
     */
    private void collapseAll() {
        int row = patternsTree.getRowCount() - 1;
        while (row >= 0) {
            patternsTree.collapseRow(row);
            row--;
        }
    }
    
    /**
     * Get the content panel.
     */
    public JComponent getContent() {
        return content;
    }
} 