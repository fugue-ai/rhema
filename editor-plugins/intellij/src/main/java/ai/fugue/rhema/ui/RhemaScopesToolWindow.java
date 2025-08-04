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
 * Tool window for displaying Rhema scopes.
 * Shows scopes in a tree view with navigation capabilities.
 */
public class RhemaScopesToolWindow {
    
    private final Project project;
    private final JPanel content;
    private final Tree scopesTree;
    
    public RhemaScopesToolWindow(Project project) {
        this.project = project;
        this.content = new JPanel(new BorderLayout());
        this.scopesTree = new Tree();
        
        initializeUI();
        loadScopes();
    }
    
    /**
     * Initialize the UI components.
     */
    private void initializeUI() {
        // Set up the tree
        scopesTree.setRootVisible(false);
        scopesTree.setShowsRootHandles(true);
        
        // Create scroll pane
        JBScrollPane scrollPane = new JBScrollPane(scopesTree);
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
        refreshButton.addActionListener(e -> loadScopes());
        toolBar.add(refreshButton);
        
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
     * Load scopes from the project.
     */
    private void loadScopes() {
        try {
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            // Create root node
            DefaultMutableTreeNode root = new DefaultMutableTreeNode("Rhema Scopes");
            
            // TODO: Load actual scopes from Rhema files
            // For now, add placeholder nodes
            DefaultMutableTreeNode scope1 = new DefaultMutableTreeNode("my-scope");
            scope1.add(new DefaultMutableTreeNode("context"));
            scope1.add(new DefaultMutableTreeNode("todos"));
            scope1.add(new DefaultMutableTreeNode("insights"));
            scope1.add(new DefaultMutableTreeNode("patterns"));
            scope1.add(new DefaultMutableTreeNode("decisions"));
            root.add(scope1);
            
            // Set the tree model
            DefaultTreeModel model = new DefaultTreeModel(root);
            scopesTree.setModel(model);
            
        } catch (Exception e) {
            // Handle error
            DefaultMutableTreeNode errorNode = new DefaultMutableTreeNode("Error loading scopes: " + e.getMessage());
            DefaultTreeModel model = new DefaultTreeModel(errorNode);
            scopesTree.setModel(model);
        }
    }
    
    /**
     * Expand all nodes in the tree.
     */
    private void expandAll() {
        for (int i = 0; i < scopesTree.getRowCount(); i++) {
            scopesTree.expandRow(i);
        }
    }
    
    /**
     * Collapse all nodes in the tree.
     */
    private void collapseAll() {
        int row = scopesTree.getRowCount() - 1;
        while (row >= 0) {
            scopesTree.collapseRow(row);
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