package ai.fugue.rhema.execution;

import com.intellij.execution.configurations.RunConfigurationBase;
import com.intellij.execution.configurations.ConfigurationFactory;
import com.intellij.execution.configurations.RunProfileState;
import com.intellij.execution.runners.ExecutionEnvironment;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Run configuration for Rhema operations.
 * Provides configuration for running Rhema-specific operations.
 */
public class RhemaRunConfiguration extends RunConfigurationBase<Object> {
    public RhemaRunConfiguration(@NotNull Project project, @NotNull ConfigurationFactory factory, String name) {
        super(project, factory, name);
    }
    
    @Nullable
    @Override
    public RunProfileState getState(@NotNull ExecutionEnvironment env) {
        // TODO: Implement run profile state for Rhema operations
        return null;
    }
}