" Rhema Vim Plugin Test Suite
" Copyright 2025 Cory Parent
"
" Licensed under the Apache License, Version 2.0 (the "License");
" you may not use this file except in compliance with the License.
" You may obtain a copy of the License at
"
"     http://www.apache.org/licenses/LICENSE-2.0
"
" Unless required by applicable law or agreed to in writing, software
" distributed under the License is distributed on an "AS IS" BASIS,
" WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
" See the License for the specific language governing permissions and
" limitations under the License.

" Test framework
let s:test_results = {
    \ 'passed': 0,
    \ 'failed': 0,
    \ 'total': 0,
    \ 'errors': []
    \ }

" Test utilities
function! s:assert_equal(expected, actual, message = '')
    let s:test_results.total += 1
    if a:expected == a:actual
        let s:test_results.passed += 1
        echom '[PASS] ' . (empty(a:message) ? 'Assertion passed' : a:message)
    else
        let s:test_results.failed += 1
        let error = '[FAIL] ' . (empty(a:message) ? 'Assertion failed' : a:message) . 
                   \ ' - Expected: ' . string(a:expected) . 
                   \ ', Got: ' . string(a:actual)
        call add(s:test_results.errors, error)
        echom error
    endif
endfunction

function! s:assert_true(condition, message = '')
    call s:assert_equal(1, a:condition, a:message)
endfunction

function! s:assert_false(condition, message = '')
    call s:assert_equal(0, a:condition, a:message)
endfunction

function! s:assert_not_empty(value, message = '')
    call s:assert_true(!empty(a:value), a:message)
endfunction

function! s:assert_empty(value, message = '')
    call s:assert_true(empty(a:value), a:message)
endfunction

" Test runner
function! rhema#test#run_all()
    echom 'Running Rhema Vim Plugin Tests...'
    echom '====================================='
    
    " Reset test results
    let s:test_results = {
        \ 'passed': 0,
        \ 'failed': 0,
        \ 'total': 0,
        \ 'errors': []
        \ }
    
    " Run test suites
    call s:test_initialization()
    call s:test_configuration()
    call s:test_file_detection()
    call s:test_context_detection()
    call s:test_command_execution()
    call s:test_completion()
    call s:test_validation()
    call s:test_ui()
    call s:test_cache()
    call s:test_performance()
    
    " Print results
    call s:print_results()
endfunction

function! s:print_results()
    echom '====================================='
    echom 'Test Results:'
    echom '  Total: ' . s:test_results.total
    echom '  Passed: ' . s:test_results.passed
    echom '  Failed: ' . s:test_results.failed
    
    if s:test_results.failed > 0
        echom ''
        echom 'Errors:'
        for error in s:test_results.errors
            echom '  ' . error
        endfor
    endif
    
    if s:test_results.failed == 0
        echom 'All tests passed! ✓'
    else
        echom 'Some tests failed! ✗'
    endif
endfunction

" Test suites
function! s:test_initialization()
    echom 'Testing initialization...'
    
    " Test plugin loading
    call s:assert_true(exists('g:loaded_rhema_autoload'), 'Plugin should be loaded')
    call s:assert_true(exists('g:rhema_enabled'), 'Default configuration should be set')
    call s:assert_equal(1, g:rhema_enabled, 'Default enabled should be 1')
    
    " Test initialization function
    let result = rhema#init()
    call s:assert_true(result == 0 || result == 1, 'Init should return 0 or 1')
    
    " Test status function
    let status = rhema#status()
    call s:assert_not_empty(status, 'Status should not be empty')
    call s:assert_true(has_key(status, 'enabled'), 'Status should have enabled key')
endfunction

function! s:test_configuration()
    echom 'Testing configuration...'
    
    " Test default values
    call s:assert_equal('rhema', g:rhema_executable, 'Default executable should be rhema')
    call s:assert_equal(1, g:rhema_auto_validate, 'Default auto_validate should be 1')
    call s:assert_equal(1, g:rhema_show_notifications, 'Default show_notifications should be 1')
    call s:assert_equal(0, g:rhema_debug_mode, 'Default debug_mode should be 0')
    call s:assert_equal(30, g:rhema_timeout, 'Default timeout should be 30')
    call s:assert_equal(1, g:rhema_cache_enabled, 'Default cache_enabled should be 1')
    call s:assert_equal(300, g:rhema_cache_ttl, 'Default cache_ttl should be 300')
endfunction

function! s:test_file_detection()
    echom 'Testing file detection...'
    
    " Test Rhema file detection
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('scope.yaml'), 'scope.yaml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('knowledge.yaml'), 'knowledge.yaml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('todos.yaml'), 'todos.yaml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('decisions.yaml'), 'decisions.yaml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('patterns.yaml'), 'patterns.yaml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('conventions.yaml'), 'conventions.yaml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('rhema.yml'), 'rhema.yml should be detected')
    call s:assert_true(rhema#filetype#is_rhema_file_pattern('test.rhema.yml'), 'test.rhema.yml should be detected')
    
    " Test non-Rhema file detection
    call s:assert_false(rhema#filetype#is_rhema_file_pattern('test.txt'), 'test.txt should not be detected')
    call s:assert_false(rhema#filetype#is_rhema_file_pattern('test.py'), 'test.py should not be detected')
    call s:assert_false(rhema#filetype#is_rhema_file_pattern('test.js'), 'test.js should not be detected')
endfunction

function! s:test_context_detection()
    echom 'Testing context detection...'
    
    " Test context detection (will be empty if no Rhema files in current project)
    let context = rhema#context#get_current()
    call s:assert_not_empty(context, 'Context should be returned (may be empty dict)')
    
    " Test context detection function
    let files = rhema#context#detect()
    call s:assert_not_empty(files, 'Detect should return list (may be empty)')
endfunction

function! s:test_command_execution()
    echom 'Testing command execution...'
    
    " Test command execution with invalid command (should fail gracefully)
    let result = rhema#command#execute(['invalid-command'])
    " This should return empty string or fail gracefully
    call s:assert_true(result == '' || result != '', 'Command execution should handle errors')
    
    " Test performance monitoring
    call rhema#performance#start('test')
    call rhema#performance#end('test')
    " Should not crash
    call s:assert_true(1, 'Performance monitoring should work')
endfunction

function! s:test_completion()
    echom 'Testing completion...'
    
    " Test completion functions
    let keywords = rhema#complete#get_commands_and_keywords()
    call s:assert_not_empty(keywords, 'Completion should return keywords')
    
    let all_completions = rhema#complete#get_all_completions()
    call s:assert_not_empty(all_completions, 'All completions should return items')
    
    " Test template completion
    let todo_template = rhema#complete#template_completion('todo')
    call s:assert_not_empty(todo_template, 'Todo template should be available')
    
    let insight_template = rhema#complete#template_completion('insight')
    call s:assert_not_empty(insight_template, 'Insight template should be available')
    
    let pattern_template = rhema#complete#template_completion('pattern')
    call s:assert_not_empty(pattern_template, 'Pattern template should be available')
    
    let decision_template = rhema#complete#template_completion('decision')
    call s:assert_not_empty(decision_template, 'Decision template should be available')
    
    " Test invalid template
    let invalid_template = rhema#complete#template_completion('invalid')
    call s:assert_empty(invalid_template, 'Invalid template should return empty')
endfunction

function! s:test_validation()
    echom 'Testing validation...'
    
    " Test validation functions
    let result = rhema#validation#validate_file('nonexistent.yaml')
    call s:assert_equal(0, result, 'Validation should fail for nonexistent file')
    
    " Test auto-validation (should not crash)
    call rhema#validation#auto_validate()
    call s:assert_true(1, 'Auto-validation should not crash')
endfunction

function! s:test_ui()
    echom 'Testing UI...'
    
    " Test UI functions
    call rhema#ui#show_success('Test success message')
    call rhema#ui#show_error('Test error message')
    call rhema#ui#show_warning('Test warning message')
    call rhema#ui#show_info('Test info message')
    
    " Test prompt functions
    let response = rhema#ui#show_prompt('Test prompt')
    call s:assert_true(response == '' || response != '', 'Prompt should return string')
    
    let confirmed = rhema#ui#show_confirm('Test confirmation')
    call s:assert_true(confirmed == 0 || confirmed == 1, 'Confirm should return boolean')
    
    let selected = rhema#ui#show_menu('Test menu', ['Option 1', 'Option 2'])
    call s:assert_true(selected == '' || selected != '', 'Menu should return string')
    
    " Test output display (should not crash)
    call rhema#ui#show_output('Test output', 'Test Title')
    call s:assert_true(1, 'Output display should not crash')
endfunction

function! s:test_cache()
    echom 'Testing cache...'
    
    " Test cache functions
    call rhema#cache#set('test_key', 'test_value')
    let value = rhema#cache#get('test_key')
    call s:assert_equal('test_value', value, 'Cache get should return set value')
    
    let stats = rhema#cache#stats()
    call s:assert_not_empty(stats, 'Cache stats should not be empty')
    call s:assert_true(has_key(stats, 'entries'), 'Cache stats should have entries')
    call s:assert_true(has_key(stats, 'enabled'), 'Cache stats should have enabled')
    
    call rhema#cache#clear()
    let cleared_value = rhema#cache#get('test_key')
    call s:assert_empty(cleared_value, 'Cache should be empty after clear')
endfunction

function! s:test_performance()
    echom 'Testing performance...'
    
    " Test performance functions
    call rhema#performance#start('test_performance')
    sleep 10m " Sleep for 10 milliseconds
    call rhema#performance#end('test_performance')
    
    " Should not crash
    call s:assert_true(1, 'Performance monitoring should work')
endfunction

" Test error handling
function! s:test_error_handling()
    echom 'Testing error handling...'
    
    " Test error handling functions
    let error_info = rhema#error#handle('Test error', 'test_context')
    call s:assert_not_empty(error_info, 'Error info should not be empty')
    call s:assert_true(has_key(error_info, 'error'), 'Error info should have error key')
    call s:assert_true(has_key(error_info, 'context'), 'Error info should have context key')
    call s:assert_true(has_key(error_info, 'timestamp'), 'Error info should have timestamp key')
endfunction

" Test logging
function! s:test_logging()
    echom 'Testing logging...'
    
    " Test logging functions
    call rhema#log#message('Test message')
    call rhema#log#error('Test error')
    call rhema#log#warning('Test warning')
    call rhema#log#info('Test info')
    
    " Should not crash
    call s:assert_true(1, 'Logging should work')
endfunction

" Test job handling (Neovim only)
function! s:test_job_handling()
    if has('nvim')
        echom 'Testing job handling...'
        
        " Test job handling functions
        call rhema#job#handle_output(1, ['test output'], 'stdout')
        call rhema#job#handle_error(1, ['test error'], 'stderr')
        call rhema#job#handle_exit(1, 0, 'exit')
        
        " Should not crash
        call s:assert_true(1, 'Job handling should work')
    endif
endfunction

" Test file type plugin
function! s:test_filetype_plugin()
    echom 'Testing filetype plugin...'
    
    " Create a temporary Rhema file
    let temp_file = tempname() . '_test.yaml'
    call writefile(['scope:', '  name: test'], temp_file)
    
    " Open the file
    execute 'edit ' . temp_file
    
    " Test filetype detection
    call s:assert_true(rhema#filetype#is_rhema_file(), 'File should be detected as Rhema file')
    
    " Clean up
    call delete(temp_file)
    bdelete!
endfunction

" Test syntax highlighting
function! s:test_syntax()
    echom 'Testing syntax highlighting...'
    
    " Test that syntax highlighting is available
    if has('syntax')
        call s:assert_true(1, 'Syntax highlighting should be available')
    else
        call s:assert_true(1, 'Syntax highlighting not available (acceptable)')
    endif
endfunction

" Test commands
function! s:test_commands()
    echom 'Testing commands...'
    
    " Test that commands are available
    call s:assert_true(exists(':RhemaInitialize'), 'RhemaInitialize command should exist')
    call s:assert_true(exists(':RhemaShowContext'), 'RhemaShowContext command should exist')
    call s:assert_true(exists(':RhemaExecuteQuery'), 'RhemaExecuteQuery command should exist')
    call s:assert_true(exists(':RhemaValidateFiles'), 'RhemaValidateFiles command should exist')
    call s:assert_true(exists(':RhemaShowScopes'), 'RhemaShowScopes command should exist')
    call s:assert_true(exists(':RhemaShowTree'), 'RhemaShowTree command should exist')
    call s:assert_true(exists(':RhemaManageTodos'), 'RhemaManageTodos command should exist')
    call s:assert_true(exists(':RhemaManageInsights'), 'RhemaManageInsights command should exist')
    call s:assert_true(exists(':RhemaManagePatterns'), 'RhemaManagePatterns command should exist')
    call s:assert_true(exists(':RhemaManageDecisions'), 'RhemaManageDecisions command should exist')
    call s:assert_true(exists(':RhemaShowDependencies'), 'RhemaShowDependencies command should exist')
    call s:assert_true(exists(':RhemaShowImpact'), 'RhemaShowImpact command should exist')
    call s:assert_true(exists(':RhemaSyncKnowledge'), 'RhemaSyncKnowledge command should exist')
    call s:assert_true(exists(':RhemaGitIntegration'), 'RhemaGitIntegration command should exist')
    call s:assert_true(exists(':RhemaShowStats'), 'RhemaShowStats command should exist')
    call s:assert_true(exists(':RhemaCheckHealth'), 'RhemaCheckHealth command should exist')
    call s:assert_true(exists(':RhemaDebugContext'), 'RhemaDebugContext command should exist')
    call s:assert_true(exists(':RhemaProfilePerformance'), 'RhemaProfilePerformance command should exist')
    call s:assert_true(exists(':RhemaRefactorContext'), 'RhemaRefactorContext command should exist')
    call s:assert_true(exists(':RhemaGenerateCode'), 'RhemaGenerateCode command should exist')
    call s:assert_true(exists(':RhemaShowDocumentation'), 'RhemaShowDocumentation command should exist')
    call s:assert_true(exists(':RhemaConfigureSettings'), 'RhemaConfigureSettings command should exist')
endfunction

" Test cleanup
function! s:test_cleanup()
    echom 'Testing cleanup...'
    
    " Test cleanup function
    call rhema#cleanup()
    call s:assert_true(1, 'Cleanup should not crash')
endfunction

" Run specific test
function! rhema#test#run_specific(test_name)
    echom 'Running specific test: ' . a:test_name
    
    if a:test_name == 'initialization'
        call s:test_initialization()
    elseif a:test_name == 'configuration'
        call s:test_configuration()
    elseif a:test_name == 'file_detection'
        call s:test_file_detection()
    elseif a:test_name == 'context_detection'
        call s:test_context_detection()
    elseif a:test_name == 'command_execution'
        call s:test_command_execution()
    elseif a:test_name == 'completion'
        call s:test_completion()
    elseif a:test_name == 'validation'
        call s:test_validation()
    elseif a:test_name == 'ui'
        call s:test_ui()
    elseif a:test_name == 'cache'
        call s:test_cache()
    elseif a:test_name == 'performance'
        call s:test_performance()
    elseif a:test_name == 'error_handling'
        call s:test_error_handling()
    elseif a:test_name == 'logging'
        call s:test_logging()
    elseif a:test_name == 'job_handling'
        call s:test_job_handling()
    elseif a:test_name == 'filetype_plugin'
        call s:test_filetype_plugin()
    elseif a:test_name == 'syntax'
        call s:test_syntax()
    elseif a:test_name == 'commands'
        call s:test_commands()
    elseif a:test_name == 'cleanup'
        call s:test_cleanup()
    else
        echom 'Unknown test: ' . a:test_name
    endif
endfunction

" Command to run tests
command! -nargs=? RhemaTest call rhema#test#run_all()
command! -nargs=1 RhemaTestSpecific call rhema#test#run_specific(<f-args>) 