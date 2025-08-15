package ai.fugue.rhema.settings;

import com.intellij.openapi.options.Configurable;
import com.intellij.openapi.options.ConfigurationException;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.Nls;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import java.awt.*;

/**
 * Project configurable for Rhema project-specific settings.
 * Provides configuration options for Rhema in the current project.
 */
public class RhemaProjectConfigurable implements Configurable {
    
    private final Project project;
    private JPanel mainPanel;
    private JCheckBox enableProjectValidation;
    private JCheckBox enableProjectCompletion;
    private JTextField projectScopeName;
    private JSpinner projectMaxContextSize;
    private JTextArea projectDescription;
    
    public RhemaProjectConfigurable(Project project) {
        this.project = project;
    }
    
    @Nls(capitalization = Nls.Capitalization.Title)
    @Override
    public String getDisplayName() {
        return "Rhema";
    }
    
    @Nullable
    @Override
    public JComponent createComponent() {
        if (mainPanel == null) {
            mainPanel = new JPanel(new BorderLayout());
            
            // Create settings panel
            JPanel settingsPanel = new JPanel(new GridBagLayout());
            GridBagConstraints gbc = new GridBagConstraints();
            gbc.insets = new Insets(5, 5, 5, 5);
            gbc.anchor = GridBagConstraints.WEST;
            
            // Project validation setting
            enableProjectValidation = new JCheckBox("Enable project validation");
            enableProjectValidation.setSelected(true);
            gbc.gridx = 0;
            gbc.gridy = 0;
            gbc.gridwidth = 2;
            settingsPanel.add(enableProjectValidation, gbc);
            
            // Project completion setting
            enableProjectCompletion = new JCheckBox("Enable project completion");
            enableProjectCompletion.setSelected(true);
            gbc.gridy = 1;
            settingsPanel.add(enableProjectCompletion, gbc);
            
            // Project scope name setting
            JLabel scopeLabel = new JLabel("Project scope name:");
            gbc.gridx = 0;
            gbc.gridy = 2;
            gbc.gridwidth = 1;
            settingsPanel.add(scopeLabel, gbc);
            
            projectScopeName = new JTextField(project.getName() + "-scope", 20);
            gbc.gridx = 1;
            gbc.fill = GridBagConstraints.HORIZONTAL;
            settingsPanel.add(projectScopeName, gbc);
            
            // Project max context size setting
            JLabel sizeLabel = new JLabel("Project max context size:");
            gbc.gridx = 0;
            gbc.gridy = 3;
            gbc.gridwidth = 1;
            gbc.fill = GridBagConstraints.NONE;
            settingsPanel.add(sizeLabel, gbc);
            
            SpinnerNumberModel sizeModel = new SpinnerNumberModel(500, 100, 5000, 100);
            projectMaxContextSize = new JSpinner(sizeModel);
            gbc.gridx = 1;
            gbc.fill = GridBagConstraints.HORIZONTAL;
            settingsPanel.add(projectMaxContextSize, gbc);
            
            // Project description setting
            JLabel descLabel = new JLabel("Project description:");
            gbc.gridx = 0;
            gbc.gridy = 4;
            gbc.gridwidth = 1;
            gbc.fill = GridBagConstraints.NONE;
            settingsPanel.add(descLabel, gbc);
            
            projectDescription = new JTextArea(3, 30);
            projectDescription.setLineWrap(true);
            projectDescription.setWrapStyleWord(true);
            JScrollPane descScrollPane = new JScrollPane(projectDescription);
            gbc.gridx = 1;
            gbc.fill = GridBagConstraints.BOTH;
            settingsPanel.add(descScrollPane, gbc);
            
            // Add settings panel to main panel
            mainPanel.add(settingsPanel, BorderLayout.CENTER);
            
            // Add description
            JTextArea description = new JTextArea(
                "Rhema Project Settings\n\n" +
                "Configure project-specific settings for the Rhema plugin. These settings apply only to this project."
            );
            description.setEditable(false);
            description.setBackground(mainPanel.getBackground());
            description.setFont(description.getFont().deriveFont(Font.PLAIN));
            mainPanel.add(description, BorderLayout.SOUTH);
        }
        
        return mainPanel;
    }
    
    @Override
    public boolean isModified() {
        // TODO: Implement modification detection
        return false;
    }
    
    @Override
    public void apply() throws ConfigurationException {
        // TODO: Save project settings
        // This would involve saving the settings to project-specific storage
    }
    
    @Override
    public void reset() {
        // TODO: Reset project settings to defaults
        if (enableProjectValidation != null) {
            enableProjectValidation.setSelected(true);
        }
        if (enableProjectCompletion != null) {
            enableProjectCompletion.setSelected(true);
        }
        if (projectScopeName != null) {
            projectScopeName.setText(project.getName() + "-scope");
        }
        if (projectMaxContextSize != null) {
            projectMaxContextSize.setValue(500);
        }
        if (projectDescription != null) {
            projectDescription.setText("");
        }
    }
    
    @Override
    public void disposeUIResources() {
        mainPanel = null;
    }
} 