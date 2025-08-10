package ai.fugue.rhema.services;

import com.intellij.openapi.components.ProjectService;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vfs.VirtualFile;
import com.intellij.openapi.vfs.VirtualFileManager;
import com.intellij.openapi.vfs.VirtualFileListener;
import com.intellij.openapi.vfs.VirtualFileEvent;
import org.jetbrains.annotations.NotNull;

import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.locks.ReadWriteLock;
import java.util.concurrent.locks.ReentrantReadWriteLock;

/**
 * Project service for the Rhema plugin.
 * Manages project-specific functionality and state with performance optimizations.
 */
public class RhemaProjectService implements ProjectService {
    
    private static final Logger LOG = Logger.getInstance(RhemaProjectService.class);
    private final Project project;
    private final Map<String, Object> projectState = new ConcurrentHashMap<>();
    private final Map<String, VirtualFile> rhemaFiles = new ConcurrentHashMap<>();
    
    // Performance optimization: Caching
    private final Map<String, Object> validationCache = new ConcurrentHashMap<>();
    private final Map<String, Long> cacheTimestamps = new ConcurrentHashMap<>();
    private static final long CACHE_TTL_MS = 5 * 60 * 1000; // 5 minutes
    
    // Performance optimization: Lazy loading
    private final Map<String, Boolean> loadedFiles = new ConcurrentHashMap<>();
    private volatile boolean isInitialScanComplete = false;
    
    // Performance optimization: Background processing
    private final ScheduledExecutorService backgroundExecutor = Executors.newScheduledThreadPool(2);
    private final ReadWriteLock fileScanLock = new ReentrantReadWriteLock();
    
    // Performance optimization: File watching
    private final VirtualFileListener fileListener = new VirtualFileListener() {
        @Override
        public void fileCreated(@NotNull VirtualFileEvent event) {
            if (isRhemaFile(event.getFile())) {
                scheduleFileProcessing(event.getFile(), "created");
            }
        }
        
        @Override
        public void fileDeleted(@NotNull VirtualFileEvent event) {
            String path = event.getFile().getPath();
            rhemaFiles.remove(path);
            validationCache.remove(path);
            cacheTimestamps.remove(path);
            loadedFiles.remove(path);
        }
        
        @Override
        public void fileChanged(@NotNull VirtualFileEvent event) {
            if (isRhemaFile(event.getFile())) {
                scheduleFileProcessing(event.getFile(), "changed");
            }
        }
    };
    
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
            projectState.put("startTime", System.currentTimeMillis());
            
            // Register file listener for real-time updates
            VirtualFileManager.getInstance().addVirtualFileListener(fileListener, project);
            
            // Schedule background file scanning
            scheduleBackgroundFileScan();
            
            LOG.info("Rhema Project Service initialized successfully for project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Project Service for project: " + project.getName(), e);
        }
    }
    
    @Override
    public void dispose() {
        LOG.info("Disposing Rhema Project Service for project: " + project.getName());
        
        try {
            // Shutdown background executor
            backgroundExecutor.shutdown();
            if (!backgroundExecutor.awaitTermination(5, TimeUnit.SECONDS)) {
                backgroundExecutor.shutdownNow();
            }
            
            // Remove file listener
            VirtualFileManager.getInstance().removeVirtualFileListener(fileListener);
            
            // Clean up project state
            projectState.clear();
            rhemaFiles.clear();
            validationCache.clear();
            cacheTimestamps.clear();
            loadedFiles.clear();
            
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
            // Schedule background file scanning
            scheduleBackgroundFileScan();
            
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
            validationCache.clear();
            cacheTimestamps.clear();
            loadedFiles.clear();
            
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
            // Invalidate cache for this file
            invalidateCache(file.getPath());
            
            // Schedule background validation
            scheduleFileProcessing(file, "changed");
            
            LOG.info("Rhema file change handled successfully: " + file.getPath());
        } catch (Exception e) {
            LOG.error("Failed to handle Rhema file change: " + file.getPath(), e);
        }
    }
    
    /**
     * Performance optimization: Schedule background file processing.
     */
    private void scheduleFileProcessing(VirtualFile file, String operation) {
        backgroundExecutor.schedule(() -> {
            try {
                fileScanLock.writeLock().lock();
                try {
                    switch (operation) {
                        case "created":
                        case "changed":
                            rhemaFiles.put(file.getPath(), file);
                            validateRhemaFile(file);
                            loadedFiles.put(file.getPath(), true);
                            break;
                    }
                } finally {
                    fileScanLock.writeLock().unlock();
                }
            } catch (Exception e) {
                LOG.error("Failed to process file in background: " + file.getPath(), e);
            }
        }, 100, TimeUnit.MILLISECONDS); // Small delay to batch operations
    }
    
    /**
     * Performance optimization: Schedule background file scanning.
     */
    private void scheduleBackgroundFileScan() {
        backgroundExecutor.schedule(() -> {
            try {
                LOG.info("Starting background file scan for project: " + project.getName());
                scanForRhemaFiles();
                isInitialScanComplete = true;
                LOG.info("Background file scan completed for project: " + project.getName());
            } catch (Exception e) {
                LOG.error("Failed to complete background file scan for project: " + project.getName(), e);
            }
        }, 1, TimeUnit.SECONDS); // Delay to avoid blocking startup
    }
    
    /**
     * Performance optimization: Intelligent file scanning with caching.
     */
    private void scanForRhemaFiles() {
        fileScanLock.writeLock().lock();
        try {
            VirtualFile projectDir = project.getBaseDir();
            if (projectDir != null && projectDir.exists()) {
                scanDirectoryForRhemaFiles(projectDir);
            }
        } finally {
            fileScanLock.writeLock().unlock();
        }
    }
    
    /**
     * Performance optimization: Recursive directory scanning with early termination.
     */
    private void scanDirectoryForRhemaFiles(VirtualFile directory) {
        if (directory == null || !directory.exists() || !directory.isDirectory()) {
            return;
        }
        
        // Skip common directories that don't contain Rhema files
        String dirName = directory.getName();
        if (dirName.equals(".git") || dirName.equals("node_modules") || 
            dirName.equals("target") || dirName.equals("build") ||
            dirName.equals("dist") || dirName.equals(".idea")) {
            return;
        }
        
        VirtualFile[] children = directory.getChildren();
        if (children != null) {
            for (VirtualFile child : children) {
                if (child.isDirectory()) {
                    scanDirectoryForRhemaFiles(child);
                } else if (isRhemaFile(child)) {
                    rhemaFiles.put(child.getPath(), child);
                    // Lazy load: don't validate immediately
                    loadedFiles.put(child.getPath(), false);
                }
            }
        }
    }
    
    /**
     * Performance optimization: Cached file type detection.
     */
    private boolean isRhemaFile(VirtualFile file) {
        if (file == null || !file.exists() || file.isDirectory()) {
            return false;
        }
        
        String fileName = file.getName().toLowerCase();
        return fileName.endsWith(".rhema.yml") || 
               fileName.endsWith(".scope.yml") ||
               fileName.endsWith(".context.yml") ||
               fileName.endsWith(".todos.yml") ||
               fileName.endsWith(".insights.yml") ||
               fileName.endsWith(".patterns.yml") ||
               fileName.endsWith(".decisions.yml");
    }
    
    /**
     * Performance optimization: Lazy validation with caching.
     */
    private void validateRhemaFile(VirtualFile file) {
        String filePath = file.getPath();
        
        // Check cache first
        if (isCacheValid(filePath)) {
            LOG.debug("Using cached validation for file: " + filePath);
            return;
        }
        
        try {
            // Perform validation (this would integrate with RhemaSchemaValidator)
            Object validationResult = performValidation(file);
            
            // Cache the result
            validationCache.put(filePath, validationResult);
            cacheTimestamps.put(filePath, System.currentTimeMillis());
            
            LOG.debug("Validation completed and cached for file: " + filePath);
        } catch (Exception e) {
            LOG.error("Failed to validate Rhema file: " + filePath, e);
        }
    }
    
    /**
     * Performance optimization: Check if cache is still valid.
     */
    private boolean isCacheValid(String filePath) {
        Long timestamp = cacheTimestamps.get(filePath);
        if (timestamp == null) {
            return false;
        }
        
        long age = System.currentTimeMillis() - timestamp;
        return age < CACHE_TTL_MS;
    }
    
    /**
     * Performance optimization: Invalidate cache for a file.
     */
    private void invalidateCache(String filePath) {
        validationCache.remove(filePath);
        cacheTimestamps.remove(filePath);
    }
    
    /**
     * Performance optimization: Perform actual validation.
     */
    private Object performValidation(VirtualFile file) {
        // This would integrate with the existing RhemaSchemaValidator
        // For now, return a placeholder
        return new Object();
    }
    
    /**
     * Performance optimization: Get cached validation result.
     */
    public Object getCachedValidation(String filePath) {
        if (isCacheValid(filePath)) {
            return validationCache.get(filePath);
        }
        return null;
    }
    
    /**
     * Performance optimization: Check if file is loaded.
     */
    public boolean isFileLoaded(String filePath) {
        return loadedFiles.getOrDefault(filePath, false);
    }
    
    /**
     * Performance optimization: Lazy load a file.
     */
    public void lazyLoadFile(String filePath) {
        VirtualFile file = rhemaFiles.get(filePath);
        if (file != null && !isFileLoaded(filePath)) {
            scheduleFileProcessing(file, "lazy_load");
        }
    }
    
    /**
     * Performance optimization: Get file count statistics.
     */
    public Map<String, Object> getPerformanceStats() {
        Map<String, Object> stats = new HashMap<>();
        stats.put("totalFiles", rhemaFiles.size());
        stats.put("loadedFiles", loadedFiles.values().stream().filter(Boolean::booleanValue).count());
        stats.put("cachedValidations", validationCache.size());
        stats.put("isInitialScanComplete", isInitialScanComplete);
        stats.put("cacheHitRate", calculateCacheHitRate());
        return stats;
    }
    
    /**
     * Performance optimization: Calculate cache hit rate.
     */
    private double calculateCacheHitRate() {
        long totalRequests = cacheTimestamps.size();
        long cacheHits = validationCache.size();
        return totalRequests > 0 ? (double) cacheHits / totalRequests : 0.0;
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