package ai.fugue.rhema.services;

import com.intellij.openapi.application.ApplicationManager;
import com.intellij.openapi.components.ApplicationService;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import org.jetbrains.annotations.NotNull;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Application service for the Rhema plugin.
 * Manages global plugin state and functionality.
 */
public class RhemaApplicationService implements ApplicationService {
    
    private static final Logger LOG = Logger.getInstance(RhemaApplicationService.class);
    private final Map<String, Object> globalState = new ConcurrentHashMap<>();
    private final Map<Project, RhemaProjectService> projectServices = new HashMap<>();
    
    @Override
    public void initialize() {
        LOG.info("Initializing Rhema Application Service");
        
        try {
            // Initialize global state
            globalState.put("initialized", true);
            globalState.put("version", "0.1.0");
            
            LOG.info("Rhema Application Service initialized successfully");
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Application Service", e);
        }
    }
    
    @Override
    public void dispose() {
        LOG.info("Disposing Rhema Application Service");
        
        try {
            // Clean up global state
            globalState.clear();
            projectServices.clear();
            
            LOG.info("Rhema Application Service disposed successfully");
        } catch (Exception e) {
            LOG.error("Failed to dispose Rhema Application Service", e);
        }
    }
    
    /**
     * Called when a project is opened.
     */
    public void onProjectOpened(@NotNull Project project) {
        LOG.info("Project opened: " + project.getName());
        
        try {
            // Initialize project-specific services
            RhemaProjectService projectService = project.getService(RhemaProjectService.class);
            projectServices.put(project, projectService);
            
            LOG.info("Project services initialized for: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to initialize project services for: " + project.getName(), e);
        }
    }
    
    /**
     * Called when a project is closed.
     */
    public void onProjectClosed(@NotNull Project project) {
        LOG.info("Project closed: " + project.getName());
        
        try {
            // Clean up project-specific services
            RhemaProjectService projectService = projectServices.remove(project);
            if (projectService != null) {
                projectService.dispose();
            }
            
            LOG.info("Project services cleaned up for: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to clean up project services for: " + project.getName(), e);
        }
    }
    
    /**
     * Get global state value.
     */
    public Object getGlobalState(String key) {
        return globalState.get(key);
    }
    
    /**
     * Set global state value.
     */
    public void setGlobalState(String key, Object value) {
        globalState.put(key, value);
    }
    
    /**
     * Get project service for a specific project.
     */
    public RhemaProjectService getProjectService(Project project) {
        return projectServices.get(project);
    }
    
    /**
     * Check if the service is initialized.
     */
    public boolean isInitialized() {
        return globalState.containsKey("initialized");
    }
    
    /**
     * Get the plugin version.
     */
    public String getVersion() {
        return (String) globalState.get("version");
    }
} 