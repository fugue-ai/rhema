package ai.fugue.rhema.language;

import com.intellij.lexer.Lexer;
import com.intellij.openapi.editor.colors.TextAttributesKey;
import com.intellij.openapi.fileTypes.SyntaxHighlighterBase;
import com.intellij.psi.tree.IElementType;
import org.jetbrains.annotations.NotNull;

import java.util.HashMap;
import java.util.Map;

/**
 * Syntax highlighter for Rhema YAML files.
 * Provides syntax highlighting for Rhema-specific YAML syntax.
 */
public class RhemaSyntaxHighlighter extends SyntaxHighlighterBase {
    
    private static final Map<IElementType, TextAttributesKey> ATTRIBUTES = new HashMap<>();
    
    static {
        // TODO: Define syntax highlighting attributes for Rhema YAML
        // This would include highlighting for Rhema-specific keywords, values, etc.
    }
    
    @NotNull
    @Override
    public Lexer getHighlightingLexer() {
        // TODO: Implement custom lexer for Rhema YAML
        // For now, we'll use the default YAML lexer
        return new RhemaYamlLexer();
    }
    
    @NotNull
    @Override
    public TextAttributesKey[] getTokenHighlights(IElementType tokenType) {
        return pack(ATTRIBUTES.get(tokenType));
    }
} 