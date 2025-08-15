;;; rhema.el --- Rhema Git-Based Agent Context Protocol Emacs Integration -*- lexical-binding: t; -*-

;; Copyright (C) 2025 Cory Parent

;; Author: Cory Parent <cory@fugue.ai>
;; Maintainer: Cory Parent <cory@fugue.ai>
;; Version: 0.1.0
;; Package-Requires: ((emacs "27.1") (yaml-mode "0.0.15") (dash "2.19.1") (s "1.12.0"))
;; Keywords: rhema, git, context, agent, protocol, yaml
;; URL: https://github.com/fugue-ai/rhema

;; This file is part of Rhema.

;; Rhema is free software: you can redistribute it and/or modify
;; it under the terms of the Apache License, Version 2.0, as published by
;; the Apache Software Foundation.

;; Rhema is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; Apache License, Version 2.0 for more details.

;; You should have received a copy of the Apache License, Version 2.0
;; along with Rhema.  If not, see <http://www.apache.org/licenses/LICENSE-2.0>.

;;; Commentary:

;; Rhema Emacs Integration provides comprehensive IDE support for the Rhema
;; Git-Based Agent Context Protocol system. This package includes:
;;
;; - Syntax highlighting for Rhema YAML files
;; - Context-aware completion and IntelliSense
;; - Real-time validation and error checking
;; - Interactive command execution
;; - Git integration and conflict handling
;; - Performance monitoring and caching
;; - Comprehensive documentation and help system
;;
;; Installation:
;;   (require 'rhema)
;;   (rhema-mode 1)
;;
;; Usage:
;;   M-x rhema-command RET - Execute Rhema commands
;;   M-x rhema-show-context RET - Show current context
;;   M-x rhema-validate RET - Validate current file
;;   M-x rhema-completion-at-point RET - Trigger completion

;;; Code:

(require 'cl-lib)
(require 'dash)
(require 's)
(require 'yaml-mode)

;; Customization group
(defgroup rhema nil
  "Rhema Git-Based Agent Context Protocol integration."
  :group 'tools
  :prefix "rhema-")

;; Customization variables
(defcustom rhema-executable "rhema"
  "Path to the Rhema executable."
  :type 'string
  :group 'rhema)

(defcustom rhema-auto-validate t
  "Automatically validate Rhema files on save."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-show-notifications t
  "Show notifications for Rhema operations."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-intellisense t
  "Enable IntelliSense features."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-debug-mode nil
  "Enable debug mode for verbose logging."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-performance-profiling nil
  "Enable performance profiling."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-context-exploration t
  "Enable context exploration features."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-git-integration t
  "Enable Git integration features."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-auto-sync nil
  "Automatically sync with Git on file changes."
  :type 'boolean
  :group 'rhema)

(defcustom rhema-theme 'auto
  "Theme for Rhema UI elements."
  :type '(choice (const auto) (const light) (const dark))
  :group 'rhema)

(defcustom rhema-language "en"
  "Language for Rhema interface."
  :type 'string
  :group 'rhema)

;; Internal variables
(defvar rhema--initialized nil
  "Whether Rhema has been initialized.")

(defvar rhema--buffer nil
  "Current Rhema output buffer.")

(defvar rhema--window nil
  "Current Rhema output window.")

(defvar rhema--last-command nil
  "Last executed Rhema command.")

(defvar rhema--last-output nil
  "Last Rhema command output.")

(defvar rhema--error-count 0
  "Number of errors in current session.")

(defvar rhema--warning-count 0
  "Number of warnings in current session.")

(defvar rhema--cache (make-hash-table :test 'equal)
  "Cache for Rhema command results.")

(defvar rhema--cache-ttl 300
  "Time-to-live for cache entries in seconds.")

(defvar rhema--cache-timestamps (make-hash-table :test 'equal)
  "Timestamps for cache entries.")

;; Utility functions
(defun rhema--log (message &optional level)
  "Log a message with optional level."
  (when (or rhema-debug-mode (eq level 'error))
    (message "[RHEMA] %s" message)))

(defun rhema--error (message)
  "Log an error message."
  (rhema--log message 'error)
  (message "RHEMA Error: %s" message))

(defun rhema--warning (message)
  "Log a warning message."
  (rhema--log message 'warning)
  (message "RHEMA Warning: %s" message))

(defun rhema--info (message)
  "Log an info message."
  (when rhema-show-notifications
    (rhema--log message 'info)
    (message "RHEMA: %s" message)))

(defun rhema--success (message)
  "Log a success message."
  (when rhema-show-notifications
    (rhema--log message 'success)
    (message "RHEMA: %s" message)))

;; Command execution
(defun rhema--execute-command (command &optional args callback)
  "Execute a Rhema command with optional arguments and callback."
  (let* ((full-command (if args
                          (concat rhema-executable " " command " " (s-join " " args))
                        (concat rhema-executable " " command)))
         (start-time (current-time))
         (process-name (format "rhema-%s" (random 10000))))
    
    (rhema--log (format "Executing: %s" full-command))
    
    (let ((process (start-process-shell-command
                   process-name
                   (get-buffer-create "*rhema-output*")
                   full-command)))
      
      (set-process-sentinel
       process
       (lambda (proc event)
         (let ((end-time (current-time))
               (duration (float-time (time-subtract end-time start-time))))
           (rhema--log (format "Command completed in %.2f seconds" duration))
           
           (when (and callback (eq (process-exit-status proc) 0))
             (funcall callback (process-buffer proc)))
           
           (when rhema-performance-profiling
             (rhema--log (format "Performance: %s took %.2f seconds" command duration))))))
      
      (setq rhema--last-command command)
      process)))

;; Cache management
(defun rhema--cache-get (key)
  "Get a value from the cache."
  (let ((timestamp (gethash key rhema--cache-timestamps))
        (value (gethash key rhema--cache)))
    (when (and timestamp value)
      (let ((age (- (float-time) timestamp)))
        (if (< age rhema--cache-ttl)
            value
          (rhema--cache-remove key)
          nil)))))

(defun rhema--cache-set (key value)
  "Set a value in the cache."
  (puthash key value rhema--cache)
  (puthash key (float-time) rhema--cache-timestamps))

(defun rhema--cache-remove (key)
  "Remove a value from the cache."
  (remhash key rhema--cache)
  (remhash key rhema--cache-timestamps))

(defun rhema--cache-clear ()
  "Clear the entire cache."
  (clrhash rhema--cache)
  (clrhash rhema--cache-timestamps)
  (rhema--info "Cache cleared"))

(defun rhema--cache-stats ()
  "Get cache statistics."
  (let ((size (hash-table-count rhema--cache)))
    (format "Cache size: %d entries" size)))

;; File type detection
(defun rhema--rhema-file-p (&optional buffer)
  "Check if the current buffer contains Rhema content."
  (with-current-buffer (or buffer (current-buffer))
    (and (derived-mode-p 'yaml-mode)
         (or (string-match "\\.rhema\\.ya?ml$" (buffer-file-name))
             (string-match "rhema:" (buffer-substring-no-properties (point-min) (min 1000 (point-max))))))))

;; Context detection
(defun rhema--get-context ()
  "Get the current Rhema context."
  (let ((cache-key "context"))
    (or (rhema--cache-get cache-key)
        (let ((context (rhema--execute-command "context")))
          (rhema--cache-set cache-key context)
          context))))

;; Validation
(defun rhema--validate-file (&optional file)
  "Validate a Rhema file."
  (let* ((file (or file (buffer-file-name)))
         (cache-key (format "validate:%s" file)))
    (or (rhema--cache-get cache-key)
        (let ((result (rhema--execute-command "validate" (list file))))
          (rhema--cache-set cache-key result)
          result))))

;; Completion
(defun rhema--completion-at-point ()
  "Provide completion at point for Rhema files."
  (when (and rhema-intellisense (rhema--rhema-file-p))
    (let* ((bounds (bounds-of-thing-at-point 'symbol))
           (start (car bounds))
           (end (cdr bounds))
           (prefix (buffer-substring-no-properties start end))
           (completions (rhema--get-completions prefix)))
      (when completions
        (list start end completions :exclusive 'no)))))

(defun rhema--get-completions (prefix)
  "Get completions for a prefix."
  (let ((cache-key (format "completions:%s" prefix)))
    (or (rhema--cache-get cache-key)
        (let ((completions (rhema--execute-command "completions" (list prefix))))
          (rhema--cache-set cache-key completions)
          completions))))

;; Interactive commands
(defun rhema-command (command &optional args)
  "Execute a Rhema command interactively."
  (interactive "sRhema command: \nsArguments: ")
  (let ((args-list (when (not (string-empty-p args))
                     (split-string args))))
    (rhema--execute-command command args-list)))

(defun rhema-show-context ()
  "Show the current Rhema context."
  (interactive)
  (let ((context (rhema--get-context)))
    (with-current-buffer (get-buffer-create "*rhema-context*")
      (erase-buffer)
      (insert context)
      (yaml-mode)
      (pop-to-buffer (current-buffer)))))

(defun rhema-validate ()
  "Validate the current Rhema file."
  (interactive)
  (when (rhema--rhema-file-p)
    (let ((result (rhema--validate-file)))
      (if (string-match "error" result)
          (rhema--error result)
        (rhema--success "File validation passed")))))

(defun rhema-show-scopes ()
  "Show available Rhema scopes."
  (interactive)
  (rhema-command "scopes"))

(defun rhema-show-tree ()
  "Show the Rhema scope tree."
  (interactive)
  (rhema-command "tree"))

(defun rhema-manage-todos ()
  "Manage Rhema todos."
  (interactive)
  (rhema-command "todos"))

(defun rhema-manage-insights ()
  "Manage Rhema insights."
  (interactive)
  (rhema-command "insights"))

(defun rhema-manage-patterns ()
  "Manage Rhema patterns."
  (interactive)
  (rhema-command "patterns"))

(defun rhema-manage-decisions ()
  "Manage Rhema decisions."
  (interactive)
  (rhema-command "decisions"))

(defun rhema-show-dependencies ()
  "Show Rhema dependencies."
  (interactive)
  (rhema-command "dependencies"))

(defun rhema-show-impact ()
  "Show Rhema impact analysis."
  (interactive)
  (rhema-command "impact"))

(defun rhema-sync-knowledge ()
  "Sync Rhema knowledge."
  (interactive)
  (rhema-command "sync"))

(defun rhema-git-integration ()
  "Show Git integration status."
  (interactive)
  (rhema-command "git"))

(defun rhema-show-stats ()
  "Show Rhema statistics."
  (interactive)
  (rhema-command "stats"))

(defun rhema-check-health ()
  "Check Rhema health."
  (interactive)
  (rhema-command "health"))

(defun rhema-debug-context ()
  "Debug Rhema context."
  (interactive)
  (rhema-command "debug"))

(defun rhema-profile-performance ()
  "Profile Rhema performance."
  (interactive)
  (rhema-command "profile"))

(defun rhema-refactor-context ()
  "Refactor Rhema context."
  (interactive)
  (rhema-command "refactor"))

(defun rhema-generate-code ()
  "Generate code using Rhema."
  (interactive)
  (rhema-command "generate"))

(defun rhema-show-documentation ()
  "Show Rhema documentation."
  (interactive)
  (rhema-command "docs"))

(defun rhema-configure-settings ()
  "Configure Rhema settings."
  (interactive)
  (rhema-command "config"))

(defun rhema-show-sidebar ()
  "Show Rhema sidebar."
  (interactive)
  (rhema-command "sidebar"))

(defun rhema-status ()
  "Show Rhema status."
  (interactive)
  (let ((status (format "Rhema Status:
- Initialized: %s
- Auto-validate: %s
- IntelliSense: %s
- Git integration: %s
- Cache: %s
- Errors: %d
- Warnings: %d"
                        rhema--initialized
                        rhema-auto-validate
                        rhema-intellisense
                        rhema-git-integration
                        (rhema--cache-stats)
                        rhema--error-count
                        rhema--warning-count)))
    (message status)))

(defun rhema-cache-clear ()
  "Clear the Rhema cache."
  (interactive)
  (rhema--cache-clear))

(defun rhema-cache-stats ()
  "Show Rhema cache statistics."
  (interactive)
  (message (rhema--cache-stats)))

;; Mode definition
(define-derived-mode rhema-mode yaml-mode "Rhema"
  "Major mode for editing Rhema YAML files."
  :group 'rhema
  
  ;; Set up completion
  (when rhema-intellisense
    (add-hook 'completion-at-point-functions #'rhema--completion-at-point nil t))
  
  ;; Set up auto-validation
  (when rhema-auto-validate
    (add-hook 'after-save-hook #'rhema-validate nil t))
  
  ;; Set up Git integration
  (when rhema-git-integration
    (add-hook 'after-save-hook #'rhema-sync-knowledge nil t)))

;; Key bindings
(defvar rhema-mode-map
  (let ((map (make-sparse-keymap)))
    (define-key map (kbd "C-c C-c") #'rhema-command)
    (define-key map (kbd "C-c C-v") #'rhema-validate)
    (define-key map (kbd "C-c C-s") #'rhema-show-context)
    (define-key map (kbd "C-c C-t") #'rhema-show-tree)
    (define-key map (kbd "C-c C-d") #'rhema-manage-todos)
    (define-key map (kbd "C-c C-i") #'rhema-manage-insights)
    (define-key map (kbd "C-c C-p") #'rhema-manage-patterns)
    (define-key map (kbd "C-c C-e") #'rhema-manage-decisions)
    (define-key map (kbd "C-c C-g") #'rhema-git-integration)
    (define-key map (kbd "C-c C-h") #'rhema-check-health)
    (define-key map (kbd "C-c C-r") #'rhema-refactor-context)
    (define-key map (kbd "C-c C-y") #'rhema-sync-knowledge)
    (define-key map (kbd "C-c C-u") #'rhema-cache-clear)
    (define-key map (kbd "C-c C-?") #'rhema-status)
    map)
  "Keymap for Rhema mode.")

;; Auto-mode-alist
(add-to-list 'auto-mode-alist '("\\.rhema\\.ya?ml\\'" . rhema-mode))

;; Menu
(easy-menu-define rhema-menu rhema-mode-map
  "Rhema menu"
  '("Rhema"
    ["Execute Command" rhema-command t]
    ["Show Context" rhema-show-context t]
    ["Validate File" rhema-validate t]
    ["Show Scopes" rhema-show-scopes t]
    ["Show Tree" rhema-show-tree t]
    "---"
    ["Manage Todos" rhema-manage-todos t]
    ["Manage Insights" rhema-manage-insights t]
    ["Manage Patterns" rhema-manage-patterns t]
    ["Manage Decisions" rhema-manage-decisions t]
    "---"
    ["Show Dependencies" rhema-show-dependencies t]
    ["Show Impact" rhema-show-impact t]
    ["Sync Knowledge" rhema-sync-knowledge t]
    ["Git Integration" rhema-git-integration t]
    "---"
    ["Show Stats" rhema-show-stats t]
    ["Check Health" rhema-check-health t]
    ["Debug Context" rhema-debug-context t]
    ["Profile Performance" rhema-profile-performance t]
    ["Refactor Context" rhema-refactor-context t]
    ["Generate Code" rhema-generate-code t]
    "---"
    ["Show Documentation" rhema-show-documentation t]
    ["Configure Settings" rhema-configure-settings t]
    ["Show Sidebar" rhema-show-sidebar t]
    ["Show Status" rhema-status t]
    "---"
    ["Clear Cache" rhema-cache-clear t]
    ["Cache Stats" rhema-cache-stats t]))

;; Initialization
(defun rhema-initialize ()
  "Initialize the Rhema package."
  (unless rhema--initialized
    (rhema--log "Initializing Rhema package")
    (setq rhema--initialized t)
    (rhema--info "Rhema package initialized")))

;; Provide the package
(provide 'rhema)

;;; rhema.el ends here 