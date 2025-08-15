" Rhema Completion System
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

" Rhema keywords for completion
let s:rhema_keywords = [
    \ 'context', 'scope', 'todos', 'insights', 'patterns', 'decisions', 'conventions',
    \ 'knowledge', 'dependencies', 'impact', 'analysis',
    \ 'service', 'app', 'library', 'tool', 'other',
    \ 'pending', 'in-progress', 'completed', 'cancelled',
    \ 'low', 'medium', 'high',
    \ 'proposed', 'approved', 'rejected', 'implemented', 'deprecated',
    \ 'recommended', 'required', 'optional', 'deprecated',
    \ 'query', 'validate', 'sync', 'generate', 'refactor',
    \ 'add', 'list', 'update', 'delete', 'complete', 'record',
    \ 'hooks', 'workflow', 'automation', 'monitoring', 'security',
    \ 'health', 'debug', 'profile', 'stats', 'documentation'
    \ ]

" Rhema commands for completion
let s:rhema_commands = [
    \ 'init', 'show', 'search', 'validate', 'scopes', 'tree',
    \ 'todo', 'insight', 'pattern', 'decision',
    \ 'dependencies', 'impact', 'sync-knowledge', 'git',
    \ 'stats', 'health', 'debug', 'profile', 'refactor', 'generate', 'docs'
    \ ]

" Rhema data types for completion
let s:rhema_types = [
    \ 'string', 'integer', 'float', 'boolean', 'array', 'object',
    \ 'yaml', 'json', 'table', 'count',
    \ 'scope-type', 'scope-name', 'auto-config',
    \ 'title', 'description', 'priority', 'assignee', 'due-date',
    \ 'status', 'outcome', 'confidence', 'category', 'tags',
    \ 'pattern-type', 'usage', 'effectiveness', 'examples', 'anti-patterns',
    \ 'context', 'makers', 'alternatives', 'rationale', 'consequences'
    \ ]

" Rhema configuration options for completion
let s:rhema_config = [
    \ 'recursive', 'json-schema', 'migrate',
    \ 'format', 'stats', 'in-file', 'regex',
    \ '--scope', '--format', '--stats', '--recursive',
    \ '--json-schema', '--migrate', '--in-file', '--regex'
    \ ]

" Rhema file patterns for completion
let s:rhema_files = [
    \ 'scope.yaml', 'knowledge.yaml', 'todos.yaml', 'decisions.yaml',
    \ 'patterns.yaml', 'conventions.yaml', 'rhema.yml'
    \ ]

" Omni-completion function
function! rhema#complete#omni(findstart, base)
    if a:findstart
        " Find the start of the word
        let line = getline('.')
        let start = col('.') - 1
        while start > 0 && line[start - 1] =~ '\w'
            let start -= 1
        endwhile
        return start
    else
        " Return completion list
        let completions = []
        let base = a:base
        
        " Get context from current line
        let line = getline('.')
        let col = col('.')
        let before_cursor = line[:col-2]
        
        " Determine completion type based on context
        if before_cursor =~ '^\s*[a-zA-Z_][a-zA-Z0-9_]*:\s*$'
            " After a key, suggest values
            let key = matchstr(before_cursor, '^\s*\([a-zA-Z_][a-zA-Z0-9_]*\):\s*$', '\1')
            call extend(completions, rhema#complete#get_values_for_key(key))
        elseif before_cursor =~ '^\s*-\s*[a-zA-Z_][a-zA-Z0-9_]*:\s*$'
            " After a list item key, suggest values
            let key = matchstr(before_cursor, '^\s*-\s*\([a-zA-Z_][a-zA-Z0-9_]*\):\s*$', '\1')
            call extend(completions, rhema#complete#get_values_for_key(key))
        elseif before_cursor =~ '^\s*$'
            " At start of line, suggest keys
            call extend(completions, rhema#complete#get_keys_for_context())
        elseif before_cursor =~ '^\s*[a-zA-Z_][a-zA-Z0-9_]*\s*$'
            " After a word, suggest commands or keywords
            call extend(completions, rhema#complete#get_commands_and_keywords())
        else
            " Default completion
            call extend(completions, rhema#complete#get_all_completions())
        endif
        
        " Filter by base
        let filtered = []
        for completion in completions
            if completion.word =~ '^' . base
                call add(filtered, completion)
            endif
        endfor
        
        return filtered
    endif
endfunction

" Get values for a specific key
function! rhema#complete#get_values_for_key(key)
    let values = []
    
    if a:key == 'status'
        let values = ['pending', 'in-progress', 'completed', 'cancelled']
    elseif a:key == 'priority'
        let values = ['low', 'medium', 'high']
    elseif a:key == 'scope-type'
        let values = ['service', 'app', 'library', 'tool', 'other']
    elseif a:key == 'decision-status'
        let values = ['proposed', 'approved', 'rejected', 'implemented', 'deprecated']
    elseif a:key == 'usage'
        let values = ['recommended', 'required', 'optional', 'deprecated']
    elseif a:key == 'format'
        let values = ['yaml', 'json', 'table', 'count']
    elseif a:key == 'type'
        let values = ['string', 'integer', 'float', 'boolean', 'array', 'object']
    else
        " Default string value
        let values = ['""']
    endif
    
    let completions = []
    for value in values
        call add(completions, {
            \ 'word': value,
            \ 'kind': 'v',
            \ 'menu': 'Rhema Value'
            \ })
    endfor
    
    return completions
endfunction

" Get keys for current context
function! rhema#complete#get_keys_for_context()
    let keys = []
    let file_type = fnamemodify(expand('%:t'), ':r')
    
    if file_type == 'todos'
        let keys = ['title', 'description', 'status', 'priority', 'assignee', 'due-date', 'tags']
    elseif file_type == 'insights'
        let keys = ['title', 'content', 'confidence', 'category', 'tags', 'context']
    elseif file_type == 'patterns'
        let keys = ['name', 'description', 'pattern-type', 'usage', 'effectiveness', 'examples', 'anti-patterns']
    elseif file_type == 'decisions'
        let keys = ['title', 'description', 'status', 'context', 'makers', 'alternatives', 'rationale', 'consequences']
    elseif file_type == 'conventions'
        let keys = ['name', 'description', 'scope', 'rules', 'examples']
    elseif file_type == 'scope'
        let keys = ['name', 'type', 'description', 'parent', 'dependencies', 'todos', 'insights', 'patterns', 'decisions']
    elseif file_type == 'knowledge'
        let keys = ['title', 'content', 'category', 'tags', 'references', 'last-updated']
    else
        " Default keys
        let keys = ['title', 'description', 'type', 'status', 'tags']
    endif
    
    let completions = []
    for key in keys
        call add(completions, {
            \ 'word': key . ':',
            \ 'kind': 'k',
            \ 'menu': 'Rhema Key'
            \ })
    endfor
    
    return completions
endfunction

" Get commands and keywords
function! rhema#complete#get_commands_and_keywords()
    let completions = []
    
    " Add commands
    for cmd in s:rhema_commands
        call add(completions, {
            \ 'word': cmd,
            \ 'kind': 'c',
            \ 'menu': 'Rhema Command'
            \ })
    endfor
    
    " Add keywords
    for keyword in s:rhema_keywords
        call add(completions, {
            \ 'word': keyword,
            \ 'kind': 'k',
            \ 'menu': 'Rhema Keyword'
            \ })
    endfor
    
    return completions
endfunction

" Get all completions
function! rhema#complete#get_all_completions()
    let completions = []
    
    " Add all types of completions
    call extend(completions, rhema#complete#get_commands_and_keywords())
    
    " Add types
    for type in s:rhema_types
        call add(completions, {
            \ 'word': type,
            \ 'kind': 't',
            \ 'menu': 'Rhema Type'
            \ })
    endfor
    
    " Add config options
    for config in s:rhema_config
        call add(completions, {
            \ 'word': config,
            \ 'kind': 'o',
            \ 'menu': 'Rhema Config'
            \ })
    endfor
    
    " Add file names
    for file in s:rhema_files
        call add(completions, {
            \ 'word': file,
            \ 'kind': 'f',
            \ 'menu': 'Rhema File'
            \ })
    endfor
    
    return completions
endfunction

" Command line completion for Rhema commands
function! rhema#complete#command_line(ArgLead, CmdLine, CursorPos)
    let completions = []
    
    " Add basic commands
    let basic_commands = [
        \ 'init', 'show', 'search', 'validate', 'scopes', 'tree',
        \ 'todo', 'insight', 'pattern', 'decision',
        \ 'dependencies', 'impact', 'sync-knowledge', 'git',
        \ 'stats', 'health', 'debug', 'profile', 'refactor', 'generate', 'docs'
        \ ]
    
    for cmd in basic_commands
        if cmd =~ '^' . a:ArgLead
            call add(completions, cmd)
        endif
    endfor
    
    return completions
endfunction

" File completion for Rhema files
function! rhema#complete#file_completion(ArgLead, CmdLine, CursorPos)
    let completions = []
    
    " Get current directory
    let current_dir = expand('%:p:h')
    
    " Look for Rhema files in current directory and subdirectories
    let files = split(globpath(current_dir, '**/*.yaml'), '\n')
    call extend(files, split(globpath(current_dir, '**/*.yml'), '\n'))
    
    for file in files
        let filename = fnamemodify(file, ':t')
        if filename =~ '^' . a:ArgLead && rhema#filetype#is_rhema_file_pattern(filename)
            call add(completions, filename)
        endif
    endfor
    
    return completions
endfunction

" Check if filename matches Rhema file pattern
function! rhema#filetype#is_rhema_file_pattern(filename)
    return a:filename =~ '\.rhema\.' || 
           a:filename =~ 'scope\.yaml' || 
           a:filename =~ 'knowledge\.yaml' ||
           a:filename =~ 'todos\.yaml' ||
           a:filename =~ 'decisions\.yaml' ||
           a:filename =~ 'patterns\.yaml' ||
           a:filename =~ 'conventions\.yaml' ||
           a:filename == 'rhema.yml'
endfunction

" Template completion
function! rhema#complete#template_completion(template_type)
    let templates = {
        \ 'todo': [
            \ 'title: "TODO: "',
            \ 'description: "Description of the todo"',
            \ 'status: pending',
            \ 'priority: medium',
            \ 'assignee: ""',
            \ 'due-date: ""',
            \ 'tags: []'
            \ ],
        \ 'insight': [
            \ 'title: "Insight: "',
            \ 'content: "Content of the insight"',
            \ 'confidence: 5',
            \ 'category: ""',
            \ 'tags: []',
            \ 'context: ""'
            \ ],
        \ 'pattern': [
            \ 'name: "Pattern Name"',
            \ 'description: "Description of the pattern"',
            \ 'pattern-type: ""',
            \ 'usage: recommended',
            \ 'effectiveness: 7',
            \ 'examples: []',
            \ 'anti-patterns: []'
            \ ],
        \ 'decision': [
            \ 'title: "Decision: "',
            \ 'description: "Description of the decision"',
            \ 'status: proposed',
            \ 'context: ""',
            \ 'makers: []',
            \ 'alternatives: []',
            \ 'rationale: ""',
            \ 'consequences: []'
            \ ]
        \ }
    
    if has_key(templates, a:template_type)
        return templates[a:template_type]
    else
        return []
    endif
endfunction

" Insert template at cursor
function! rhema#complete#insert_template(template_type)
    let template = rhema#complete#template_completion(a:template_type)
    if !empty(template)
        let lines = []
        for line in template
            call add(lines, '  ' . line)
        endfor
        
        " Insert template at cursor
        call append(line('.'), lines)
        call rhema#log#info('Inserted ' . a:template_type . ' template')
    endif
endfunction 