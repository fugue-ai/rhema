package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.ui.DialogWrapper;
import com.intellij.openapi.ui.ValidationInfo;
import com.intellij.ui.components.JBTextField;
import com.intellij.ui.components.JBLabel;
import com.intellij.ui.components.JBScrollPane;
import com.intellij.ui.table.JBTable;
import com.intellij.util.ui.FormBuilder;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.swing.*;
import javax.swing.table.DefaultTableModel;
import java.awt.*;
import java.util.List;
import java.util.ArrayList;

/**
 * Action to search Rhema context.
 * Provides a dialog for searching through Rhema context with various filters.
 */
public class RhemaSearchContextAction extends AnAction {

    public RhemaSearchContextAction() {
        super("Search Context", "Search Rhema context", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        RhemaSearchContextDialog dialog = new RhemaSearchContextDialog(project);
        if (dialog.showAndGet()) {
            // Handle search results
            String searchTerm = dialog.getSearchTerm();
            String searchType = dialog.getSearchType();
            List<SearchResult> results = performSearch(project, searchTerm, searchType);
            showSearchResults(project, results);
        }
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    private List<SearchResult> performSearch(Project project, String searchTerm, String searchType) {
        // TODO: Implement actual search logic using Rhema services
        List<SearchResult> results = new ArrayList<>();
        
        // Mock search results for demonstration
        results.add(new SearchResult("scope.yml", "scope", "Found in scope definition", 10));
        results.add(new SearchResult("context.yml", "context", "Found in context section", 25));
        results.add(new SearchResult("todos.yml", "todos", "Found in todos section", 15));
        
        return results;
    }

    private void showSearchResults(Project project, List<SearchResult> results) {
        RhemaSearchResultsDialog resultsDialog = new RhemaSearchResultsDialog(project, results);
        resultsDialog.show();
    }

    /**
     * Dialog for entering search parameters.
     */
    private static class RhemaSearchContextDialog extends DialogWrapper {
        private final JBTextField searchField;
        private final JComboBox<String> searchTypeCombo;

        public RhemaSearchContextDialog(@NotNull Project project) {
            super(project);
            setTitle("Search Rhema Context");
            
            searchField = new JBTextField(30);
            searchTypeCombo = new JComboBox<>(new String[]{"All", "Scopes", "Context", "Todos", "Insights", "Patterns", "Decisions"});
            
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            JPanel dialogPanel = FormBuilder.createFormBuilder()
                    .addLabeledComponent(new JBLabel("Search Term:"), searchField)
                    .addLabeledComponent(new JBLabel("Search Type:"), searchTypeCombo)
                    .addComponentFillVertically(new JPanel(), 0)
                    .getPanel();
            dialogPanel.setPreferredSize(new Dimension(400, 100));
            return dialogPanel;
        }

        public String getSearchTerm() {
            return searchField.getText();
        }

        public String getSearchType() {
            return (String) searchTypeCombo.getSelectedItem();
        }

        @Override
        protected ValidationInfo doValidate() {
            if (searchField.getText().trim().isEmpty()) {
                return new ValidationInfo("Search term cannot be empty", searchField);
            }
            return null;
        }
    }

    /**
     * Dialog for displaying search results.
     */
    private static class RhemaSearchResultsDialog extends DialogWrapper {
        private final List<SearchResult> results;

        public RhemaSearchResultsDialog(@NotNull Project project, List<SearchResult> results) {
            super(project);
            this.results = results;
            setTitle("Search Results");
            setSize(600, 400);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            String[] columnNames = {"File", "Type", "Description", "Line"};
            DefaultTableModel model = new DefaultTableModel(columnNames, 0);
            
            for (SearchResult result : results) {
                model.addRow(new Object[]{result.file, result.type, result.description, result.line});
            }
            
            JBTable table = new JBTable(model);
            table.setAutoResizeMode(JTable.AUTO_RESIZE_OFF);
            table.getColumnModel().getColumn(0).setPreferredWidth(150);
            table.getColumnModel().getColumn(1).setPreferredWidth(100);
            table.getColumnModel().getColumn(2).setPreferredWidth(250);
            table.getColumnModel().getColumn(3).setPreferredWidth(50);
            
            JBScrollPane scrollPane = new JBScrollPane(table);
            scrollPane.setPreferredSize(new Dimension(550, 300));
            
            return scrollPane;
        }
    }

    /**
     * Represents a search result.
     */
    private static class SearchResult {
        final String file;
        final String type;
        final String description;
        final int line;

        SearchResult(String file, String type, String description, int line) {
            this.file = file;
            this.type = type;
            this.description = description;
            this.line = line;
        }
    }
} 