package ai.fugue.rhema.validation;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.psi.PsiElement;
import com.intellij.psi.PsiFile;
import org.jetbrains.annotations.NotNull;

import java.io.IOException;
import java.io.InputStream;
import java.util.*;

/**
 * Rhema schema validator that validates YAML files against Rhema schemas.
 */
public class RhemaSchemaValidator {
    
    private static final Logger LOG = Logger.getInstance(RhemaSchemaValidator.class);
    private static final ObjectMapper YAML_MAPPER = new ObjectMapper(new YAMLFactory());
    private static final ObjectMapper JSON_MAPPER = new ObjectMapper();
    
    // Rhema document types
    public enum DocumentType {
        SCOPE("scope"),
        CONTEXT("context"),
        TODOS("todos"),
        INSIGHTS("insights"),
        PATTERNS("patterns"),
        DECISIONS("decisions"),
        ACTION("action"),
        KNOWLEDGE("knowledge"),
        LOCK("lock"),
        CONVENTIONS("conventions");
        
        private final String keyword;
        
        DocumentType(String keyword) {
            this.keyword = keyword;
        }
        
        public String getKeyword() {
            return keyword;
        }
        
        public static DocumentType fromKeyword(String keyword) {
            for (DocumentType type : values()) {
                if (type.keyword.equals(keyword)) {
                    return type;
                }
            }
            return null;
        }
    }
    
    /**
     * Validation result containing errors and warnings.
     */
    public static class ValidationResult {
        private final List<ValidationError> errors = new ArrayList<>();
        private final List<ValidationWarning> warnings = new ArrayList<>();
        
        public void addError(String message, int line, int column) {
            errors.add(new ValidationError(message, line, column));
        }
        
        public void addWarning(String message, int line, int column) {
            warnings.add(new ValidationWarning(message, line, column));
        }
        
        public List<ValidationError> getErrors() {
            return errors;
        }
        
        public List<ValidationWarning> getWarnings() {
            return warnings;
        }
        
        public boolean hasErrors() {
            return !errors.isEmpty();
        }
        
        public boolean hasWarnings() {
            return !warnings.isEmpty();
        }
    }
    
    public static class ValidationError {
        private final String message;
        private final int line;
        private final int column;
        
        public ValidationError(String message, int line, int column) {
            this.message = message;
            this.line = line;
            this.column = column;
        }
        
        public String getMessage() { return message; }
        public int getLine() { return line; }
        public int getColumn() { return column; }
    }
    
    public static class ValidationWarning {
        private final String message;
        private final int line;
        private final int column;
        
        public ValidationWarning(String message, int line, int column) {
            this.message = message;
            this.line = line;
            this.column = column;
        }
        
        public String getMessage() { return message; }
        public int getLine() { return line; }
        public int getColumn() { return column; }
    }
    
    /**
     * Validate a Rhema YAML file.
     */
    public ValidationResult validateFile(@NotNull PsiFile file) {
        ValidationResult result = new ValidationResult();
        
        try {
            String content = file.getText();
            DocumentType documentType = detectDocumentType(content);
            
            if (documentType == null) {
                result.addError("Could not detect Rhema document type", 1, 1);
                return result;
            }
            
            // Parse YAML content
            JsonNode yamlNode = YAML_MAPPER.readTree(content);
            
            // Validate against schema
            validateDocument(yamlNode, documentType, result);
            
        } catch (IOException e) {
            result.addError("Failed to parse YAML: " + e.getMessage(), 1, 1);
        } catch (Exception e) {
            LOG.error("Validation error", e);
            result.addError("Validation error: " + e.getMessage(), 1, 1);
        }
        
        return result;
    }
    
    /**
     * Detect the document type from content.
     */
    public DocumentType detectDocumentType(String content) {
        String[] lines = content.split("\n");
        for (String line : lines) {
            line = line.trim();
            if (line.startsWith("rhema:")) {
                // Check for rhema wrapper
                continue;
            }
            for (DocumentType type : DocumentType.values()) {
                if (line.startsWith(type.getKeyword() + ":")) {
                    return type;
                }
            }
        }
        return null;
    }
    
    /**
     * Validate document against its schema.
     */
    private void validateDocument(JsonNode node, DocumentType type, ValidationResult result) {
        switch (type) {
            case SCOPE:
                validateScope(node, result);
                break;
            case TODOS:
                validateTodos(node, result);
                break;
            case CONTEXT:
                validateContext(node, result);
                break;
            case INSIGHTS:
                validateInsights(node, result);
                break;
            case PATTERNS:
                validatePatterns(node, result);
                break;
            case DECISIONS:
                validateDecisions(node, result);
                break;
            case ACTION:
                validateAction(node, result);
                break;
            case KNOWLEDGE:
                validateKnowledge(node, result);
                break;
            case LOCK:
                validateLock(node, result);
                break;
            case CONVENTIONS:
                validateConventions(node, result);
                break;
        }
    }
    
    /**
     * Validate scope document.
     */
    private void validateScope(JsonNode node, ValidationResult result) {
        // Check for rhema wrapper
        if (node.has("rhema")) {
            JsonNode rhemaNode = node.get("rhema");
            validateRhemaWrapper(rhemaNode, result);
            
            if (rhemaNode.has("scope")) {
                JsonNode scopeNode = rhemaNode.get("scope");
                validateScopeContent(scopeNode, result);
            } else {
                result.addError("Scope document must contain 'scope' section", 1, 1);
            }
        } else {
            result.addError("Scope document must be wrapped in 'rhema' section", 1, 1);
        }
    }
    
    /**
     * Validate rhema wrapper.
     */
    private void validateRhemaWrapper(JsonNode rhemaNode, ValidationResult result) {
        if (!rhemaNode.has("version")) {
            result.addError("Rhema section must contain 'version' field", 1, 1);
        } else {
            String version = rhemaNode.get("version").asText();
            if (!version.matches("^\\d+\\.\\d+\\.\\d+$")) {
                result.addWarning("Version should follow semantic versioning format (e.g., 1.0.0)", 1, 1);
            }
        }
    }
    
    /**
     * Validate scope content.
     */
    private void validateScopeContent(JsonNode scopeNode, ValidationResult result) {
        // Required fields
        if (!scopeNode.has("type")) {
            result.addError("Scope must contain 'type' field", 1, 1);
        } else {
            String type = scopeNode.get("type").asText();
            String[] validTypes = {"repository", "service", "application", "library", "component"};
            if (!Arrays.asList(validTypes).contains(type)) {
                result.addError("Scope type must be one of: " + Arrays.toString(validTypes), 1, 1);
            }
        }
        
        if (!scopeNode.has("name")) {
            result.addError("Scope must contain 'name' field", 1, 1);
        } else {
            String name = scopeNode.get("name").asText();
            if (name.trim().isEmpty()) {
                result.addError("Scope name cannot be empty", 1, 1);
            }
        }
        
        if (!scopeNode.has("description")) {
            result.addError("Scope must contain 'description' field", 1, 1);
        } else {
            String description = scopeNode.get("description").asText();
            if (description.trim().isEmpty()) {
                result.addWarning("Scope description should not be empty", 1, 1);
            }
        }
        
        // Optional fields validation
        if (scopeNode.has("boundaries")) {
            validateBoundaries(scopeNode.get("boundaries"), result);
        }
        
        if (scopeNode.has("dependencies")) {
            validateDependencies(scopeNode.get("dependencies"), result);
        }
        
        if (scopeNode.has("responsibilities")) {
            validateResponsibilities(scopeNode.get("responsibilities"), result);
        }
        
        if (scopeNode.has("tech")) {
            validateTech(scopeNode.get("tech"), result);
        }
    }
    
    /**
     * Validate boundaries section.
     */
    private void validateBoundaries(JsonNode boundariesNode, ValidationResult result) {
        if (boundariesNode.has("includes")) {
            JsonNode includesNode = boundariesNode.get("includes");
            if (!includesNode.isArray()) {
                result.addError("Boundaries 'includes' must be an array", 1, 1);
            }
        }
        
        if (boundariesNode.has("excludes")) {
            JsonNode excludesNode = boundariesNode.get("excludes");
            if (!excludesNode.isArray()) {
                result.addError("Boundaries 'excludes' must be an array", 1, 1);
            }
        }
    }
    
    /**
     * Validate dependencies section.
     */
    private void validateDependencies(JsonNode dependenciesNode, ValidationResult result) {
        if (dependenciesNode.has("parent")) {
            String parent = dependenciesNode.get("parent").asText();
            if (parent.trim().isEmpty()) {
                result.addWarning("Parent dependency path should not be empty", 1, 1);
            }
        }
        
        if (dependenciesNode.has("children")) {
            JsonNode childrenNode = dependenciesNode.get("children");
            if (!childrenNode.isArray()) {
                result.addError("Dependencies 'children' must be an array", 1, 1);
            }
        }
        
        if (dependenciesNode.has("peers")) {
            JsonNode peersNode = dependenciesNode.get("peers");
            if (!peersNode.isArray()) {
                result.addError("Dependencies 'peers' must be an array", 1, 1);
            }
        }
    }
    
    /**
     * Validate responsibilities section.
     */
    private void validateResponsibilities(JsonNode responsibilitiesNode, ValidationResult result) {
        if (!responsibilitiesNode.isArray()) {
            result.addError("Responsibilities must be an array", 1, 1);
        } else {
            for (JsonNode responsibility : responsibilitiesNode) {
                if (!responsibility.isTextual() || responsibility.asText().trim().isEmpty()) {
                    result.addWarning("Responsibility should be a non-empty string", 1, 1);
                }
            }
        }
    }
    
    /**
     * Validate tech section.
     */
    private void validateTech(JsonNode techNode, ValidationResult result) {
        String[] techFields = {"primary_languages", "frameworks", "databases"};
        for (String field : techFields) {
            if (techNode.has(field)) {
                JsonNode fieldNode = techNode.get(field);
                if (!fieldNode.isArray()) {
                    result.addError("Tech '" + field + "' must be an array", 1, 1);
                }
            }
        }
    }
    
    /**
     * Validate todos document.
     */
    private void validateTodos(JsonNode node, ValidationResult result) {
        if (node.has("active")) {
            JsonNode activeNode = node.get("active");
            if (!activeNode.isObject()) {
                result.addError("Todos 'active' must be an object", 1, 1);
            } else {
                validateTodoItems(activeNode, result, "active");
            }
        }
        
        if (node.has("completed")) {
            JsonNode completedNode = node.get("completed");
            if (!completedNode.isObject()) {
                result.addError("Todos 'completed' must be an object", 1, 1);
            } else {
                validateCompletedItems(completedNode, result);
            }
        }
    }
    
    /**
     * Validate todo items.
     */
    private void validateTodoItems(JsonNode itemsNode, ValidationResult result, String section) {
        Iterator<Map.Entry<String, JsonNode>> fields = itemsNode.fields();
        while (fields.hasNext()) {
            Map.Entry<String, JsonNode> entry = fields.next();
            String itemName = entry.getKey();
            JsonNode itemNode = entry.getValue();
            
            if (!itemNode.isObject()) {
                result.addError("Todo item '" + itemName + "' must be an object", 1, 1);
                continue;
            }
            
            // Required fields
            if (!itemNode.has("title")) {
                result.addError("Todo item '" + itemName + "' must contain 'title' field", 1, 1);
            }
            
            if (!itemNode.has("priority")) {
                result.addError("Todo item '" + itemName + "' must contain 'priority' field", 1, 1);
            } else {
                String priority = itemNode.get("priority").asText();
                String[] validPriorities = {"low", "medium", "high", "critical"};
                if (!Arrays.asList(validPriorities).contains(priority)) {
                    result.addError("Todo priority must be one of: " + Arrays.toString(validPriorities), 1, 1);
                }
            }
            
            if (!itemNode.has("status")) {
                result.addError("Todo item '" + itemName + "' must contain 'status' field", 1, 1);
            } else {
                String status = itemNode.get("status").asText();
                String[] validStatuses = {"todo", "in_progress", "blocked", "review", "done"};
                if (!Arrays.asList(validStatuses).contains(status)) {
                    result.addError("Todo status must be one of: " + Arrays.toString(validStatuses), 1, 1);
                }
            }
            
            if (!itemNode.has("created")) {
                result.addError("Todo item '" + itemName + "' must contain 'created' field", 1, 1);
            }
        }
    }
    
    /**
     * Validate completed items.
     */
    private void validateCompletedItems(JsonNode itemsNode, ValidationResult result) {
        Iterator<Map.Entry<String, JsonNode>> fields = itemsNode.fields();
        while (fields.hasNext()) {
            Map.Entry<String, JsonNode> entry = fields.next();
            String itemName = entry.getKey();
            JsonNode itemNode = entry.getValue();
            
            if (!itemNode.isObject()) {
                result.addError("Completed item '" + itemName + "' must be an object", 1, 1);
                continue;
            }
            
            // Required fields
            if (!itemNode.has("title")) {
                result.addError("Completed item '" + itemName + "' must contain 'title' field", 1, 1);
            }
            
            if (!itemNode.has("completed")) {
                result.addError("Completed item '" + itemName + "' must contain 'completed' field", 1, 1);
            }
        }
    }
    
    // Placeholder validation methods for other document types
    private void validateContext(JsonNode node, ValidationResult result) {
        // TODO: Implement context validation
        result.addWarning("Context validation not yet implemented", 1, 1);
    }
    
    private void validateInsights(JsonNode node, ValidationResult result) {
        // TODO: Implement insights validation
        result.addWarning("Insights validation not yet implemented", 1, 1);
    }
    
    private void validatePatterns(JsonNode node, ValidationResult result) {
        // TODO: Implement patterns validation
        result.addWarning("Patterns validation not yet implemented", 1, 1);
    }
    
    private void validateDecisions(JsonNode node, ValidationResult result) {
        // TODO: Implement decisions validation
        result.addWarning("Decisions validation not yet implemented", 1, 1);
    }
    
    private void validateAction(JsonNode node, ValidationResult result) {
        // TODO: Implement action validation
        result.addWarning("Action validation not yet implemented", 1, 1);
    }
    
    private void validateKnowledge(JsonNode node, ValidationResult result) {
        // TODO: Implement knowledge validation
        result.addWarning("Knowledge validation not yet implemented", 1, 1);
    }
    
    private void validateLock(JsonNode node, ValidationResult result) {
        // TODO: Implement lock validation
        result.addWarning("Lock validation not yet implemented", 1, 1);
    }
    
    private void validateConventions(JsonNode node, ValidationResult result) {
        // TODO: Implement conventions validation
        result.addWarning("Conventions validation not yet implemented", 1, 1);
    }
} 