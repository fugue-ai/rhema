package ai.fugue.rhema.ui;

import com.intellij.ide.projectView.ProjectViewNode;
import com.intellij.ide.projectView.ProjectViewNodeDecorator;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.ui.ColoredTreeCellRenderer;
import org.jetbrains.annotations.NotNull;

/**
 * Project view decorator for Rhema files.
 * Provides visual decorations for Rhema files in the project view.
 */
public class RhemaProjectViewDecorator implements ProjectViewNodeDecorator {
    
    @Override
    public void decorate(ProjectViewNode<?> node, ColoredTreeCellRenderer cellRenderer) {
        // TODO: Implement Rhema-specific decorations
        // This would involve:
        // - Adding icons for Rhema files
        // - Adding color coding for different Rhema elements
        // - Adding status indicators
        
        VirtualFile file = node.getVirtualFile();
        if (file != null && isRhemaFile(file)) {
            // Add Rhema-specific decoration
            cellRenderer.setIcon(null); // TODO: Add Rhema icon
            cellRenderer.append(" [Rhema]", cellRenderer.getAttributesKey());
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
} 