package ai.fugue.rhema.components;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.components.ProjectComponent;
import com.intellij.openapi.components.ServiceManager;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.openapi.vfs.VirtualFileManager;
import com.intellij.openapi.vfs.newvfs.BulkFileListener;
import com.intellij.openapi.vfs.newvfs.events.VFileEvent;
import org.jetbrains.annotations.NotNull;

import java.util.List;

/**
 * Project component for the Rhema plugin.
 * Manages project-specific functionality and state.
 */
public class RhemaProjectComponent implements ProjectComponent {
    
    private static final Logger LOG = Logger.getInstance(RhemaProjectComponent.class);
    private final Project project;
    private RhemaProjectService projectService;
    
    public RhemaProjectComponent(@NotNull Project project) {
        this.project = project;
    }
    
    @Override
    public void initComponent() {
        LOG.info("Initializing Rhema Project Component for project: " + project.getName());
        
        try {
            // Initialize project service
            projectService = ServiceManager.getService(project, RhemaProjectService.class);
            projectService.initialize();
            
            // Register file listener for Rhema files
            VirtualFileManager.getInstance().addVirtualFileListener(new BulkFileListener() {
                @Override
                public void after(@NotNull List<? extends VFileEvent> events) {
                    for (VFileEvent event : events) {
                        VirtualFile file = event.getFile();
                        if (file != null && isRhemaFile(file)) {
                            LOG.info("Rhema file changed: " + file.getPath());
                            projectService.onRhemaFileChanged(file);
                        }
                    }
                }
            });
            
            LOG.info("Rhema Project Component initialized successfully for project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Project Component for project: " + project.getName(), e);
        }
    }
    
    @Override
    public void disposeComponent() {
        LOG.info("Disposing Rhema Project Component for project: " + project.getName());
        
        try {
            if (projectService != null) {
                projectService.dispose();
            }
            LOG.info("Rhema Project Component disposed successfully for project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to dispose Rhema Project Component for project: " + project.getName(), e);
        }
    }
    
    @NotNull
    @Override
    public String getComponentName() {
        return "RhemaProjectComponent";
    }
    
    @Override
    public void projectOpened() {
        LOG.info("Project opened: " + project.getName());
        if (projectService != null) {
            projectService.onProjectOpened();
        }
    }
    
    @Override
    public void projectClosed() {
        LOG.info("Project closed: " + project.getName());
        if (projectService != null) {
            projectService.onProjectClosed();
        }
    }
    
    /**
     * Check if a file is a Rhema file.
     */
    private boolean isRhemaFile(VirtualFile file) {
        String name = file.getName();
        return name.endsWith(".rhema.yml") || 
               name.endsWith(".rhema.yaml") || 
               name.equals("rhema.yml") || 
               name.equals("rhema.yaml");
    }
    
    /**
     * Get the project service instance.
     */
    public RhemaProjectService getProjectService() {
        return projectService;
    }
    
    /**
     * Get the associated project.
     */
    public Project getProject() {
        return project;
    }
} 