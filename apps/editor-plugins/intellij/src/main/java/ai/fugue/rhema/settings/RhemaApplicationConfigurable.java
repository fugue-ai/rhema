package ai.fugue.rhema.settings;

import com.intellij.openapi.options.Configurable;
import com.intellij.openapi.options.ConfigurationException;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.Nls;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import java.awt.*;

/**
 * Application configurable for Rhema global settings.
 * Provides configuration options for the Rhema plugin.
 */
public class RhemaApplicationConfigurable implements Configurable {
    
    private JPanel mainPanel;
    private JCheckBox enableAutoValidation;
    private JCheckBox enableAutoCompletion;
    private JCheckBox enableSyntaxHighlighting;
    private JTextField defaultScopeName;
    private JSpinner maxContextSize;
    
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
            
            // Auto validation setting
            enableAutoValidation = new JCheckBox("Enable automatic validation");
            enableAutoValidation.setSelected(true);
            gbc.gridx = 0;
            gbc.gridy = 0;
            gbc.gridwidth = 2;
            settingsPanel.add(enableAutoValidation, gbc);
            
            // Auto completion setting
            enableAutoCompletion = new JCheckBox("Enable automatic completion");
            enableAutoCompletion.setSelected(true);
            gbc.gridy = 1;
            settingsPanel.add(enableAutoCompletion, gbc);
            
            // Syntax highlighting setting
            enableSyntaxHighlighting = new JCheckBox("Enable syntax highlighting");
            enableSyntaxHighlighting.setSelected(true);
            gbc.gridy = 2;
            settingsPanel.add(enableSyntaxHighlighting, gbc);
            
            // Default scope name setting
            JLabel scopeLabel = new JLabel("Default scope name:");
            gbc.gridx = 0;
            gbc.gridy = 3;
            gbc.gridwidth = 1;
            settingsPanel.add(scopeLabel, gbc);
            
            defaultScopeName = new JTextField("my-scope", 20);
            gbc.gridx = 1;
            gbc.fill = GridBagConstraints.HORIZONTAL;
            settingsPanel.add(defaultScopeName, gbc);
            
            // Max context size setting
            JLabel sizeLabel = new JLabel("Maximum context size:");
            gbc.gridx = 0;
            gbc.gridy = 4;
            gbc.gridwidth = 1;
            gbc.fill = GridBagConstraints.NONE;
            settingsPanel.add(sizeLabel, gbc);
            
            SpinnerNumberModel sizeModel = new SpinnerNumberModel(1000, 100, 10000, 100);
            maxContextSize = new JSpinner(sizeModel);
            gbc.gridx = 1;
            gbc.fill = GridBagConstraints.HORIZONTAL;
            settingsPanel.add(maxContextSize, gbc);
            
            // Add settings panel to main panel
            mainPanel.add(settingsPanel, BorderLayout.CENTER);
            
            // Add description
            JTextArea description = new JTextArea(
                "Rhema Plugin Settings\n\n" +
                "Configure global settings for the Rhema plugin. These settings apply to all projects."
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
        // TODO: Save settings
        // This would involve saving the settings to persistent storage
    }
    
    @Override
    public void reset() {
        // TODO: Reset settings to defaults
        if (enableAutoValidation != null) {
            enableAutoValidation.setSelected(true);
        }
        if (enableAutoCompletion != null) {
            enableAutoCompletion.setSelected(true);
        }
        if (enableSyntaxHighlighting != null) {
            enableSyntaxHighlighting.setSelected(true);
        }
        if (defaultScopeName != null) {
            defaultScopeName.setText("my-scope");
        }
        if (maxContextSize != null) {
            maxContextSize.setValue(1000);
        }
    }
    
    @Override
    public void disposeUIResources() {
        mainPanel = null;
    }
} 