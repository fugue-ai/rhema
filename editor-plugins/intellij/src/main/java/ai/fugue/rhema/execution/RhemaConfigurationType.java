package ai.fugue.rhema.execution;

import com.intellij.execution.configurations.ConfigurationType;
import com.intellij.execution.configurations.ConfigurationFactory;
import com.intellij.openapi.util.IconLoader;
import org.jetbrains.annotations.NotNull;

import javax.swing.*;

/**
 * Configuration type for Rhema run configurations.
 * Provides configuration type for Rhema-specific operations.
 */
public class RhemaConfigurationType implements ConfigurationType {
    
    private final ConfigurationFactory factory;
    
    public RhemaConfigurationType() {
        this.factory = new RhemaConfigurationFactory();
    }
    
    @Override
    public String getDisplayName() {
        return "Rhema";
    }
    
    @Override
    public String getConfigurationTypeDescription() {
        return "Rhema run configuration";
    }
    
    @Override
    public Icon getIcon() {
        // TODO: Provide a custom icon for Rhema run configurations
        return null;
    }
    
    @NotNull
    @Override
    public String getId() {
        return "RHEMA_RUN_CONFIGURATION";
    }
    
    @Override
    public ConfigurationFactory[] getConfigurationFactories() {
        return new ConfigurationFactory[] { factory };
    }
}