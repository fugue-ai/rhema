package ai.fugue.rhema.language;

import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighter;
import com.intellij.openapi.fileTypes.SyntaxHighlighterFactory;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

/**
 * Syntax highlighter factory for Rhema files.
 * Provides syntax highlighting for Rhema YAML files.
 */
public class RhemaSyntaxHighlighterFactory extends SyntaxHighlighterFactory {
    
    @NotNull
    @Override
    public SyntaxHighlighter getSyntaxHighlighter(@Nullable Project project, @Nullable VirtualFile virtualFile) {
        return new RhemaSyntaxHighlighter();
    }
} 