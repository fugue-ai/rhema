;;; rhema-test.el --- Tests for Rhema Emacs Integration -*- lexical-binding: t; -*-

;; Copyright (C) 2025 Cory Parent

;; Author: Cory Parent <cory@fugue.ai>
;; Version: 0.1.0

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

;; Test suite for the Rhema Emacs integration package.
;; Run tests with: M-x ert RET t RET

;;; Code:

(require 'ert)
(require 'rhema)

;; Test utilities
(defun rhema-test--create-temp-file (content)
  "Create a temporary file with CONTENT."
  (let ((temp-file (make-temp-file "rhema-test-" nil ".yml")))
    (with-temp-file temp-file
      (insert content))
    temp-file))

(defun rhema-test--cleanup-temp-file (file)
  "Clean up temporary FILE."
  (when (and file (file-exists-p file))
    (delete-file file)))

;; Test data
(defconst rhema-test--sample-rhema-file
  "rhema:
  version: 1.0
  scope: test-scope
  context:
    name: test-context
    description: Test context for unit tests
  todos:
    - name: test-todo
      description: Test todo item
      priority: high
  insights:
    - name: test-insight
      description: Test insight
      category: architecture
  patterns:
    - name: test-pattern
      description: Test pattern
      type: design
  decisions:
    - name: test-decision
      description: Test decision
      status: approved")

(defconst rhema-test--sample-yaml-file
  "version: 1.0
name: test
description: Test YAML file")

;; Test setup and teardown
(defun rhema-test--setup ()
  "Set up test environment."
  (setq rhema-debug-mode t)
  (setq rhema-show-notifications nil)
  (rhema-initialize))

(defun rhema-test--teardown ()
  "Clean up test environment."
  (setq rhema-debug-mode nil)
  (setq rhema-show-notifications t)
  (rhema--cache-clear))

;; Test cases

(ert-deftest rhema-test-initialization ()
  "Test Rhema package initialization."
  (rhema-test--setup)
  (should rhema--initialized)
  (rhema-test--teardown))

(ert-deftest rhema-test-rhema-file-detection ()
  "Test Rhema file detection."
  (rhema-test--setup)
  
  ;; Test with .rhema.yml file
  (let ((temp-file (rhema-test--create-temp-file rhema-test--sample-rhema-file)))
    (unwind-protect
        (with-temp-buffer
          (insert-file-contents temp-file)
          (yaml-mode)
          (should (rhema--rhema-file-p)))
      (rhema-test--cleanup-temp-file temp-file)))
  
  ;; Test with regular YAML file
  (let ((temp-file (rhema-test--create-temp-file rhema-test--sample-yaml-file)))
    (unwind-protect
        (with-temp-buffer
          (insert-file-contents temp-file)
          (yaml-mode)
          (should-not (rhema--rhema-file-p)))
      (rhema-test--cleanup-temp-file temp-file)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-cache-operations ()
  "Test cache operations."
  (rhema-test--setup)
  
  ;; Test cache set and get
  (rhema--cache-set "test-key" "test-value")
  (should (equal (rhema--cache-get "test-key") "test-value"))
  
  ;; Test cache remove
  (rhema--cache-remove "test-key")
  (should-not (rhema--cache-get "test-key"))
  
  ;; Test cache clear
  (rhema--cache-set "key1" "value1")
  (rhema--cache-set "key2" "value2")
  (rhema--cache-clear)
  (should-not (rhema--cache-get "key1"))
  (should-not (rhema--cache-get "key2"))
  
  ;; Test cache stats
  (should (stringp (rhema--cache-stats)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-logging-functions ()
  "Test logging functions."
  (rhema-test--setup)
  
  ;; Test info logging
  (should-not (rhema--info "Test info message"))
  
  ;; Test error logging
  (should-not (rhema--error "Test error message"))
  
  ;; Test warning logging
  (should-not (rhema--warning "Test warning message"))
  
  ;; Test success logging
  (should-not (rhema--success "Test success message"))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-command-execution ()
  "Test command execution."
  (rhema-test--setup)
  
  ;; Test command execution with callback
  (let ((callback-called nil))
    (let ((process (rhema--execute-command "version" nil
                                          (lambda (buffer)
                                            (setq callback-called t)))))
      (should (processp process))
      ;; Wait for process to complete
      (while (process-live-p process)
        (sit-for 0.1))
      (should callback-called)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-completion-system ()
  "Test completion system."
  (rhema-test--setup)
  
  ;; Test completion at point
  (let ((temp-file (rhema-test--create-temp-file rhema-test--sample-rhema-file)))
    (unwind-protect
        (with-temp-buffer
          (insert-file-contents temp-file)
          (yaml-mode)
          (goto-char (point-min))
          (should (functionp #'rhema--completion-at-point)))
      (rhema-test--cleanup-temp-file temp-file)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-validation-system ()
  "Test validation system."
  (rhema-test--setup)
  
  ;; Test file validation
  (let ((temp-file (rhema-test--create-temp-file rhema-test--sample-rhema-file)))
    (unwind-protect
        (with-temp-buffer
          (insert-file-contents temp-file)
          (yaml-mode)
          (should (functionp #'rhema--validate-file)))
      (rhema-test--cleanup-temp-file temp-file)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-context-detection ()
  "Test context detection."
  (rhema-test--setup)
  
  ;; Test context detection
  (should (functionp #'rhema--get-context))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-interactive-commands ()
  "Test interactive commands."
  (rhema-test--setup)
  
  ;; Test that all interactive commands are functions
  (should (functionp #'rhema-command))
  (should (functionp #'rhema-show-context))
  (should (functionp #'rhema-validate))
  (should (functionp #'rhema-show-scopes))
  (should (functionp #'rhema-show-tree))
  (should (functionp #'rhema-manage-todos))
  (should (functionp #'rhema-manage-insights))
  (should (functionp #'rhema-manage-patterns))
  (should (functionp #'rhema-manage-decisions))
  (should (functionp #'rhema-show-dependencies))
  (should (functionp #'rhema-show-impact))
  (should (functionp #'rhema-sync-knowledge))
  (should (functionp #'rhema-git-integration))
  (should (functionp #'rhema-show-stats))
  (should (functionp #'rhema-check-health))
  (should (functionp #'rhema-debug-context))
  (should (functionp #'rhema-profile-performance))
  (should (functionp #'rhema-refactor-context))
  (should (functionp #'rhema-generate-code))
  (should (functionp #'rhema-show-documentation))
  (should (functionp #'rhema-configure-settings))
  (should (functionp #'rhema-show-sidebar))
  (should (functionp #'rhema-status))
  (should (functionp #'rhema-cache-clear))
  (should (functionp #'rhema-cache-stats))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-mode-definition ()
  "Test Rhema mode definition."
  (rhema-test--setup)
  
  ;; Test mode definition
  (should (derived-mode-p 'rhema-mode 'yaml-mode))
  (should (keymapp rhema-mode-map))
  
  ;; Test key bindings
  (should (lookup-key rhema-mode-map (kbd "C-c C-c")))
  (should (lookup-key rhema-mode-map (kbd "C-c C-v")))
  (should (lookup-key rhema-mode-map (kbd "C-c C-s")))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-auto-mode-alist ()
  "Test auto-mode-alist configuration."
  (rhema-test--setup)
  
  ;; Test that .rhema.yml files are associated with rhema-mode
  (should (assoc "\\.rhema\\.ya?ml\\'" auto-mode-alist))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-customization-variables ()
  "Test customization variables."
  (rhema-test--setup)
  
  ;; Test that all customization variables are defined
  (should (boundp 'rhema-executable))
  (should (boundp 'rhema-auto-validate))
  (should (boundp 'rhema-show-notifications))
  (should (boundp 'rhema-intellisense))
  (should (boundp 'rhema-debug-mode))
  (should (boundp 'rhema-performance-profiling))
  (should (boundp 'rhema-context-exploration))
  (should (boundp 'rhema-git-integration))
  (should (boundp 'rhema-auto-sync))
  (should (boundp 'rhema-theme))
  (should (boundp 'rhema-language))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-internal-variables ()
  "Test internal variables."
  (rhema-test--setup)
  
  ;; Test that all internal variables are defined
  (should (boundp 'rhema--initialized))
  (should (boundp 'rhema--buffer))
  (should (boundp 'rhema--window))
  (should (boundp 'rhema--last-command))
  (should (boundp 'rhema--last-output))
  (should (boundp 'rhema--error-count))
  (should (boundp 'rhema--warning-count))
  (should (boundp 'rhema--cache))
  (should (boundp 'rhema--cache-ttl))
  (should (boundp 'rhema--cache-timestamps))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-menu-integration ()
  "Test menu integration."
  (rhema-test--setup)
  
  ;; Test that menu is defined
  (should (boundp 'rhema-menu))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-performance-monitoring ()
  "Test performance monitoring."
  (rhema-test--setup)
  
  ;; Test performance profiling
  (let ((rhema-performance-profiling t))
    (let ((process (rhema--execute-command "version")))
      (should (processp process))
      ;; Wait for process to complete
      (while (process-live-p process)
        (sit-for 0.1))))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-error-handling ()
  "Test error handling."
  (rhema-test--setup)
  
  ;; Test error handling with invalid command
  (let ((process (rhema--execute-command "invalid-command")))
    (should (processp process))
    ;; Wait for process to complete
    (while (process-live-p process)
      (sit-for 0.1)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-git-integration ()
  "Test Git integration."
  (rhema-test--setup)
  
  ;; Test Git integration command
  (should (functionp #'rhema-git-integration))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-documentation-system ()
  "Test documentation system."
  (rhema-test--setup)
  
  ;; Test documentation command
  (should (functionp #'rhema-show-documentation))
  
  (rhema-test--teardown))

;; Integration tests

(ert-deftest rhema-test-integration-workflow ()
  "Test complete workflow integration."
  (rhema-test--setup)
  
  ;; Create a test file
  (let ((temp-file (rhema-test--create-temp-file rhema-test--sample-rhema-file)))
    (unwind-protect
        (with-temp-buffer
          ;; Load the file
          (insert-file-contents temp-file)
          (yaml-mode)
          
          ;; Test file detection
          (should (rhema--rhema-file-p))
          
          ;; Test validation
          (should (functionp #'rhema--validate-file))
          
          ;; Test completion
          (should (functionp #'rhema--completion-at-point))
          
          ;; Test context detection
          (should (functionp #'rhema--get-context)))
      (rhema-test--cleanup-temp-file temp-file)))
  
  (rhema-test--teardown))

(ert-deftest rhema-test-cache-performance ()
  "Test cache performance."
  (rhema-test--setup)
  
  ;; Test cache performance with multiple operations
  (let ((start-time (current-time)))
    (dotimes (i 100)
      (rhema--cache-set (format "key-%d" i) (format "value-%d" i)))
    
    (dotimes (i 100)
      (rhema--cache-get (format "key-%d" i)))
    
    (let ((end-time (current-time))
          (duration (float-time (time-subtract end-time start-time))))
      (should (< duration 1.0)))) ;; Should complete in less than 1 second
  
  (rhema-test--teardown))

;; Performance tests

(ert-deftest rhema-test-command-execution-performance ()
  "Test command execution performance."
  (rhema-test--setup)
  
  ;; Test command execution performance
  (let ((start-time (current-time)))
    (let ((process (rhema--execute-command "version")))
      (while (process-live-p process)
        (sit-for 0.1))
      (let ((end-time (current-time))
            (duration (float-time (time-subtract end-time start-time))))
        (should (< duration 5.0)))) ;; Should complete in less than 5 seconds
  
  (rhema-test--teardown))

;; Stress tests

(ert-deftest rhema-test-stress-cache-operations ()
  "Stress test cache operations."
  (rhema-test--setup)
  
  ;; Stress test with many cache operations
  (dotimes (i 1000)
    (rhema--cache-set (format "stress-key-%d" i) (format "stress-value-%d" i)))
  
  ;; Verify all values are cached
  (dotimes (i 1000)
    (should (equal (rhema--cache-get (format "stress-key-%d" i))
                   (format "stress-value-%d" i))))
  
  ;; Clear cache
  (rhema--cache-clear)
  
  ;; Verify cache is empty
  (dotimes (i 1000)
    (should-not (rhema--cache-get (format "stress-key-%d" i))))
  
  (rhema-test--teardown))

;; Provide test module
(provide 'rhema-test)

;;; rhema-test.el ends here 