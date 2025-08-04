package ai.fugue.rhema.ui;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.project.Project;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.components.JBTextArea;

import javax.swing.*;
import java.awt.*;

/**
 * Tool window for displaying Rhema context information.
 * Shows context details in a formatted view.
 */
public class RhemaContextToolWindow {
    
    private final Project project;
    private final JPanel content;
    private final JBTextArea contextTextArea;
    
    public RhemaContextToolWindow(Project project) {
        this.project = project;
        this.content = new JPanel(new BorderLayout());
        this.contextTextArea = new JBTextArea();
        
        initializeUI();
        loadContext();
    }
    
    /**
     * Initialize the UI components.
     */
    private void initializeUI() {
        // Set up the text area
        contextTextArea.setEditable(false);
        contextTextArea.setFont(new Font("Monospaced", Font.PLAIN, 12));
        
        // Create scroll pane
        JBScrollPane scrollPane = new JBScrollPane(contextTextArea);
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
        refreshButton.addActionListener(e -> loadContext());
        toolBar.add(refreshButton);
        
        // Add export button
        JButton exportButton = new JButton("Export");
        exportButton.addActionListener(e -> exportContext());
        toolBar.add(exportButton);
        
        return toolBar;
    }
    
    /**
     * Load context from the project.
     */
    private void loadContext() {
        try {
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            StringBuilder contextInfo = new StringBuilder();
            contextInfo.append("Rhema Context Information\n");
            contextInfo.append("========================\n\n");
            
            // Add project information
            contextInfo.append("Project: ").append(project.getName()).append("\n");
            contextInfo.append("Rhema Files: ").append(projectService.getRhemaFiles().size()).append("\n\n");
            
            // Add context details
            contextInfo.append("Context Details:\n");
            contextInfo.append("---------------\n");
            contextInfo.append("Scopes: 0\n");
            contextInfo.append("Todos: 0\n");
            contextInfo.append("Insights: 0\n");
            contextInfo.append("Patterns: 0\n");
            contextInfo.append("Decisions: 0\n\n");
            
            // Add file list
            contextInfo.append("Rhema Files:\n");
            contextInfo.append("------------\n");
            projectService.getRhemaFiles().forEach((path, file) -> {
                contextInfo.append("- ").append(file.getName()).append(" (").append(path).append(")\n");
            });
            
            contextTextArea.setText(contextInfo.toString());
            
        } catch (Exception e) {
            contextTextArea.setText("Error loading context: " + e.getMessage());
        }
    }
    
    /**
     * Export context information.
     */
    private void exportContext() {
        // TODO: Implement context export functionality
        JOptionPane.showMessageDialog(
            content,
            "Context export functionality will be implemented in a future version.",
            "Export Context",
            JOptionPane.INFORMATION_MESSAGE
        );
    }
    
    /**
     * Get the content panel.
     */
    public JComponent getContent() {
        return content;
    }
} 