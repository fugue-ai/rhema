package ai.fugue.rhema.language;

import com.intellij.openapi.fileTypes.LanguageFileType;
import com.intellij.openapi.util.NlsContexts;
import com.intellij.openapi.util.NlsSafe;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;

/**
 * File type for Rhema YAML files.
 * Defines file properties, icons, and language association.
 */
public class RhemaFileType extends LanguageFileType {
    
    public static final RhemaFileType INSTANCE = new RhemaFileType();
    
    private RhemaFileType() {
        super(RhemaLanguageSupport.INSTANCE);
    }
    
    @NotNull
    @Override
    public String getName() {
        return "Rhema YAML";
    }
    
    @NotNull
    @Override
    public String getDescription() {
        return "Rhema YAML configuration files";
    }
    
    @NotNull
    @Override
    public String getDefaultExtension() {
        return "rhema.yml";
    }
    
    @Nullable
    @Override
    public Icon getIcon() {
        // Use a generic YAML icon for now
        // TODO: Create custom Rhema icon
        return com.intellij.icons.AllIcons.FileTypes.Yaml;
    }
    
    @Override
    public boolean isBinary() {
        return false;
    }
    
    @Override
    public boolean isReadOnly() {
        return false;
    }
} 