package ai.fugue.rhema.listeners;

import com.intellij.openapi.editor.Document;
import com.intellij.openapi.editor.event.DocumentEvent;
import com.intellij.openapi.editor.event.DocumentListener;
import com.intellij.openapi.fileEditor.FileDocumentManager;
import com.intellij.openapi.vfs.VirtualFile;
import org.jetbrains.annotations.NotNull;

/**
 * Document listener for Rhema plugin.
 * Handles document changes for Rhema files.
 */
public class RhemaDocumentListener implements DocumentListener {
    
    @Override
    public void documentChanged(@NotNull DocumentEvent event) {
        Document document = event.getDocument();
        VirtualFile file = FileDocumentManager.getInstance().getFile(document);
        
        if (file != null && isRhemaFile(file)) {
            handleRhemaDocumentChange(event);
        }
    }
    
    /**
     * Handle Rhema document changes.
     */
    private void handleRhemaDocumentChange(DocumentEvent event) {
        // TODO: Handle Rhema document changes
        // This would involve:
        // - Detecting changes in Rhema files
        // - Triggering real-time validation
        // - Updating IntelliSense
        // - Refreshing UI components
        
        System.out.println("Rhema: Document changed");
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