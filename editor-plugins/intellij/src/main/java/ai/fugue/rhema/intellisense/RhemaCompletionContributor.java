package ai.fugue.rhema.intellisense;

import com.intellij.codeInsight.completion.*;
import com.intellij.codeInsight.lookup.LookupElementBuilder;
import com.intellij.patterns.PlatformPatterns;
import com.intellij.util.ProcessingContext;
import org.jetbrains.annotations.NotNull;

/**
 * Completion contributor for Rhema YAML files.
 * Provides IntelliSense functionality for Rhema-specific completions.
 */
public class RhemaCompletionContributor extends CompletionContributor {
    
    public RhemaCompletionContributor() {
        // Register completion providers for Rhema YAML files
        extend(CompletionType.BASIC, 
               PlatformPatterns.psiElement().withLanguage(RhemaLanguageSupport.INSTANCE),
               new RhemaCompletionProvider());
    }
    
    /**
     * Completion provider for Rhema YAML files.
     */
    private static class RhemaCompletionProvider extends CompletionProvider<CompletionParameters> {
        
        @Override
        protected void addCompletions(@NotNull CompletionParameters parameters,
                                    @NotNull ProcessingContext context,
                                    @NotNull CompletionResultSet result) {
            
            // Add Rhema-specific keywords and values
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
            
            // Add more Rhema-specific completions
            addRhemaCompletions(result);
        }
        
        /**
         * Add Rhema-specific completions.
         */
        private void addRhemaCompletions(@NotNull CompletionResultSet result) {
            // Add Rhema document types
            addDocumentTypeCompletions(result);
            
            // Add Rhema field completions
            addFieldCompletions(result);
            
            // Add Rhema value completions
            addValueCompletions(result);
            
            // Add Rhema snippet completions
            addSnippetCompletions(result);
        }
        
        /**
         * Add document type completions.
         */
        private void addDocumentTypeCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("scope")
                    .withTypeText("Rhema scope document")
                    .withInsertHandler((context, item) -> {
                        // Insert scope template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  version: 1.0.0\n  tags: []\n  dependencies: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("context")
                    .withTypeText("Rhema context document")
                    .withInsertHandler((context, item) -> {
                        // Insert context template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  version: 1.0.0\n  tags: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("todos")
                    .withTypeText("Rhema todos document")
                    .withInsertHandler((context, item) -> {
                        // Insert todos template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  items: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("insights")
                    .withTypeText("Rhema insights document")
                    .withInsertHandler((context, item) -> {
                        // Insert insights template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  insights: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("patterns")
                    .withTypeText("Rhema patterns document")
                    .withInsertHandler((context, item) -> {
                        // Insert patterns template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  patterns: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("decisions")
                    .withTypeText("Rhema decisions document")
                    .withInsertHandler((context, item) -> {
                        // Insert decisions template
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  name: \n  description: \n  scope: \n  decisions: []");
                    }));
        }
        
        /**
         * Add field completions.
         */
        private void addFieldCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("name")
                    .withTypeText("Document name")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("description")
                    .withTypeText("Document description")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
            
            result.addElement(LookupElementBuilder.create("version")
                    .withTypeText("Document version")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": 1.0.0");
                    }));
            
            result.addElement(LookupElementBuilder.create("tags")
                    .withTypeText("Document tags")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("dependencies")
                    .withTypeText("Document dependencies")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": []");
                    }));
            
            result.addElement(LookupElementBuilder.create("scope")
                    .withTypeText("Parent scope")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), ": ");
                    }));
        }
        
        /**
         * Add value completions.
         */
        private void addValueCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("high")
                    .withTypeText("Priority: High")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("medium")
                    .withTypeText("Priority: Medium")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("low")
                    .withTypeText("Priority: Low")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("true")
                    .withTypeText("Boolean: True")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
            
            result.addElement(LookupElementBuilder.create("false")
                    .withTypeText("Boolean: False")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), "");
                    }));
        }
        
        /**
         * Add snippet completions.
         */
        private void addSnippetCompletions(@NotNull CompletionResultSet result) {
            result.addElement(LookupElementBuilder.create("rhema-scope")
                    .withTypeText("Complete scope template")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\nscope:\n  name: \n  description: \n  version: 1.0.0\n  tags: []\n  dependencies: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("rhema-context")
                    .withTypeText("Complete context template")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\ncontext:\n  name: \n  description: \n  scope: \n  version: 1.0.0\n  tags: []");
                    }));
            
            result.addElement(LookupElementBuilder.create("rhema-todo")
                    .withTypeText("Complete todo item")
                    .withInsertHandler((context, item) -> {
                        context.getDocument().insertString(context.getTailOffset(), 
                            "\n  - title: \n    description: \n    priority: medium\n    status: pending");
                    }));
        }
    }
} 