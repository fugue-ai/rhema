package ai.fugue.rhema.execution;

import com.intellij.execution.configurations.ConfigurationFactory;
import com.intellij.execution.configurations.ConfigurationType;
import org.jetbrains.annotations.NotNull;

/**
 * Configuration factory for Rhema run configurations.
 * Provides configuration factory for Rhema-specific operations.
 */
public class RhemaConfigurationFactory extends ConfigurationFactory {
    public RhemaConfigurationFactory() {
        super(new RhemaConfigurationType());
    }
    
    @NotNull
    @Override
    public String getName() {
        return "Rhema";
    }
}