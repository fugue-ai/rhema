package ai.fugue.rhema.components;

import com.intellij.openapi.components.ModuleComponent;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.module.Module;
import org.jetbrains.annotations.NotNull;

/**
 * Module component for the Rhema plugin.
 * Manages module-specific functionality and state.
 */
public class RhemaModuleComponent implements ModuleComponent {
    
    private static final Logger LOG = Logger.getInstance(RhemaModuleComponent.class);
    private final Module module;
    
    public RhemaModuleComponent(@NotNull Module module) {
        this.module = module;
    }
    
    @Override
    public void initComponent() {
        LOG.info("Initializing Rhema Module Component for module: " + module.getName());
        
        try {
            // Initialize module-specific functionality
            LOG.info("Rhema Module Component initialized successfully for module: " + module.getName());
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Module Component for module: " + module.getName(), e);
        }
    }
    
    @Override
    public void disposeComponent() {
        LOG.info("Disposing Rhema Module Component for module: " + module.getName());
        
        try {
            // Clean up module-specific resources
            LOG.info("Rhema Module Component disposed successfully for module: " + module.getName());
        } catch (Exception e) {
            LOG.error("Failed to dispose Rhema Module Component for module: " + module.getName(), e);
        }
    }
    
    @NotNull
    @Override
    public String getComponentName() {
        return "RhemaModuleComponent";
    }
    
    @Override
    public void moduleAdded() {
        LOG.info("Module added: " + module.getName());
    }
    
    /**
     * Get the associated module.
     */
    public Module getModule() {
        return module;
    }
} 