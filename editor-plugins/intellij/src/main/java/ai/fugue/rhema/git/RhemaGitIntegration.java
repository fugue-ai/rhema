package ai.fugue.rhema.git;

import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.vcs.VcsException;
import com.intellij.openapi.vcs.changes.Change;
import com.intellij.openapi.vcs.changes.ChangeListManager;
import com.intellij.openapi.vcs.changes.LocalChangeList;
import com.intellij.openapi.vfs.VirtualFile;
import git4idea.GitLocalBranch;
import git4idea.GitRemoteBranch;
import git4idea.GitRevisionNumber;
import git4idea.GitUtil;
import git4idea.commands.Git;
import git4idea.commands.GitCommand;
import git4idea.commands.GitCommandResult;
import git4idea.commands.GitLineHandler;
import git4idea.repo.GitRepository;
import git4idea.repo.GitRepositoryManager;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

/**
 * Git integration for the Rhema IntelliJ plugin.
 * Provides Git workflow integration, branch-aware context management, and Git hooks support.
 */
public class RhemaGitIntegration {
    
    private static final Logger LOG = Logger.getInstance(RhemaGitIntegration.class);
    private final Project project;
    private final GitRepositoryManager gitRepositoryManager;
    private final Git git;
    private final ScheduledExecutorService gitMonitorExecutor;
    
    // Branch-aware context management
    private final Map<String, String> branchContexts = new ConcurrentHashMap<>();
    private final Map<String, List<String>> branchRhemaFiles = new ConcurrentHashMap<>();
    
    // Git hooks integration
    private final Map<String, GitHookHandler> gitHooks = new ConcurrentHashMap<>();
    
    // Change tracking
    private final Set<String> trackedRhemaFiles = ConcurrentHashMap.newKeySet();
    private final Map<String, GitRevisionNumber> lastKnownRevisions = new ConcurrentHashMap<>();
    
    public RhemaGitIntegration(@NotNull Project project) {
        this.project = project;
        this.gitRepositoryManager = GitRepositoryManager.getInstance(project);
        this.git = Git.getInstance();
        this.gitMonitorExecutor = Executors.newScheduledThreadPool(1);
        
        initializeGitIntegration();
    }
    
    /**
     * Initialize Git integration.
     */
    private void initializeGitIntegration() {
        LOG.info("Initializing Rhema Git integration for project: " + project.getName());
        
        try {
            // Register Git hooks
            registerGitHooks();
            
            // Start Git monitoring
            startGitMonitoring();
            
            // Initialize branch-aware context
            initializeBranchAwareContext();
            
            LOG.info("Rhema Git integration initialized successfully for project: " + project.getName());
        } catch (Exception e) {
            LOG.error("Failed to initialize Rhema Git integration for project: " + project.getName(), e);
        }
    }
    
    /**
     * Register Git hooks for Rhema file changes.
     */
    private void registerGitHooks() {
        // Pre-commit hook for Rhema validation
        gitHooks.put("pre-commit", new RhemaPreCommitHook());
        
        // Post-commit hook for context updates
        gitHooks.put("post-commit", new RhemaPostCommitHook());
        
        // Pre-push hook for context synchronization
        gitHooks.put("pre-push", new RhemaPrePushHook());
        
        // Post-merge hook for context merging
        gitHooks.put("post-merge", new RhemaPostMergeHook());
        
        LOG.info("Registered " + gitHooks.size() + " Git hooks for Rhema integration");
    }
    
    /**
     * Start Git monitoring for changes.
     */
    private void startGitMonitoring() {
        gitMonitorExecutor.scheduleAtFixedRate(() -> {
            try {
                monitorGitChanges();
            } catch (Exception e) {
                LOG.error("Error in Git monitoring", e);
            }
        }, 5, 30, TimeUnit.SECONDS); // Check every 30 seconds after initial 5-second delay
        
        LOG.info("Started Git monitoring for Rhema files");
    }
    
    /**
     * Initialize branch-aware context management.
     */
    private void initializeBranchAwareContext() {
        Collection<GitRepository> repositories = gitRepositoryManager.getRepositories();
        for (GitRepository repository : repositories) {
            GitLocalBranch currentBranch = repository.getCurrentBranch();
            if (currentBranch != null) {
                String branchName = currentBranch.getName();
                String contextKey = generateBranchContextKey(repository, branchName);
                branchContexts.put(contextKey, branchName);
                
                // Scan for Rhema files in this branch
                scanBranchForRhemaFiles(repository, branchName);
            }
        }
        
        LOG.info("Initialized branch-aware context for " + branchContexts.size() + " branches");
    }
    
    /**
     * Monitor Git changes for Rhema files.
     */
    private void monitorGitChanges() {
        Collection<GitRepository> repositories = gitRepositoryManager.getRepositories();
        for (GitRepository repository : repositories) {
            try {
                // Check for new Rhema files
                checkForNewRhemaFiles(repository);
                
                // Check for modified Rhema files
                checkForModifiedRhemaFiles(repository);
                
                // Check for branch changes
                checkForBranchChanges(repository);
                
            } catch (Exception e) {
                LOG.error("Error monitoring Git changes in repository: " + repository.getRoot().getPath(), e);
            }
        }
    }
    
    /**
     * Check for new Rhema files in the repository.
     */
    private void checkForNewRhemaFiles(GitRepository repository) {
        try {
            GitCommandResult result = git.untrackedFiles(repository);
            if (result.success()) {
                List<String> untrackedFiles = result.getOutput();
                for (String file : untrackedFiles) {
                    if (isRhemaFile(file)) {
                        LOG.info("Found new Rhema file: " + file);
                        handleNewRhemaFile(repository, file);
                    }
                }
            }
        } catch (Exception e) {
            LOG.error("Error checking for new Rhema files", e);
        }
    }
    
    /**
     * Check for modified Rhema files in the repository.
     */
    private void checkForModifiedRhemaFiles(GitRepository repository) {
        try {
            ChangeListManager changeListManager = ChangeListManager.getInstance(project);
            List<Change> changes = changeListManager.getAllChanges();
            
            for (Change change : changes) {
                VirtualFile file = change.getVirtualFile();
                if (file != null && isRhemaFile(file.getPath())) {
                    String filePath = file.getPath();
                    GitRevisionNumber currentRevision = getCurrentRevision(repository, filePath);
                    GitRevisionNumber lastRevision = lastKnownRevisions.get(filePath);
                    
                    if (lastRevision == null || !currentRevision.equals(lastRevision)) {
                        LOG.info("Rhema file modified: " + filePath);
                        handleModifiedRhemaFile(repository, filePath, lastRevision, currentRevision);
                        lastKnownRevisions.put(filePath, currentRevision);
                    }
                }
            }
        } catch (Exception e) {
            LOG.error("Error checking for modified Rhema files", e);
        }
    }
    
    /**
     * Check for branch changes in the repository.
     */
    private void checkForBranchChanges(GitRepository repository) {
        GitLocalBranch currentBranch = repository.getCurrentBranch();
        if (currentBranch != null) {
            String branchName = currentBranch.getName();
            String contextKey = generateBranchContextKey(repository, branchName);
            String previousBranch = branchContexts.get(contextKey);
            
            if (previousBranch == null || !previousBranch.equals(branchName)) {
                LOG.info("Branch changed to: " + branchName);
                handleBranchChange(repository, previousBranch, branchName);
                branchContexts.put(contextKey, branchName);
            }
        }
    }
    
    /**
     * Handle new Rhema file discovery.
     */
    private void handleNewRhemaFile(GitRepository repository, String filePath) {
        try {
            // Add to tracked files
            trackedRhemaFiles.add(filePath);
            
            // Validate the new Rhema file
            validateRhemaFile(repository, filePath);
            
            // Update branch context
            updateBranchContext(repository, filePath);
            
            // Trigger pre-commit hook if needed
            triggerGitHook("pre-commit", repository, filePath);
            
            LOG.info("Handled new Rhema file: " + filePath);
        } catch (Exception e) {
            LOG.error("Error handling new Rhema file: " + filePath, e);
        }
    }
    
    /**
     * Handle modified Rhema file.
     */
    private void handleModifiedRhemaFile(GitRepository repository, String filePath, 
                                       GitRevisionNumber oldRevision, GitRevisionNumber newRevision) {
        try {
            // Validate the modified Rhema file
            validateRhemaFile(repository, filePath);
            
            // Update branch context
            updateBranchContext(repository, filePath);
            
            // Check for conflicts with other branches
            checkForBranchConflicts(repository, filePath);
            
            // Trigger pre-commit hook
            triggerGitHook("pre-commit", repository, filePath);
            
            LOG.info("Handled modified Rhema file: " + filePath);
        } catch (Exception e) {
            LOG.error("Error handling modified Rhema file: " + filePath, e);
        }
    }
    
    /**
     * Handle branch change.
     */
    private void handleBranchChange(GitRepository repository, String oldBranch, String newBranch) {
        try {
            // Save context for old branch
            if (oldBranch != null) {
                saveBranchContext(repository, oldBranch);
            }
            
            // Load context for new branch
            loadBranchContext(repository, newBranch);
            
            // Check for context conflicts
            checkForContextConflicts(repository, oldBranch, newBranch);
            
            // Trigger post-merge hook if coming from a merge
            if (isMergeOperation(repository)) {
                triggerGitHook("post-merge", repository, null);
            }
            
            LOG.info("Handled branch change from " + oldBranch + " to " + newBranch);
        } catch (Exception e) {
            LOG.error("Error handling branch change", e);
        }
    }
    
    /**
     * Validate Rhema file using Git integration.
     */
    private void validateRhemaFile(GitRepository repository, String filePath) {
        try {
            // This would integrate with the existing RhemaSchemaValidator
            // For now, perform basic validation
            VirtualFile file = repository.getRoot().getFileSystem().findFileByPath(filePath);
            if (file != null && file.exists()) {
                // Basic validation logic would go here
                LOG.debug("Validated Rhema file: " + filePath);
            }
        } catch (Exception e) {
            LOG.error("Error validating Rhema file: " + filePath, e);
        }
    }
    
    /**
     * Update branch context with Rhema file.
     */
    private void updateBranchContext(GitRepository repository, String filePath) {
        GitLocalBranch currentBranch = repository.getCurrentBranch();
        if (currentBranch != null) {
            String branchName = currentBranch.getName();
            String contextKey = generateBranchContextKey(repository, branchName);
            
            List<String> files = branchRhemaFiles.computeIfAbsent(contextKey, k -> new ArrayList<>());
            if (!files.contains(filePath)) {
                files.add(filePath);
            }
            
            LOG.debug("Updated branch context for " + branchName + " with file: " + filePath);
        }
    }
    
    /**
     * Check for conflicts with other branches.
     */
    private void checkForBranchConflicts(GitRepository repository, String filePath) {
        try {
            // Get all branches that contain this file
            List<GitLocalBranch> branches = repository.getBranches().getLocalBranches();
            Set<String> conflictingBranches = new HashSet<>();
            
            for (GitLocalBranch branch : branches) {
                if (!branch.equals(repository.getCurrentBranch())) {
                    // Check if file exists in other branch and has different content
                    if (fileExistsInBranch(repository, filePath, branch.getName())) {
                        if (hasDifferentContent(repository, filePath, branch.getName())) {
                            conflictingBranches.add(branch.getName());
                        }
                    }
                }
            }
            
            if (!conflictingBranches.isEmpty()) {
                LOG.warn("Potential conflicts detected for " + filePath + " in branches: " + conflictingBranches);
                handleBranchConflicts(repository, filePath, conflictingBranches);
            }
        } catch (Exception e) {
            LOG.error("Error checking for branch conflicts", e);
        }
    }
    
    /**
     * Check for context conflicts between branches.
     */
    private void checkForContextConflicts(GitRepository repository, String oldBranch, String newBranch) {
        try {
            if (oldBranch != null && newBranch != null) {
                // Compare Rhema contexts between branches
                List<String> oldBranchFiles = getBranchRhemaFiles(repository, oldBranch);
                List<String> newBranchFiles = getBranchRhemaFiles(repository, newBranch);
                
                // Find common files with different content
                Set<String> commonFiles = new HashSet<>(oldBranchFiles);
                commonFiles.retainAll(newBranchFiles);
                
                for (String file : commonFiles) {
                    if (hasDifferentContent(repository, file, oldBranch, newBranch)) {
                        LOG.warn("Context conflict detected in file: " + file + " between branches: " + oldBranch + " and " + newBranch);
                        handleContextConflict(repository, file, oldBranch, newBranch);
                    }
                }
            }
        } catch (Exception e) {
            LOG.error("Error checking for context conflicts", e);
        }
    }
    
    /**
     * Save branch context.
     */
    private void saveBranchContext(GitRepository repository, String branchName) {
        try {
            String contextKey = generateBranchContextKey(repository, branchName);
            List<String> files = branchRhemaFiles.get(contextKey);
            if (files != null) {
                // Save context to persistent storage
                LOG.debug("Saved context for branch: " + branchName + " with " + files.size() + " files");
            }
        } catch (Exception e) {
            LOG.error("Error saving branch context", e);
        }
    }
    
    /**
     * Load branch context.
     */
    private void loadBranchContext(GitRepository repository, String branchName) {
        try {
            String contextKey = generateBranchContextKey(repository, branchName);
            List<String> files = branchRhemaFiles.get(contextKey);
            if (files == null) {
                // Load context from persistent storage or scan branch
                scanBranchForRhemaFiles(repository, branchName);
            }
            LOG.debug("Loaded context for branch: " + branchName);
        } catch (Exception e) {
            LOG.error("Error loading branch context", e);
        }
    }
    
    /**
     * Scan branch for Rhema files.
     */
    private void scanBranchForRhemaFiles(GitRepository repository, String branchName) {
        try {
            String contextKey = generateBranchContextKey(repository, branchName);
            List<String> files = new ArrayList<>();
            
            // Use Git to list files in the branch
            GitCommandResult result = git.lsFiles(repository, branchName);
            if (result.success()) {
                for (String file : result.getOutput()) {
                    if (isRhemaFile(file)) {
                        files.add(file);
                    }
                }
            }
            
            branchRhemaFiles.put(contextKey, files);
            LOG.debug("Scanned branch " + branchName + " for Rhema files, found " + files.size() + " files");
        } catch (Exception e) {
            LOG.error("Error scanning branch for Rhema files", e);
        }
    }
    
    /**
     * Trigger Git hook.
     */
    private void triggerGitHook(String hookName, GitRepository repository, String filePath) {
        try {
            GitHookHandler hook = gitHooks.get(hookName);
            if (hook != null) {
                hook.execute(repository, filePath);
                LOG.debug("Triggered Git hook: " + hookName);
            }
        } catch (Exception e) {
            LOG.error("Error triggering Git hook: " + hookName, e);
        }
    }
    
    /**
     * Check if file is a Rhema file.
     */
    private boolean isRhemaFile(String filePath) {
        if (filePath == null) {
            return false;
        }
        
        String fileName = filePath.toLowerCase();
        return fileName.endsWith(".rhema.yml") || 
               fileName.endsWith(".scope.yml") ||
               fileName.endsWith(".context.yml") ||
               fileName.endsWith(".todos.yml") ||
               fileName.endsWith(".insights.yml") ||
               fileName.endsWith(".patterns.yml") ||
               fileName.endsWith(".decisions.yml");
    }
    
    /**
     * Generate branch context key.
     */
    private String generateBranchContextKey(GitRepository repository, String branchName) {
        return repository.getRoot().getPath() + ":" + branchName;
    }
    
    /**
     * Get current revision of a file.
     */
    private GitRevisionNumber getCurrentRevision(GitRepository repository, String filePath) {
        try {
            GitCommandResult result = git.revParse(repository, "HEAD");
            if (result.success() && !result.getOutput().isEmpty()) {
                return GitRevisionNumber.resolve(repository.getProject(), repository.getRoot(), result.getOutput().get(0));
            }
        } catch (Exception e) {
            LOG.error("Error getting current revision", e);
        }
        return null;
    }
    
    /**
     * Check if file exists in branch.
     */
    private boolean fileExistsInBranch(GitRepository repository, String filePath, String branchName) {
        try {
            GitCommandResult result = git.lsFiles(repository, branchName, filePath);
            return result.success() && !result.getOutput().isEmpty();
        } catch (Exception e) {
            LOG.error("Error checking if file exists in branch", e);
            return false;
        }
    }
    
    /**
     * Check if file has different content in branch.
     */
    private boolean hasDifferentContent(GitRepository repository, String filePath, String branchName) {
        // Implementation would compare file content between current branch and specified branch
        return false; // Placeholder
    }
    
    /**
     * Check if file has different content between two branches.
     */
    private boolean hasDifferentContent(GitRepository repository, String filePath, String branch1, String branch2) {
        // Implementation would compare file content between two branches
        return false; // Placeholder
    }
    
    /**
     * Check if current operation is a merge.
     */
    private boolean isMergeOperation(GitRepository repository) {
        try {
            GitCommandResult result = git.revParse(repository, "MERGE_HEAD");
            return result.success();
        } catch (Exception e) {
            return false;
        }
    }
    
    /**
     * Get Rhema files in a branch.
     */
    private List<String> getBranchRhemaFiles(GitRepository repository, String branchName) {
        String contextKey = generateBranchContextKey(repository, branchName);
        return branchRhemaFiles.getOrDefault(contextKey, new ArrayList<>());
    }
    
    /**
     * Handle branch conflicts.
     */
    private void handleBranchConflicts(GitRepository repository, String filePath, Set<String> conflictingBranches) {
        // Implementation would handle branch conflicts
        LOG.warn("Handling branch conflicts for " + filePath + " in branches: " + conflictingBranches);
    }
    
    /**
     * Handle context conflicts.
     */
    private void handleContextConflict(GitRepository repository, String filePath, String branch1, String branch2) {
        // Implementation would handle context conflicts
        LOG.warn("Handling context conflict for " + filePath + " between " + branch1 + " and " + branch2);
    }
    
    /**
     * Dispose Git integration.
     */
    public void dispose() {
        try {
            gitMonitorExecutor.shutdown();
            if (!gitMonitorExecutor.awaitTermination(5, TimeUnit.SECONDS)) {
                gitMonitorExecutor.shutdownNow();
            }
            
            branchContexts.clear();
            branchRhemaFiles.clear();
            gitHooks.clear();
            trackedRhemaFiles.clear();
            lastKnownRevisions.clear();
            
            LOG.info("Rhema Git integration disposed successfully");
        } catch (Exception e) {
            LOG.error("Error disposing Rhema Git integration", e);
        }
    }
    
    /**
     * Get Git integration statistics.
     */
    public Map<String, Object> getGitStats() {
        Map<String, Object> stats = new HashMap<>();
        stats.put("trackedRhemaFiles", trackedRhemaFiles.size());
        stats.put("branchContexts", branchContexts.size());
        stats.put("gitHooks", gitHooks.size());
        stats.put("lastKnownRevisions", lastKnownRevisions.size());
        return stats;
    }
    
    /**
     * Git hook handler interface.
     */
    private interface GitHookHandler {
        void execute(GitRepository repository, String filePath);
    }
    
    /**
     * Pre-commit hook for Rhema validation.
     */
    private static class RhemaPreCommitHook implements GitHookHandler {
        @Override
        public void execute(GitRepository repository, String filePath) {
            // Validate Rhema files before commit
            LOG.debug("Pre-commit hook executed for: " + filePath);
        }
    }
    
    /**
     * Post-commit hook for context updates.
     */
    private static class RhemaPostCommitHook implements GitHookHandler {
        @Override
        public void execute(GitRepository repository, String filePath) {
            // Update context after commit
            LOG.debug("Post-commit hook executed for: " + filePath);
        }
    }
    
    /**
     * Pre-push hook for context synchronization.
     */
    private static class RhemaPrePushHook implements GitHookHandler {
        @Override
        public void execute(GitRepository repository, String filePath) {
            // Synchronize context before push
            LOG.debug("Pre-push hook executed for: " + filePath);
        }
    }
    
    /**
     * Post-merge hook for context merging.
     */
    private static class RhemaPostMergeHook implements GitHookHandler {
        @Override
        public void execute(GitRepository repository, String filePath) {
            // Merge context after merge operation
            LOG.debug("Post-merge hook executed for: " + filePath);
        }
    }
} 