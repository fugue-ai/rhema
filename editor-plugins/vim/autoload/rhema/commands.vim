" Rhema Enhanced Command System
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

" Interactive command execution with better UX
function! rhema#commands#execute_interactive(command, args = [])
    call rhema#performance#start('interactive_command')
    
    " Build command with arguments
    let full_args = [a:command] + a:args
    let output = rhema#command#execute(full_args)
    
    call rhema#performance#end('interactive_command')
    
    if !empty(output)
        call rhema#ui#show_output(output, 'Rhema ' . a:command)
        return 1
    else
        call rhema#log#error('Command failed: ' . a:command)
        return 0
    endif
endfunction

" Initialize Rhema scope with interactive prompts
function! rhema#commands#init_scope()
    call rhema#log#message('Initializing Rhema scope...')
    
    let scope_type = input('Select scope type (service/app/library/tool/other): ')
    if empty(scope_type)
        call rhema#log#warning('Scope initialization cancelled')
        return 0
    endif

    let scope_name = input('Enter scope name: ')
    if empty(scope_name)
        call rhema#log#warning('Scope initialization cancelled')
        return 0
    endif

    let auto_config = input('Auto-detect configuration? (y/n): ')
    let args = ['init', '--scope-type', scope_type, '--scope-name', scope_name]
    if auto_config =~? '^y'
        call add(args, '--auto-config')
    endif

    return rhema#commands#execute_interactive('init', args[1:])
endfunction

" Show context with file selection
function! rhema#commands#show_context()
    call rhema#log#message('Showing Rhema context...')
    
    let file = input('Enter file name (without .yaml extension): ')
    if empty(file)
        return 0
    endif

    let scope = input('Enter scope path (optional): ')
    let args = ['show', file]
    if !empty(scope)
        call add(args, '--scope')
        call add(args, scope)
    endif

    return rhema#commands#execute_interactive('show', args[1:])
endfunction

" Execute query with format selection
function! rhema#commands#execute_query()
    call rhema#log#message('Executing Rhema query...')
    
    let query = input('Enter CQL query: ')
    if empty(query)
        return 0
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

    return rhema#commands#execute_interactive('query', args[1:])
endfunction

" Search context with options
function! rhema#commands#search_context()
    call rhema#log#message('Searching Rhema context...')
    
    let term = input('Enter search term: ')
    if empty(term)
        return 0
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

    return rhema#commands#execute_interactive('search', args[1:])
endfunction

" Validate files with options
function! rhema#commands#validate_files()
    call rhema#log#message('Validating Rhema files...')
    
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

    return rhema#commands#execute_interactive('validate', args[1:])
endfunction

" Show scopes
function! rhema#commands#show_scopes()
    return rhema#commands#execute_interactive('scopes')
endfunction

" Show tree
function! rhema#commands#show_tree()
    return rhema#commands#execute_interactive('tree')
endfunction

" Manage todos with subcommands
function! rhema#commands#manage_todos()
    call rhema#log#message('Managing Rhema todos...')
    
    let action = input('Select action (add/list/complete/update/delete): ')
    if empty(action)
        return 0
    endif

    if action == 'add'
        return rhema#commands#add_todo()
    elseif action == 'list'
        return rhema#commands#list_todos()
    elseif action == 'complete'
        return rhema#commands#complete_todo()
    elseif action == 'update'
        return rhema#commands#update_todo()
    elseif action == 'delete'
        return rhema#commands#delete_todo()
    else
        call rhema#log#error('Invalid action: ' . action)
        return 0
    endif
endfunction

" Add todo with all fields
function! rhema#commands#add_todo()
    let title = input('Enter todo title: ')
    if empty(title)
        return 0
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

    return rhema#commands#execute_interactive('todo', args[1:])
endfunction

" List todos with filters
function! rhema#commands#list_todos()
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

    return rhema#commands#execute_interactive('todo', args[1:])
endfunction

" Complete todo
function! rhema#commands#complete_todo()
    let id = input('Enter todo ID: ')
    if empty(id)
        return 0
    endif

    let outcome = input('Enter completion outcome (optional): ')
    let args = ['todo', 'complete', id]
    if !empty(outcome)
        call add(args, '--outcome')
        call add(args, outcome)
    endif

    return rhema#commands#execute_interactive('todo', args[1:])
endfunction

" Update todo
function! rhema#commands#update_todo()
    let id = input('Enter todo ID: ')
    if empty(id)
        return 0
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

    return rhema#commands#execute_interactive('todo', args[1:])
endfunction

" Delete todo
function! rhema#commands#delete_todo()
    let id = input('Enter todo ID: ')
    if empty(id)
        return 0
    endif

    return rhema#commands#execute_interactive('todo', ['delete', id])
endfunction

" Manage insights with subcommands
function! rhema#commands#manage_insights()
    call rhema#log#message('Managing Rhema insights...')
    
    let action = input('Select action (record/list/update/delete): ')
    if empty(action)
        return 0
    endif

    if action == 'record'
        return rhema#commands#record_insight()
    elseif action == 'list'
        return rhema#commands#list_insights()
    elseif action == 'update'
        return rhema#commands#update_insight()
    elseif action == 'delete'
        return rhema#commands#delete_insight()
    else
        call rhema#log#error('Invalid action: ' . action)
        return 0
    endif
endfunction

" Record insight
function! rhema#commands#record_insight()
    let title = input('Enter insight title: ')
    if empty(title)
        return 0
    endif

    let content = input('Enter insight content: ')
    if empty(content)
        return 0
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

    return rhema#commands#execute_interactive('insight', args[1:])
endfunction

" List insights
function! rhema#commands#list_insights()
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

    return rhema#commands#execute_interactive('insight', args[1:])
endfunction

" Update insight
function! rhema#commands#update_insight()
    let id = input('Enter insight ID: ')
    if empty(id)
        return 0
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

    return rhema#commands#execute_interactive('insight', args[1:])
endfunction

" Delete insight
function! rhema#commands#delete_insight()
    let id = input('Enter insight ID: ')
    if empty(id)
        return 0
    endif

    return rhema#commands#execute_interactive('insight', ['delete', id])
endfunction

" Manage patterns with subcommands
function! rhema#commands#manage_patterns()
    call rhema#log#message('Managing Rhema patterns...')
    
    let action = input('Select action (add/list/update/delete): ')
    if empty(action)
        return 0
    endif

    if action == 'add'
        return rhema#commands#add_pattern()
    elseif action == 'list'
        return rhema#commands#list_patterns()
    elseif action == 'update'
        return rhema#commands#update_pattern()
    elseif action == 'delete'
        return rhema#commands#delete_pattern()
    else
        call rhema#log#error('Invalid action: ' . action)
        return 0
    endif
endfunction

" Add pattern
function! rhema#commands#add_pattern()
    let name = input('Enter pattern name: ')
    if empty(name)
        return 0
    endif

    let description = input('Enter pattern description: ')
    if empty(description)
        return 0
    endif

    let pattern_type = input('Enter pattern type: ')
    if empty(pattern_type)
        return 0
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

    return rhema#commands#execute_interactive('pattern', args[1:])
endfunction

" List patterns
function! rhema#commands#list_patterns()
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

    return rhema#commands#execute_interactive('pattern', args[1:])
endfunction

" Update pattern
function! rhema#commands#update_pattern()
    let id = input('Enter pattern ID: ')
    if empty(id)
        return 0
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

    return rhema#commands#execute_interactive('pattern', args[1:])
endfunction

" Delete pattern
function! rhema#commands#delete_pattern()
    let id = input('Enter pattern ID: ')
    if empty(id)
        return 0
    endif

    return rhema#commands#execute_interactive('pattern', ['delete', id])
endfunction

" Manage decisions with subcommands
function! rhema#commands#manage_decisions()
    call rhema#log#message('Managing Rhema decisions...')
    
    let action = input('Select action (record/list/update/delete): ')
    if empty(action)
        return 0
    endif

    if action == 'record'
        return rhema#commands#record_decision()
    elseif action == 'list'
        return rhema#commands#list_decisions()
    elseif action == 'update'
        return rhema#commands#update_decision()
    elseif action == 'delete'
        return rhema#commands#delete_decision()
    else
        call rhema#log#error('Invalid action: ' . action)
        return 0
    endif
endfunction

" Record decision
function! rhema#commands#record_decision()
    let title = input('Enter decision title: ')
    if empty(title)
        return 0
    endif

    let description = input('Enter decision description: ')
    if empty(description)
        return 0
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

    return rhema#commands#execute_interactive('decision', args[1:])
endfunction

" List decisions
function! rhema#commands#list_decisions()
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

    return rhema#commands#execute_interactive('decision', args[1:])
endfunction

" Update decision
function! rhema#commands#update_decision()
    let id = input('Enter decision ID: ')
    if empty(id)
        return 0
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

    return rhema#commands#execute_interactive('decision', args[1:])
endfunction

" Delete decision
function! rhema#commands#delete_decision()
    let id = input('Enter decision ID: ')
    if empty(id)
        return 0
    endif

    return rhema#commands#execute_interactive('decision', ['delete', id])
endfunction

" Show dependencies
function! rhema#commands#show_dependencies()
    return rhema#commands#execute_interactive('dependencies')
endfunction

" Show impact
function! rhema#commands#show_impact()
    call rhema#log#message('Showing Rhema impact...')
    
    let file = input('Enter file to analyze: ')
    if empty(file)
        return 0
    endif

    return rhema#commands#execute_interactive('impact', [file])
endfunction

" Sync knowledge
function! rhema#commands#sync_knowledge()
    return rhema#commands#execute_interactive('sync-knowledge')
endfunction

" Git integration
function! rhema#commands#git_integration()
    call rhema#log#message('Executing Rhema Git integration...')
    
    let action = input('Select Git integration action (hooks/workflow/automation/monitoring/security): ')
    if empty(action)
        return 0
    endif

    return rhema#commands#execute_interactive('git', [action])
endfunction

" Show stats
function! rhema#commands#show_stats()
    return rhema#commands#execute_interactive('stats')
endfunction

" Check health
function! rhema#commands#check_health()
    call rhema#log#message('Checking Rhema health...')
    
    let scope = input('Enter specific scope to check (optional): ')
    let args = ['health']
    if !empty(scope)
        call add(args, '--scope')
        call add(args, scope)
    endif

    return rhema#commands#execute_interactive('health', args[1:])
endfunction

" Debug context
function! rhema#commands#debug_context()
    return rhema#commands#execute_interactive('debug')
endfunction

" Profile performance
function! rhema#commands#profile_performance()
    return rhema#commands#execute_interactive('profile')
endfunction

" Refactor context
function! rhema#commands#refactor_context()
    return rhema#commands#execute_interactive('refactor')
endfunction

" Generate code
function! rhema#commands#generate_code()
    return rhema#commands#execute_interactive('generate')
endfunction

" Show documentation
function! rhema#commands#show_documentation()
    return rhema#commands#execute_interactive('docs')
endfunction 