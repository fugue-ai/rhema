package ai.fugue.rhema.intellisense;

import com.intellij.codeInsight.completion.*;
import com.intellij.codeInsight.lookup.LookupElementBuilder;
import com.intellij.patterns.PlatformPatterns;
import com.intellij.util.ProcessingContext;
import org.jetbrains.annotations.NotNull;

/**
 * Completion contributor for YAML files with Rhema-specific completions.
 * Provides IntelliSense functionality for Rhema keywords in regular YAML files.
 */
public class RhemaYamlCompletionContributor extends CompletionContributor {
    
    public RhemaYamlCompletionContributor() {
        // Register completion providers for YAML files
        extend(CompletionType.BASIC, 
               PlatformPatterns.psiElement().withLanguage(com.intellij.lang.yaml.YAMLLanguage.INSTANCE),
               new RhemaYamlCompletionProvider());
    }
    
    /**
     * Completion provider for YAML files with Rhema completions.
     */
    private static class RhemaYamlCompletionProvider extends CompletionProvider<CompletionParameters> {
        
        @Override
        protected void addCompletions(@NotNull CompletionParameters parameters,
                                    @NotNull ProcessingContext context,
                                    @NotNull CompletionResultSet result) {
            
            // Add Rhema-specific keywords for YAML files
            result.addElement(LookupElementBuilder.create("rhema")
                    .withTypeText("Rhema configuration")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of rhema keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("scope")
                    .withTypeText("Rhema scope definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of scope keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("context")
                    .withTypeText("Rhema context definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of context keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("todos")
                    .withTypeText("Rhema todos definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of todos keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("insights")
                    .withTypeText("Rhema insights definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of insights keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("patterns")
                    .withTypeText("Rhema patterns definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of patterns keyword
                    }));
            
            result.addElement(LookupElementBuilder.create("decisions")
                    .withTypeText("Rhema decisions definition")
                    .withInsertHandler((context1, item) -> {
                        // Handle insertion of decisions keyword
                    }));
            
            // Add more Rhema-specific completions for YAML
            addRhemaYamlCompletions(result);
        }
        
        /**
         * Add Rhema-specific completions for YAML files.
         */
        private void addRhemaYamlCompletions(@NotNull CompletionResultSet result) {
            // TODO: Add more comprehensive Rhema completions for YAML files
            // This would include Rhema schema elements that can be used in regular YAML
        }
    }
} 