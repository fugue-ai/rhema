package ai.fugue.rhema.ui;

import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowFactory;
import com.intellij.ui.content.Content;
import com.intellij.ui.content.ContentFactory;
import org.jetbrains.annotations.NotNull;

/**
 * Tool window factory for Rhema Patterns.
 * Creates the patterns tool window for viewing Rhema patterns.
 */
public class RhemaPatternsToolWindowFactory implements ToolWindowFactory {
    
    @Override
    public void createToolWindowContent(@NotNull Project project, @NotNull ToolWindow toolWindow) {
        // Create the patterns tool window content
        RhemaPatternsToolWindow patternsToolWindow = new RhemaPatternsToolWindow(project);
        ContentFactory contentFactory = ContentFactory.getInstance();
        Content content = contentFactory.createContent(patternsToolWindow.getContent(), "", false);
        toolWindow.getContentManager().addContent(content);
    }
    
    @Override
    public boolean shouldBeAvailable(@NotNull Project project) {
        // Show the tool window if the project has Rhema files
        return true; // TODO: Check if project has Rhema files
    }
} 