" Rhema - Git-Based Agent Context Protocol Vim Integration
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

" Plugin configuration
let s:plugin_name = 'rhema'
let s:plugin_version = '0.1.0'

" Check if plugin is already loaded
if exists('g:loaded_rhema')
    finish
endif

" Set plugin as loaded
let g:loaded_rhema = 1

" Default configuration
if !exists('g:rhema_enabled')
    let g:rhema_enabled = 1
endif

if !exists('g:rhema_executable')
    let g:rhema_executable = 'rhema'
endif

if !exists('g:rhema_auto_validate')
    let g:rhema_auto_validate = 1
endif

if !exists('g:rhema_show_notifications')
    let g:rhema_show_notifications = 1
endif

if !exists('g:rhema_intellisense')
    let g:rhema_intellisense = 1
endif

if !exists('g:rhema_debug_mode')
    let g:rhema_debug_mode = 0
endif

if !exists('g:rhema_performance_profiling')
    let g:rhema_performance_profiling = 0
endif

if !exists('g:rhema_context_exploration')
    let g:rhema_context_exploration = 1
endif

if !exists('g:rhema_git_integration')
    let g:rhema_git_integration = 1
endif

if !exists('g:rhema_auto_sync')
    let g:rhema_auto_sync = 0
endif

if !exists('g:rhema_theme')
    let g:rhema_theme = 'auto'
endif

if !exists('g:rhema_language')
    let g:rhema_language = 'en'
endif

" Plugin state
let s:rhema_initialized = 0
let s:rhema_buffer = -1
let s:rhema_window = -1
let s:rhema_last_command = ''
let s:rhema_last_output = ''
let s:rhema_error_count = 0
let s:rhema_warning_count = 0

" Utility functions
function! s:Log(message, level = 'info')
    if g:rhema_debug_mode || a:level == 'error'
        echom '[RHEMA] ' . a:message
    endif
endfunction

function! s:Error(message)
    call s:Log(a:message, 'error')
    echohl ErrorMsg
    echom '[RHEMA] Error: ' . a:message
    echohl None
endfunction

function! s:Warning(message)
    call s:Log(a:message, 'warning')
    echohl WarningMsg
    echom '[RHEMA] Warning: ' . a:message
    echohl None
endfunction

function! s:Info(message)
    if g:rhema_show_notifications
        echom '[RHEMA] ' . a:message
    endif
endfunction

function! s:IsRhemaFile()
    let filename = expand('%:t')
    return filename =~ '\.rhema\.' || 
           filename =~ 'scope\.yaml' || 
           filename =~ 'knowledge\.yaml' ||
           filename =~ 'todos\.yaml' ||
           filename =~ 'decisions\.yaml' ||
           filename =~ 'patterns\.yaml' ||
           filename =~ 'conventions\.yaml'
endfunction

function! s:ExecuteCommand(args, callback = '')
    if !executable(g:rhema_executable)
        call s:Error('Rhema executable not found: ' . g:rhema_executable)
        return ''
    endif

    let cmd = g:rhema_executable . ' ' . join(a:args, ' ')
    let s:rhema_last_command = cmd

    call s:Log('Executing: ' . cmd)

    if has('nvim')
        " Neovim async execution
        let job_id = jobstart(cmd, {
            \ 'on_stdout': function('s:HandleJobOutput'),
            \ 'on_stderr': function('s:HandleJobError'),
            \ 'on_exit': function('s:HandleJobExit')
            \ })
        return job_id
    else
        " Vim synchronous execution
        let output = system(cmd)
        let s:rhema_last_output = output
        if v:shell_error
            call s:Error('Command failed: ' . output)
            return ''
        else
            call s:Log('Command succeeded: ' . output)
            if !empty(a:callback)
                call call(a:callback, [output])
            endif
            return output
        endif
    endif
endfunction

" Job handling for Neovim
function! s:HandleJobOutput(job_id, data, event)
    let s:rhema_last_output = join(a:data, "\n")
    call s:Log('Job output: ' . s:rhema_last_output)
endfunction

function! s:HandleJobError(job_id, data, event)
    let error = join(a:data, "\n")
    call s:Error('Job error: ' . error)
endfunction

function! s:HandleJobExit(job_id, data, event)
    if a:data == 0
        call s:Log('Job completed successfully')
    else
        call s:Error('Job failed with exit code: ' . a:data)
    endif
endfunction

" Main Rhema functions
function! rhema#Initialize()
    call s:Log('Initializing RHEMA...')
    
    let scope_type = input('Select scope type (service/app/library/tool/other): ')
    if empty(scope_type)
        return
    endif

    let scope_name = input('Enter scope name: ')
    if empty(scope_name)
        return
    endif

    let auto_config = input('Auto-detect configuration? (y/n): ')
    let args = ['init', '--scope-type', scope_type, '--scope-name', scope_name]
    if auto_config =~? '^y'
        call add(args, '--auto-config')
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Rhema scope initialized successfully')
    endif
endfunction

function! rhema#ShowContext()
    call s:Log('Showing Rhema context...')
    
    let file = input('Enter file name (without .yaml extension): ')
    if empty(file)
        return
    endif

    let scope = input('Enter scope path (optional): ')
    let args = ['show', file]
    if !empty(scope)
        call add(args, '--scope')
        call add(args, scope)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Context')
    endif
endfunction

function! rhema#ExecuteQuery()
    call s:Log('Executing Rhema query...')
    
    let query = input('Enter CQL query: ')
    if empty(query)
        return
    endif

    let format = input('Select output format (yaml/json/table/count): ')
    if empty(format)
        let format = 'yaml'
    endif

    let include_stats = input('Include query statistics? (y/n): ')
    let args = ['query', query, '--format', format]
    if include_stats =~? '^y'
        call add(args, '--stats')
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Query Result')
    endif
endfunction

function! rhema#SearchContext()
    call s:Log('Searching Rhema context...')
    
    let term = input('Enter search term: ')
    if empty(term)
        return
    endif

    let in_file = input('Search in specific file type (optional): ')
    let use_regex = input('Use regex pattern? (y/n): ')
    
    let args = ['search', term]
    if !empty(in_file)
        call add(args, '--in-file')
        call add(args, in_file)
    endif
    if use_regex =~? '^y'
        call add(args, '--regex')
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Search Results')
    endif
endfunction

function! rhema#ValidateFiles()
    call s:Log('Validating Rhema files...')
    
    let recursive = input('Validate recursively? (y/n): ')
    let show_schema = input('Show JSON schemas? (y/n): ')
    let migrate = input('Migrate schemas to latest version? (y/n): ')
    
    let args = ['validate']
    if recursive =~? '^y'
        call add(args, '--recursive')
    endif
    if show_schema =~? '^y'
        call add(args, '--json-schema')
    endif
    if migrate =~? '^y'
        call add(args, '--migrate')
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Validation Results')
    endif
endfunction

function! rhema#ShowScopes()
    call s:Log('Showing Rhema scopes...')
    
    let output = s:ExecuteCommand(['scopes'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Scopes')
    endif
endfunction

function! rhema#ShowTree()
    call s:Log('Showing Rhema scope tree...')
    
    let output = s:ExecuteCommand(['tree'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Scope Tree')
    endif
endfunction

function! rhema#ManageTodos()
    call s:Log('Managing Rhema todos...')
    
    let action = input('Select action (add/list/complete/update/delete): ')
    if empty(action)
        return
    endif

    if action == 'add'
        call rhema#AddTodo()
    elseif action == 'list'
        call rhema#ListTodos()
    elseif action == 'complete'
        call rhema#CompleteTodo()
    elseif action == 'update'
        call rhema#UpdateTodo()
    elseif action == 'delete'
        call rhema#DeleteTodo()
    else
        call s:Error('Invalid action: ' . action)
    endif
endfunction

function! rhema#AddTodo()
    let title = input('Enter todo title: ')
    if empty(title)
        return
    endif

    let description = input('Enter todo description (optional): ')
    let priority = input('Select priority (low/medium/high): ')
    let assignee = input('Enter assignee (optional): ')
    let due_date = input('Enter due date in ISO format (optional): ')
    
    let args = ['todo', 'add', title]
    if !empty(description)
        call add(args, '--description')
        call add(args, description)
    endif
    if !empty(priority)
        call add(args, '--priority')
        call add(args, priority)
    endif
    if !empty(assignee)
        call add(args, '--assignee')
        call add(args, assignee)
    endif
    if !empty(due_date)
        call add(args, '--due-date')
        call add(args, due_date)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Todo added successfully')
    endif
endfunction

function! rhema#ListTodos()
    let status = input('Filter by status (pending/in-progress/completed/cancelled, optional): ')
    let priority = input('Filter by priority (low/medium/high, optional): ')
    let assignee = input('Filter by assignee (optional): ')
    
    let args = ['todo', 'list']
    if !empty(status)
        call add(args, '--status')
        call add(args, status)
    endif
    if !empty(priority)
        call add(args, '--priority')
        call add(args, priority)
    endif
    if !empty(assignee)
        call add(args, '--assignee')
        call add(args, assignee)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Todos')
    endif
endfunction

function! rhema#CompleteTodo()
    let id = input('Enter todo ID: ')
    if empty(id)
        return
    endif

    let outcome = input('Enter completion outcome (optional): ')
    let args = ['todo', 'complete', id]
    if !empty(outcome)
        call add(args, '--outcome')
        call add(args, outcome)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Todo completed successfully')
    endif
endfunction

function! rhema#UpdateTodo()
    let id = input('Enter todo ID: ')
    if empty(id)
        return
    endif

    let title = input('Enter new title (optional): ')
    let description = input('Enter new description (optional): ')
    let status = input('Enter new status (pending/in-progress/completed/cancelled, optional): ')
    let priority = input('Enter new priority (low/medium/high, optional): ')
    let assignee = input('Enter new assignee (optional): ')
    let due_date = input('Enter new due date in ISO format (optional): ')
    
    let args = ['todo', 'update', id]
    if !empty(title)
        call add(args, '--title')
        call add(args, title)
    endif
    if !empty(description)
        call add(args, '--description')
        call add(args, description)
    endif
    if !empty(status)
        call add(args, '--status')
        call add(args, status)
    endif
    if !empty(priority)
        call add(args, '--priority')
        call add(args, priority)
    endif
    if !empty(assignee)
        call add(args, '--assignee')
        call add(args, assignee)
    endif
    if !empty(due_date)
        call add(args, '--due-date')
        call add(args, due_date)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Todo updated successfully')
    endif
endfunction

function! rhema#DeleteTodo()
    let id = input('Enter todo ID: ')
    if empty(id)
        return
    endif

    let output = s:ExecuteCommand(['todo', 'delete', id])
    if !empty(output)
        call s:Info('Todo deleted successfully')
    endif
endfunction

function! rhema#ManageInsights()
    call s:Log('Managing Rhema insights...')
    
    let action = input('Select action (record/list/update/delete): ')
    if empty(action)
        return
    endif

    if action == 'record'
        call rhema#RecordInsight()
    elseif action == 'list'
        call rhema#ListInsights()
    elseif action == 'update'
        call rhema#UpdateInsight()
    elseif action == 'delete'
        call rhema#DeleteInsight()
    else
        call s:Error('Invalid action: ' . action)
    endif
endfunction

function! rhema#RecordInsight()
    let title = input('Enter insight title: ')
    if empty(title)
        return
    endif

    let content = input('Enter insight content: ')
    if empty(content)
        return
    endif

    let confidence = input('Enter confidence level (1-10, optional): ')
    let category = input('Enter category (optional): ')
    let tags = input('Enter tags (comma-separated, optional): ')
    
    let args = ['insight', 'record', title, '--content', content]
    if !empty(confidence)
        call add(args, '--confidence')
        call add(args, confidence)
    endif
    if !empty(category)
        call add(args, '--category')
        call add(args, category)
    endif
    if !empty(tags)
        call add(args, '--tags')
        call add(args, tags)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Insight recorded successfully')
    endif
endfunction

function! rhema#ListInsights()
    let category = input('Filter by category (optional): ')
    let tag = input('Filter by tag (optional): ')
    let min_confidence = input('Minimum confidence level (1-10, optional): ')
    
    let args = ['insight', 'list']
    if !empty(category)
        call add(args, '--category')
        call add(args, category)
    endif
    if !empty(tag)
        call add(args, '--tag')
        call add(args, tag)
    endif
    if !empty(min_confidence)
        call add(args, '--min-confidence')
        call add(args, min_confidence)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Insights')
    endif
endfunction

function! rhema#UpdateInsight()
    let id = input('Enter insight ID: ')
    if empty(id)
        return
    endif

    let title = input('Enter new title (optional): ')
    let content = input('Enter new content (optional): ')
    let confidence = input('Enter new confidence level (1-10, optional): ')
    let category = input('Enter new category (optional): ')
    let tags = input('Enter new tags (comma-separated, optional): ')
    
    let args = ['insight', 'update', id]
    if !empty(title)
        call add(args, '--title')
        call add(args, title)
    endif
    if !empty(content)
        call add(args, '--content')
        call add(args, content)
    endif
    if !empty(confidence)
        call add(args, '--confidence')
        call add(args, confidence)
    endif
    if !empty(category)
        call add(args, '--category')
        call add(args, category)
    endif
    if !empty(tags)
        call add(args, '--tags')
        call add(args, tags)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Insight updated successfully')
    endif
endfunction

function! rhema#DeleteInsight()
    let id = input('Enter insight ID: ')
    if empty(id)
        return
    endif

    let output = s:ExecuteCommand(['insight', 'delete', id])
    if !empty(output)
        call s:Info('Insight deleted successfully')
    endif
endfunction

function! rhema#ManagePatterns()
    call s:Log('Managing Rhema patterns...')
    
    let action = input('Select action (add/list/update/delete): ')
    if empty(action)
        return
    endif

    if action == 'add'
        call rhema#AddPattern()
    elseif action == 'list'
        call rhema#ListPatterns()
    elseif action == 'update'
        call rhema#UpdatePattern()
    elseif action == 'delete'
        call rhema#DeletePattern()
    else
        call s:Error('Invalid action: ' . action)
    endif
endfunction

function! rhema#AddPattern()
    let name = input('Enter pattern name: ')
    if empty(name)
        return
    endif

    let description = input('Enter pattern description: ')
    if empty(description)
        return
    endif

    let pattern_type = input('Enter pattern type: ')
    if empty(pattern_type)
        return
    endif

    let usage = input('Select usage context (recommended/required/optional/deprecated): ')
    let effectiveness = input('Enter effectiveness rating (1-10, optional): ')
    let examples = input('Enter examples (comma-separated, optional): ')
    let anti_patterns = input('Enter anti-patterns to avoid (comma-separated, optional): ')
    
    let args = ['pattern', 'add', name, '--description', description, '--pattern-type', pattern_type]
    if !empty(usage)
        call add(args, '--usage')
        call add(args, usage)
    endif
    if !empty(effectiveness)
        call add(args, '--effectiveness')
        call add(args, effectiveness)
    endif
    if !empty(examples)
        call add(args, '--examples')
        call add(args, examples)
    endif
    if !empty(anti_patterns)
        call add(args, '--anti-patterns')
        call add(args, anti_patterns)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Pattern added successfully')
    endif
endfunction

function! rhema#ListPatterns()
    let pattern_type = input('Filter by pattern type (optional): ')
    let usage = input('Filter by usage context (recommended/required/optional/deprecated, optional): ')
    let min_effectiveness = input('Minimum effectiveness rating (1-10, optional): ')
    
    let args = ['pattern', 'list']
    if !empty(pattern_type)
        call add(args, '--pattern-type')
        call add(args, pattern_type)
    endif
    if !empty(usage)
        call add(args, '--usage')
        call add(args, usage)
    endif
    if !empty(min_effectiveness)
        call add(args, '--min-effectiveness')
        call add(args, min_effectiveness)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Patterns')
    endif
endfunction

function! rhema#UpdatePattern()
    let id = input('Enter pattern ID: ')
    if empty(id)
        return
    endif

    let name = input('Enter new name (optional): ')
    let description = input('Enter new description (optional): ')
    let pattern_type = input('Enter new pattern type (optional): ')
    let usage = input('Enter new usage context (recommended/required/optional/deprecated, optional): ')
    let effectiveness = input('Enter new effectiveness rating (1-10, optional): ')
    let examples = input('Enter new examples (comma-separated, optional): ')
    let anti_patterns = input('Enter new anti-patterns (comma-separated, optional): ')
    
    let args = ['pattern', 'update', id]
    if !empty(name)
        call add(args, '--name')
        call add(args, name)
    endif
    if !empty(description)
        call add(args, '--description')
        call add(args, description)
    endif
    if !empty(pattern_type)
        call add(args, '--pattern-type')
        call add(args, pattern_type)
    endif
    if !empty(usage)
        call add(args, '--usage')
        call add(args, usage)
    endif
    if !empty(effectiveness)
        call add(args, '--effectiveness')
        call add(args, effectiveness)
    endif
    if !empty(examples)
        call add(args, '--examples')
        call add(args, examples)
    endif
    if !empty(anti_patterns)
        call add(args, '--anti-patterns')
        call add(args, anti_patterns)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Pattern updated successfully')
    endif
endfunction

function! rhema#DeletePattern()
    let id = input('Enter pattern ID: ')
    if empty(id)
        return
    endif

    let output = s:ExecuteCommand(['pattern', 'delete', id])
    if !empty(output)
        call s:Info('Pattern deleted successfully')
    endif
endfunction

function! rhema#ManageDecisions()
    call s:Log('Managing Rhema decisions...')
    
    let action = input('Select action (record/list/update/delete): ')
    if empty(action)
        return
    endif

    if action == 'record'
        call rhema#RecordDecision()
    elseif action == 'list'
        call rhema#ListDecisions()
    elseif action == 'update'
        call rhema#UpdateDecision()
    elseif action == 'delete'
        call rhema#DeleteDecision()
    else
        call s:Error('Invalid action: ' . action)
    endif
endfunction

function! rhema#RecordDecision()
    let title = input('Enter decision title: ')
    if empty(title)
        return
    endif

    let description = input('Enter decision description: ')
    if empty(description)
        return
    endif

    let status = input('Select decision status (proposed/approved/rejected/implemented/deprecated): ')
    let context = input('Enter decision context (optional): ')
    let makers = input('Enter decision makers (comma-separated, optional): ')
    let alternatives = input('Enter alternatives considered (comma-separated, optional): ')
    let rationale = input('Enter rationale (optional): ')
    let consequences = input('Enter consequences (comma-separated, optional): ')
    
    let args = ['decision', 'record', title, '--description', description]
    if !empty(status)
        call add(args, '--status')
        call add(args, status)
    endif
    if !empty(context)
        call add(args, '--context')
        call add(args, context)
    endif
    if !empty(makers)
        call add(args, '--makers')
        call add(args, makers)
    endif
    if !empty(alternatives)
        call add(args, '--alternatives')
        call add(args, alternatives)
    endif
    if !empty(rationale)
        call add(args, '--rationale')
        call add(args, rationale)
    endif
    if !empty(consequences)
        call add(args, '--consequences')
        call add(args, consequences)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Decision recorded successfully')
    endif
endfunction

function! rhema#ListDecisions()
    let status = input('Filter by status (proposed/approved/rejected/implemented/deprecated, optional): ')
    let maker = input('Filter by decision maker (optional): ')
    
    let args = ['decision', 'list']
    if !empty(status)
        call add(args, '--status')
        call add(args, status)
    endif
    if !empty(maker)
        call add(args, '--maker')
        call add(args, maker)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Decisions')
    endif
endfunction

function! rhema#UpdateDecision()
    let id = input('Enter decision ID: ')
    if empty(id)
        return
    endif

    let title = input('Enter new title (optional): ')
    let description = input('Enter new description (optional): ')
    let status = input('Enter new status (proposed/approved/rejected/implemented/deprecated, optional): ')
    let context = input('Enter new context (optional): ')
    let makers = input('Enter new decision makers (comma-separated, optional): ')
    let alternatives = input('Enter new alternatives (comma-separated, optional): ')
    let rationale = input('Enter new rationale (optional): ')
    let consequences = input('Enter new consequences (comma-separated, optional): ')
    
    let args = ['decision', 'update', id]
    if !empty(title)
        call add(args, '--title')
        call add(args, title)
    endif
    if !empty(description)
        call add(args, '--description')
        call add(args, description)
    endif
    if !empty(status)
        call add(args, '--status')
        call add(args, status)
    endif
    if !empty(context)
        call add(args, '--context')
        call add(args, context)
    endif
    if !empty(makers)
        call add(args, '--makers')
        call add(args, makers)
    endif
    if !empty(alternatives)
        call add(args, '--alternatives')
        call add(args, alternatives)
    endif
    if !empty(rationale)
        call add(args, '--rationale')
        call add(args, rationale)
    endif
    if !empty(consequences)
        call add(args, '--consequences')
        call add(args, consequences)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:Info('Decision updated successfully')
    endif
endfunction

function! rhema#DeleteDecision()
    let id = input('Enter decision ID: ')
    if empty(id)
        return
    endif

    let output = s:ExecuteCommand(['decision', 'delete', id])
    if !empty(output)
        call s:Info('Decision deleted successfully')
    endif
endfunction

function! rhema#ShowDependencies()
    call s:Log('Showing Rhema dependencies...')
    
    let output = s:ExecuteCommand(['dependencies'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Dependencies')
    endif
endfunction

function! rhema#ShowImpact()
    call s:Log('Showing Rhema impact...')
    
    let file = input('Enter file to analyze: ')
    if empty(file)
        return
    endif

    let output = s:ExecuteCommand(['impact', file])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Impact Analysis')
    endif
endfunction

function! rhema#SyncKnowledge()
    call s:Log('Syncing Rhema knowledge...')
    
    let output = s:ExecuteCommand(['sync-knowledge'])
    if !empty(output)
        call s:Info('Knowledge synced successfully')
    endif
endfunction

function! rhema#GitIntegration()
    call s:Log('Executing Rhema Git integration...')
    
    let action = input('Select Git integration action (hooks/workflow/automation/monitoring/security): ')
    if empty(action)
        return
    endif

    let output = s:ExecuteCommand(['git', action])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Git ' . action)
    endif
endfunction

function! rhema#ShowStats()
    call s:Log('Showing Rhema statistics...')
    
    let output = s:ExecuteCommand(['stats'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Statistics')
    endif
endfunction

function! rhema#CheckHealth()
    call s:Log('Checking Rhema health...')
    
    let scope = input('Enter specific scope to check (optional): ')
    let args = ['health']
    if !empty(scope)
        call add(args, '--scope')
        call add(args, scope)
    endif

    let output = s:ExecuteCommand(args)
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Health Check')
    endif
endfunction

function! rhema#DebugContext()
    call s:Log('Debugging Rhema context...')
    
    let output = s:ExecuteCommand(['debug'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Debug Information')
    endif
endfunction

function! rhema#ProfilePerformance()
    call s:Log('Profiling Rhema performance...')
    
    let output = s:ExecuteCommand(['profile'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Performance Profile')
    endif
endfunction

function! rhema#RefactorContext()
    call s:Log('Refactoring Rhema context...')
    
    let output = s:ExecuteCommand(['refactor'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Refactoring')
    endif
endfunction

function! rhema#GenerateCode()
    call s:Log('Generating code from Rhema context...')
    
    let output = s:ExecuteCommand(['generate'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Code Generation')
    endif
endfunction

function! rhema#ShowDocumentation()
    call s:Log('Showing Rhema documentation...')
    
    let output = s:ExecuteCommand(['docs'])
    if !empty(output)
        call s:ShowOutput(output, 'Rhema Documentation')
    endif
endfunction

function! rhema#ConfigureSettings()
    call s:Log('Opening Rhema settings...')
    
    " Open the plugin configuration
    if exists(':RhemaSettings')
        RhemaSettings
    else
        call s:Info('Rhema settings can be configured via g:rhema_* variables')
        call s:Info('See :help rhema for more information')
    endif
endfunction

function! s:ShowOutput(output, title)
    " Create a new buffer to show the output
    let bufname = '[RHEMA] ' . a:title
    let buf = bufnr(bufname)
    
    if buf == -1
        " Create new buffer
        execute 'new ' . bufname
        let s:rhema_buffer = bufnr('%')
    else
        " Switch to existing buffer
        execute 'buffer ' . buf
        let s:rhema_buffer = buf
    endif
    
    " Clear buffer and insert output
    setlocal modifiable
    silent! %delete _
    call setline(1, split(a:output, '\n'))
    setlocal nomodifiable
    setlocal buftype=nofile
    setlocal bufhidden=hide
    setlocal noswapfile
    setlocal filetype=yaml
    
    " Set syntax highlighting
    if has('syntax')
        syntax on
    endif
    
    " Position cursor at top
    normal! gg
endfunction

" Auto-validation on save
function! s:AutoValidate()
    if g:rhema_auto_validate && s:IsRhemaFile()
        call s:Log('Auto-validating Rhema file...')
        let output = s:ExecuteCommand(['validate'])
        if !empty(output)
            call s:Info('Rhema file validated successfully')
        endif
    endif
endfunction

" Initialize plugin
function! s:Initialize()
    if s:rhema_initialized
        return
    endif
    
    call s:Log('Initializing Rhema plugin...')
    
    " Check if Rhema executable is available
    if !executable(g:rhema_executable)
        call s:Error('Rhema executable not found: ' . g:rhema_executable)
        return
    endif
    
    " Set up autocommands
    augroup RhemaAutoGroup
        autocmd!
        autocmd BufWritePost * call s:AutoValidate()
    augroup END
    
    let s:rhema_initialized = 1
    call s:Info('Rhema plugin initialized successfully')
endfunction

" Plugin initialization
call rhema#init()

" Command definitions
command! -nargs=0 RhemaInitialize call rhema#commands#init_scope()
command! -nargs=0 RhemaShowContext call rhema#commands#show_context()
command! -nargs=0 RhemaExecuteQuery call rhema#commands#execute_query()
command! -nargs=0 RhemaSearchContext call rhema#commands#search_context()
command! -nargs=0 RhemaValidateFiles call rhema#commands#validate_files()
command! -nargs=0 RhemaShowScopes call rhema#commands#show_scopes()
command! -nargs=0 RhemaShowTree call rhema#commands#show_tree()
command! -nargs=0 RhemaManageTodos call rhema#commands#manage_todos()
command! -nargs=0 RhemaManageInsights call rhema#commands#manage_insights()
command! -nargs=0 RhemaManagePatterns call rhema#commands#manage_patterns()
command! -nargs=0 RhemaManageDecisions call rhema#commands#manage_decisions()
command! -nargs=0 RhemaShowDependencies call rhema#commands#show_dependencies()
command! -nargs=0 RhemaShowImpact call rhema#commands#show_impact()
command! -nargs=0 RhemaSyncKnowledge call rhema#commands#sync_knowledge()
command! -nargs=0 RhemaGitIntegration call rhema#commands#git_integration()
command! -nargs=0 RhemaShowStats call rhema#commands#show_stats()
command! -nargs=0 RhemaCheckHealth call rhema#commands#check_health()
command! -nargs=0 RhemaDebugContext call rhema#commands#debug_context()
command! -nargs=0 RhemaProfilePerformance call rhema#commands#profile_performance()
command! -nargs=0 RhemaRefactorContext call rhema#commands#refactor_context()
command! -nargs=0 RhemaGenerateCode call rhema#commands#generate_code()
command! -nargs=0 RhemaShowDocumentation call rhema#commands#show_documentation()
command! -nargs=0 RhemaConfigureSettings call rhema#ui#show_info('Configure via g:rhema_* variables')
command! -nargs=0 RhemaShowSidebar call rhema#ui#show_sidebar()
command! -nargs=0 RhemaStatus call rhema#ui#show_output(string(rhema#status()), 'Plugin Status')
command! -nargs=0 RhemaCacheClear call rhema#cache#clear()
command! -nargs=0 RhemaCacheStats call rhema#ui#show_output(string(rhema#cache#stats()), 'Cache Statistics')

" Key mappings
if !exists('g:rhema_no_mappings') || !g:rhema_no_mappings
    nnoremap <silent> <leader>gi :RhemaInitialize<CR>
    nnoremap <silent> <leader>gc :RhemaShowContext<CR>
    nnoremap <silent> <leader>gq :RhemaExecuteQuery<CR>
    nnoremap <silent> <leader>gs :RhemaSearchContext<CR>
    nnoremap <silent> <leader>gv :RhemaValidateFiles<CR>
    nnoremap <silent> <leader>gp :RhemaShowScopes<CR>
    nnoremap <silent> <leader>gt :RhemaShowTree<CR>
    nnoremap <silent> <leader>gt :RhemaManageTodos<CR>
    nnoremap <silent> <leader>gi :RhemaManageInsights<CR>
    nnoremap <silent> <leader>gp :RhemaManagePatterns<CR>
    nnoremap <silent> <leader>gd :RhemaManageDecisions<CR>
    nnoremap <silent> <leader>gd :RhemaShowDependencies<CR>
    nnoremap <silent> <leader>gi :RhemaShowImpact<CR>
    nnoremap <silent> <leader>gk :RhemaSyncKnowledge<CR>
    nnoremap <silent> <leader>gg :RhemaGitIntegration<CR>
    nnoremap <silent> <leader>gs :RhemaShowStats<CR>
    nnoremap <silent> <leader>gh :RhemaCheckHealth<CR>
    nnoremap <silent> <leader>gb :RhemaDebugContext<CR>
    nnoremap <silent> <leader>gf :RhemaProfilePerformance<CR>
    nnoremap <silent> <leader>gr :RhemaRefactorContext<CR>
    nnoremap <silent> <leader>gc :RhemaGenerateCode<CR>
    nnoremap <silent> <leader>gh :RhemaShowDocumentation<CR>
    nnoremap <silent> <leader>gc :RhemaConfigureSettings<CR>
    nnoremap <silent> <leader>gs :RhemaShowSidebar<CR>
    nnoremap <silent> <leader>gs :RhemaStatus<CR>
    nnoremap <silent> <leader>gc :RhemaCacheClear<CR>
    nnoremap <silent> <leader>gc :RhemaCacheStats<CR>
endif 