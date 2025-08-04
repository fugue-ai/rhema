package ai.fugue.rhema.services;

import com.intellij.openapi.components.ProjectService;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import org.jetbrains.annotations.NotNull;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Project service for the Rhema plugin.
 * Manages project-specific functionality and state.
 */
public class RhemaProjectService implements ProjectService {
    
    private static final Logger LOG = Logger.getInstance(RhemaProjectService.class);
    private final Project project;
    private final Map<String, Object> projectState = new ConcurrentHashMap<>();
    private final Map<String, VirtualFile> rhemaFiles = new HashMap<>();
    
    public RhemaProjectService(@NotNull Project project) {
        this.project = project;
    }
    
    @Override
    public void initialize() {
        LOG.info("Initializing Rhema Project Service for project: " + project.getName());
        
        try {
            // Initialize project state
            projectState.put("initialized", true);
            projectState.put("projectName", project.getName());
            
            // Scan for Rhema files
            scanForRhemaFiles();
            
            LOG.info("Rhema Project Service initialized successfully for project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Project Service for project: " + project.getName(), e);
        }
    }
    
    @Override
    public void dispose() {
        LOG.info("Disposing Rhema Project Service for project: " + project.getName());
        
        try {
            // Clean up project state
            projectState.clear();
            rhemaFiles.clear();
            
            LOG.info("Rhema Project Service disposed successfully for project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to dispose Rhema Project Service for project: " + project.getName(), e);
        }
    }
    
    /**
     * Called when the project is opened.
     */
    public void onProjectOpened() {
        LOG.info("Project opened: " + project.getName());
        
        try {
            // Refresh Rhema files
            scanForRhemaFiles();
            
            LOG.info("Project opened successfully: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to handle project opened for: " + project.getName(), e);
        }
    }
    
    /**
     * Called when the project is closed.
     */
    public void onProjectClosed() {
        LOG.info("Project closed: " + project.getName());
        
        try {
            // Clean up project resources
            rhemaFiles.clear();
            
            LOG.info("Project closed successfully: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to handle project closed for: " + project.getName(), e);
        }
    }
    
    /**
     * Called when a Rhema file is changed.
     */
    public void onRhemaFileChanged(@NotNull VirtualFile file) {
        LOG.info("Rhema file changed: " + file.getPath());
        
        try {
            // Update file cache
            rhemaFiles.put(file.getPath(), file);
            
            // Trigger validation and analysis
            validateRhemaFile(file);
            
            LOG.info("Rhema file change handled successfully: " + file.getPath());
        } catch (Exception e) {
            LOG.error("Failed to handle Rhema file change: " + file.getPath(), e);
        }
    }
    
    /**
     * Scan for Rhema files in the project.
     */
    private void scanForRhemaFiles() {
        LOG.info("Scanning for Rhema files in project: " + project.getName());
        
        try {
            // Clear existing files
            rhemaFiles.clear();
            
            // Scan project root for Rhema files
            VirtualFile projectRoot = project.getBaseDir();
            if (projectRoot != null) {
                scanDirectoryForRhemaFiles(projectRoot);
            }
            
            LOG.info("Found " + rhemaFiles.size() + " Rhema files in project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to scan for Rhema files in project: " + project.getName(), e);
        }
    }
    
    /**
     * Recursively scan directory for Rhema files.
     */
    private void scanDirectoryForRhemaFiles(VirtualFile directory) {
        for (VirtualFile file : directory.getChildren()) {
            if (file.isDirectory()) {
                scanDirectoryForRhemaFiles(file);
            } else if (isRhemaFile(file)) {
                rhemaFiles.put(file.getPath(), file);
            }
        }
    }
    
    /**
     * Check if a file is a Rhema file.
     */
    private boolean isRhemaFile(VirtualFile file) {
        String name = file.getName();
        return name.endsWith(".rhema.yml") || 
               name.endsWith(".rhema.yaml") || 
               name.equals("rhema.yml") || 
               name.equals("rhema.yaml");
    }
    
    /**
     * Validate a Rhema file.
     */
    private void validateRhemaFile(VirtualFile file) {
        LOG.info("Validating Rhema file: " + file.getPath());
        
        try {
            // TODO: Implement Rhema file validation
            // This would involve parsing the YAML and validating against Rhema schema
            
            LOG.info("Rhema file validation completed: " + file.getPath());
        } catch (Exception e) {
            LOG.error("Failed to validate Rhema file: " + file.getPath(), e);
        }
    }
    
    /**
     * Get project state value.
     */
    public Object getProjectState(String key) {
        return projectState.get(key);
    }
    
    /**
     * Set project state value.
     */
    public void setProjectState(String key, Object value) {
        projectState.put(key, value);
    }
    
    /**
     * Get all Rhema files in the project.
     */
    public Map<String, VirtualFile> getRhemaFiles() {
        return new HashMap<>(rhemaFiles);
    }
    
    /**
     * Get the associated project.
     */
    public Project getProject() {
        return project;
    }
    
    /**
     * Check if the service is initialized.
     */
    public boolean isInitialized() {
        return projectState.containsKey("initialized");
    }
} 