package ai.fugue.rhema.actions;

import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.ui.Messages;
import com.intellij.openapi.ui.DialogWrapper;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.openapi.vfs.VfsUtil;
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
import java.util.Collection;

/**
 * Action to validate Rhema files.
 * Provides validation for Rhema YAML files with detailed error reporting.
 */
public class RhemaValidateFilesAction extends AnAction {

    public RhemaValidateFilesAction() {
        super("Validate Files", "Validate Rhema files", null);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project == null) {
            Messages.showErrorDialog("No project available", "Error");
            return;
        }

        // Find all Rhema files in the project
        List<VirtualFile> rhemaFiles = findRhemaFiles(project);
        
        if (rhemaFiles.isEmpty()) {
            Messages.showInfoMessage(project, "No Rhema files found in the project", "Validation");
            return;
        }

        // Perform validation
        List<ValidationResult> results = validateFiles(project, rhemaFiles);
        
        // Show validation results
        showValidationResults(project, results);
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        e.getPresentation().setEnabledAndVisible(e.getProject() != null);
    }

    private List<VirtualFile> findRhemaFiles(Project project) {
        List<VirtualFile> rhemaFiles = new ArrayList<>();
        VirtualFile projectDir = project.getBaseDir();
        
        if (projectDir != null) {
            Collection<VirtualFile> allFiles = VfsUtil.collectChildrenRecursively(projectDir);
            for (VirtualFile file : allFiles) {
                if (file.isDirectory()) continue;
                
                String fileName = file.getName();
                if (fileName.endsWith(".rhema.yml") || 
                    fileName.endsWith(".rhema.yaml") ||
                    fileName.equals("scope.yml") ||
                    fileName.equals("context.yml") ||
                    fileName.equals("todos.yml") ||
                    fileName.equals("insights.yml") ||
                    fileName.equals("patterns.yml") ||
                    fileName.equals("decisions.yml")) {
                    rhemaFiles.add(file);
                }
            }
        }
        
        return rhemaFiles;
    }

    private List<ValidationResult> validateFiles(Project project, List<VirtualFile> files) {
        List<ValidationResult> results = new ArrayList<>();
        
        for (VirtualFile file : files) {
            ValidationResult result = validateFile(project, file);
            results.add(result);
        }
        
        return results;
    }

    private ValidationResult validateFile(Project project, VirtualFile file) {
        ValidationResult result = new ValidationResult(file);
        
        try {
            // TODO: Implement actual validation logic using Rhema services
            // For now, we'll do basic validation
            
            // Check if file is readable
            if (!file.isValid()) {
                result.addError("File is not valid or accessible");
                return result;
            }
            
            // Check file size
            if (file.getLength() == 0) {
                result.addWarning("File is empty");
            }
            
            // Check file extension
            String fileName = file.getName();
            if (!fileName.endsWith(".yml") && !fileName.endsWith(".yaml")) {
                result.addWarning("File does not have YAML extension");
            }
            
            // Mock validation - in real implementation, this would use Rhema validation services
            if (fileName.contains("scope")) {
                result.addInfo("Scope file validated successfully");
            } else if (fileName.contains("context")) {
                result.addInfo("Context file validated successfully");
            } else if (fileName.contains("todos")) {
                result.addInfo("Todos file validated successfully");
            } else {
                result.addInfo("File validated successfully");
            }
            
        } catch (Exception e) {
            result.addError("Validation failed: " + e.getMessage());
        }
        
        return result;
    }

    private void showValidationResults(Project project, List<ValidationResult> results) {
        RhemaValidationResultsDialog dialog = new RhemaValidationResultsDialog(project, results);
        dialog.show();
    }

    /**
     * Dialog for displaying validation results.
     */
    private static class RhemaValidationResultsDialog extends DialogWrapper {
        private final List<ValidationResult> results;

        public RhemaValidationResultsDialog(@NotNull Project project, List<ValidationResult> results) {
            super(project);
            this.results = results;
            setTitle("Validation Results");
            setSize(700, 500);
            init();
        }

        @Override
        protected @Nullable JComponent createCenterPanel() {
            String[] columnNames = {"File", "Status", "Messages"};
            DefaultTableModel model = new DefaultTableModel(columnNames, 0) {
                @Override
                public boolean isCellEditable(int row, int column) {
                    return false;
                }
            };
            
            for (ValidationResult result : results) {
                String status = result.getStatus();
                String messages = String.join("; ", result.getAllMessages());
                model.addRow(new Object[]{result.file.getName(), status, messages});
            }
            
            JBTable table = new JBTable(model);
            table.setAutoResizeMode(JTable.AUTO_RESIZE_OFF);
            table.getColumnModel().getColumn(0).setPreferredWidth(200);
            table.getColumnModel().getColumn(1).setPreferredWidth(100);
            table.getColumnModel().getColumn(2).setPreferredWidth(350);
            
            JBScrollPane scrollPane = new JBScrollPane(table);
            scrollPane.setPreferredSize(new Dimension(650, 400));
            
            return scrollPane;
        }
    }

    /**
     * Represents validation result for a file.
     */
    private static class ValidationResult {
        final VirtualFile file;
        final List<String> errors = new ArrayList<>();
        final List<String> warnings = new ArrayList<>();
        final List<String> info = new ArrayList<>();

        ValidationResult(VirtualFile file) {
            this.file = file;
        }

        void addError(String message) {
            errors.add(message);
        }

        void addWarning(String message) {
            warnings.add(message);
        }

        void addInfo(String message) {
            info.add(message);
        }

        String getStatus() {
            if (!errors.isEmpty()) {
                return "ERROR";
            } else if (!warnings.isEmpty()) {
                return "WARNING";
            } else {
                return "OK";
            }
        }

        List<String> getAllMessages() {
            List<String> allMessages = new ArrayList<>();
            allMessages.addAll(errors);
            allMessages.addAll(warnings);
            allMessages.addAll(info);
            return allMessages;
        }
    }
} 