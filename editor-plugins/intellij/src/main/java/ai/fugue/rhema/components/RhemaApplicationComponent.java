package ai.fugue.rhema.components;

import ai.fugue.rhema.services.RhemaApplicationService;
import com.intellij.openapi.application.ApplicationManager;
import com.intellij.openapi.components.ApplicationComponent;
import com.intellij.openapi.components.ServiceManager;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.project.ProjectManager;
import com.intellij.openapi.project.ProjectManagerListener;
import org.jetbrains.annotations.NotNull;

/**
 * Main application component for the Rhema plugin.
 * Manages global plugin state and initialization.
 */
public class RhemaApplicationComponent implements ApplicationComponent {
    
    private static final Logger LOG = Logger.getInstance(RhemaApplicationComponent.class);
    private RhemaApplicationService applicationService;
    
    @Override
    public void initComponent() {
        LOG.info("Initializing Rhema Application Component");
        
        try {
            // Initialize application service
            applicationService = ServiceManager.getService(RhemaApplicationService.class);
            applicationService.initialize();
            
            // Register project listener
            ProjectManager.getInstance().addProjectManagerListener(new ProjectManagerListener() {
                @Override
                public void projectOpened(@NotNull Project project) {
                    LOG.info("Project opened: " + project.getName());
                    applicationService.onProjectOpened(project);
                }
                
                @Override
                public void projectClosed(@NotNull Project project) {
                    LOG.info("Project closed: " + project.getName());
                    applicationService.onProjectClosed(project);
                }
            });
            
            LOG.info("Rhema Application Component initialized successfully");
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Application Component", e);
        }
    }
    
    @Override
    public void disposeComponent() {
        LOG.info("Disposing Rhema Application Component");
        
        try {
            if (applicationService != null) {
                applicationService.dispose();
            }
            LOG.info("Rhema Application Component disposed successfully");
        } catch (Exception e) {
            LOG.error("Failed to dispose Rhema Application Component", e);
        }
    }
    
    @NotNull
    @Override
    public String getComponentName() {
        return "RhemaApplicationComponent";
    }
    
    /**
     * Get the application service instance.
     */
    public RhemaApplicationService getApplicationService() {
        return applicationService;
    }
} 