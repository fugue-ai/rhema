package ai.fugue.rhema.listeners;

import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.openapi.vfs.VirtualFileListener;
import com.intellij.openapi.vfs.newvfs.events.VFileEvent;
import org.jetbrains.annotations.NotNull;

import java.util.List;

/**
 * File listener for Rhema plugin.
 * Handles file system events for Rhema files.
 */
public class RhemaFileListener implements VirtualFileListener {
    
    @Override
    public void after(@NotNull List<? extends VFileEvent> events) {
        for (VFileEvent event : events) {
            VirtualFile file = event.getFile();
            if (file != null && isRhemaFile(file)) {
                handleRhemaFileEvent(event);
            }
        }
    }
    
    /**
     * Handle Rhema file events.
     */
    private void handleRhemaFileEvent(VFileEvent event) {
        VirtualFile file = event.getFile();
        
        // TODO: Handle Rhema file events
        // This would involve:
        // - Detecting file changes
        // - Triggering validation
        // - Updating context information
        // - Refreshing UI components
        
        System.out.println("Rhema: File event: " + event.getFile().getPath());
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
} 