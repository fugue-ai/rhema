package ai.fugue.rhema.language;

import com.intellij.openapi.fileTypes.FileTypeConsumer;
import com.intellij.openapi.fileTypes.FileTypeFactory;
import org.jetbrains.annotations.NotNull;

/**
 * File type factory for Rhema files.
 * Registers Rhema file types with IntelliJ.
 */
public class RhemaFileTypeFactory extends FileTypeFactory {
    
    @Override
    public void createFileTypes(@NotNull FileTypeConsumer consumer) {
        consumer.consume(RhemaFileType.INSTANCE, "rhema.yml;rhema.yaml");
    }
} 