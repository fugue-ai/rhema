package ai.fugue.rhema.language;

import com.intellij.lang.Language;
import org.jetbrains.annotations.NotNull;

/**
 * Language support for Rhema YAML files.
 * Provides language definition for Rhema-specific YAML files.
 */
public class RhemaLanguageSupport extends Language {
    
    public static final RhemaLanguageSupport INSTANCE = new RhemaLanguageSupport();
    
    private RhemaLanguageSupport() {
        super("RHEMA_YAML");
    }
    
    @NotNull
    @Override
    public String getDisplayName() {
        return "Rhema YAML";
    }
    
    @Override
    public boolean isCaseSensitive() {
        return true;
    }
} 