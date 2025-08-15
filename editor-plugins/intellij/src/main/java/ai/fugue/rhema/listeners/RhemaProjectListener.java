package ai.fugue.rhema.listeners;

import ai.fugue.rhema.services.RhemaProjectService;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.project.ProjectManagerListener;
import org.jetbrains.annotations.NotNull;

/**
 * Project listener for Rhema plugin.
 * Handles project lifecycle events and manages Rhema functionality.
 */
public class RhemaProjectListener implements ProjectManagerListener {
    
    @Override
    public void projectOpened(@NotNull Project project) {
        // TODO: Handle project opened event
        // This would involve:
        // - Initializing Rhema services for the project
        // - Scanning for Rhema files
        // - Setting up project-specific functionality
        
        System.out.println("Rhema: Project opened: " + project.getName());
    }
    
    @Override
    public void projectClosed(@NotNull Project project) {
        // TODO: Handle project closed event
        // This would involve:
        // - Cleaning up Rhema services for the project
        // - Saving project state
        // - Cleaning up resources
        
        System.out.println("Rhema: Project closed: " + project.getName());
    }
    
    @Override
    public void projectClosing(@NotNull Project project) {
        // TODO: Handle project closing event
        // This would involve:
        // - Preparing for project closure
        // - Saving any pending changes
        // - Cleaning up temporary resources
        
        System.out.println("Rhema: Project closing: " + project.getName());
    }
} 