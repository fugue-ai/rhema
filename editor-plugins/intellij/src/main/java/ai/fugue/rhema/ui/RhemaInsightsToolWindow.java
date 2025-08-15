package ai.fugue.rhema.ui;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.project.Project;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.components.JBTextArea;

import javax.swing.*;
import java.awt.*;

/**
 * Tool window for displaying Rhema insights.
 * Shows insights in a formatted view with management capabilities.
 */
public class RhemaInsightsToolWindow {
    
    private final Project project;
    private final JPanel content;
    private final JBTextArea insightsTextArea;
    
    public RhemaInsightsToolWindow(Project project) {
        this.project = project;
        this.content = new JPanel(new BorderLayout());
        this.insightsTextArea = new JBTextArea();
        
        initializeUI();
        loadInsights();
    }
    
    /**
     * Initialize the UI components.
     */
    private void initializeUI() {
        // Set up the text area
        insightsTextArea.setEditable(false);
        insightsTextArea.setFont(new Font("Monospaced", Font.PLAIN, 12));
        
        // Create scroll pane
        JBScrollPane scrollPane = new JBScrollPane(insightsTextArea);
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
        refreshButton.addActionListener(e -> loadInsights());
        toolBar.add(refreshButton);
        
        // Add add insight button
        JButton addButton = new JButton("Add Insight");
        addButton.addActionListener(e -> addInsight());
        toolBar.add(addButton);
        
        // Add export button
        JButton exportButton = new JButton("Export");
        exportButton.addActionListener(e -> exportInsights());
        toolBar.add(exportButton);
        
        return toolBar;
    }
    
    /**
     * Load insights from the project.
     */
    private void loadInsights() {
        try {
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            
            StringBuilder insightsInfo = new StringBuilder();
            insightsInfo.append("Rhema Insights\n");
            insightsInfo.append("==============\n\n");
            
            // TODO: Load actual insights from Rhema files
            // For now, add placeholder insights
            insightsInfo.append("1. Project Structure\n");
            insightsInfo.append("   - The project follows a modular architecture\n");
            insightsInfo.append("   - Clear separation of concerns between components\n");
            insightsInfo.append("   - Good use of dependency injection\n\n");
            
            insightsInfo.append("2. Code Quality\n");
            insightsInfo.append("   - Consistent coding standards\n");
            insightsInfo.append("   - Good error handling patterns\n");
            insightsInfo.append("   - Comprehensive logging\n\n");
            
            insightsInfo.append("3. Performance\n");
            insightsInfo.append("   - Efficient resource management\n");
            insightsInfo.append("   - Minimal memory footprint\n");
            insightsInfo.append("   - Fast startup times\n\n");
            
            insightsTextArea.setText(insightsInfo.toString());
            
        } catch (Exception e) {
            insightsTextArea.setText("Error loading insights: " + e.getMessage());
        }
    }
    
    /**
     * Add a new insight.
     */
    private void addInsight() {
        String insightText = JOptionPane.showInputDialog(
            content,
            "Enter insight text:",
            "Add Insight",
            JOptionPane.PLAIN_MESSAGE
        );
        
        if (insightText != null && !insightText.trim().isEmpty()) {
            // TODO: Add insight to the project
            JOptionPane.showMessageDialog(
                content,
                "Insight added: " + insightText,
                "Insight Added",
                JOptionPane.INFORMATION_MESSAGE
            );
        }
    }
    
    /**
     * Export insights.
     */
    private void exportInsights() {
        // TODO: Implement insights export functionality
        JOptionPane.showMessageDialog(
            content,
            "Insights export functionality will be implemented in a future version.",
            "Export Insights",
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