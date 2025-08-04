" Rhema Vim Plugin - Main Autoload File
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

" Check if plugin is already loaded
if exists('g:loaded_rhema_autoload')
    finish
endif

" Set plugin as loaded
let g:loaded_rhema_autoload = 1

" Global state
let s:rhema_cache = {}
let s:rhema_context = {}
let s:rhema_last_command = ''
let s:rhema_last_output = ''
let s:rhema_error_count = 0
let s:rhema_warning_count = 0
let s:rhema_performance_stats = {}

" Configuration defaults
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

if !exists('g:rhema_timeout')
    let g:rhema_timeout = 30
endif

if !exists('g:rhema_cache_enabled')
    let g:rhema_cache_enabled = 1
endif

if !exists('g:rhema_cache_ttl')
    let g:rhema_cache_ttl = 300
endif

" Utility functions
function! rhema#log#message(message, level = 'info')
    if g:rhema_debug_mode || a:level == 'error'
        echom '[RHEMA] ' . a:message
    endif
endfunction

function! rhema#log#error(message)
    call rhema#log#message(a:message, 'error')
    echohl ErrorMsg
    echom '[RHEMA] Error: ' . a:message
    echohl None
endfunction

function! rhema#log#warning(message)
    call rhema#log#message(a:message, 'warning')
    echohl WarningMsg
    echom '[RHEMA] Warning: ' . a:message
    echohl None
endfunction

function! rhema#log#info(message)
    if g:rhema_show_notifications
        echom '[RHEMA] ' . a:message
    endif
endfunction

" Cache management
function! rhema#cache#get(key)
    if !g:rhema_cache_enabled
        return ''
    endif
    
    if has_key(s:rhema_cache, a:key)
        let entry = s:rhema_cache[a:key]
        if localtime() - entry.timestamp < g:rhema_cache_ttl
            return entry.value
        else
            " Remove expired entry
            call remove(s:rhema_cache, a:key)
        endif
    endif
    
    return ''
endfunction

function! rhema#cache#set(key, value)
    if !g:rhema_cache_enabled
        return
    endif
    
    let s:rhema_cache[a:key] = {
        \ 'value': a:value,
        \ 'timestamp': localtime()
        \ }
endfunction

function! rhema#cache#clear()
    let s:rhema_cache = {}
    call rhema#log#info('Cache cleared')
endfunction

function! rhema#cache#stats()
    let count = len(s:rhema_cache)
    let size = 0
    for [key, entry] in items(s:rhema_cache)
        let size += len(string(entry.value))
    endfor
    
    return {
        \ 'entries': count,
        \ 'size': size,
        \ 'enabled': g:rhema_cache_enabled,
        \ 'ttl': g:rhema_cache_ttl
        \ }
endfunction

" Performance monitoring
function! rhema#performance#start(label)
    if !g:rhema_performance_profiling
        return
    endif
    
    let s:rhema_performance_stats[a:label] = {
        \ 'start': reltime()
        \ }
endfunction

function! rhema#performance#end(label)
    if !g:rhema_performance_profiling
        return
    endif
    
    if has_key(s:rhema_performance_stats, a:label)
        let entry = s:rhema_performance_stats[a:label]
        let elapsed = reltimefloat(reltime(entry.start))
        let entry.elapsed = elapsed
        call rhema#log#message('Performance [' . a:label . ']: ' . string(elapsed) . 's')
    endif
endfunction

" Context detection
function! rhema#context#detect()
    let current_file = expand('%:p')
    let project_root = fnamemodify(current_file, ':h')
    
    " Look for rhema.yml files in the project
    let rhema_files = []
    let root = project_root
    
    while root != fnamemodify(root, ':h')
        let scope_file = root . '/scope.yaml'
        let rhema_file = root . '/rhema.yml'
        
        if filereadable(scope_file)
            call add(rhema_files, scope_file)
        endif
        
        if filereadable(rhema_file)
            call add(rhema_files, rhema_file)
        endif
        
        let root = fnamemodify(root, ':h')
    endwhile
    
    return rhema_files
endfunction

function! rhema#context#get_current()
    let files = rhema#context#detect()
    if empty(files)
        return {}
    endif
    
    " Use the closest rhema file
    let current_file = expand('%:p')
    let closest_file = ''
    let min_distance = 999999
    
    for file in files
        let distance = len(split(current_file, '/')) - len(split(file, '/'))
        if distance >= 0 && distance < min_distance
            let min_distance = distance
            let closest_file = file
        endif
    endfor
    
    if !empty(closest_file)
        return {
            \ 'file': closest_file,
            \ 'root': fnamemodify(closest_file, ':h'),
            \ 'type': fnamemodify(closest_file, ':t')
            \ }
    endif
    
    return {}
endfunction

" File type detection
function! rhema#filetype#is_rhema_file()
    let filename = expand('%:t')
    return filename =~ '\.rhema\.' || 
           filename =~ 'scope\.yaml' || 
           filename =~ 'knowledge\.yaml' ||
           filename =~ 'todos\.yaml' ||
           filename =~ 'decisions\.yaml' ||
           filename =~ 'patterns\.yaml' ||
           filename =~ 'conventions\.yaml' ||
           filename == 'rhema.yml'
endfunction

" Command execution with enhanced features
function! rhema#command#execute(args, callback = '', timeout = g:rhema_timeout)
    if !executable(g:rhema_executable)
        call rhema#log#error('Rhema executable not found: ' . g:rhema_executable)
        return ''
    endif

    let cmd = g:rhema_executable . ' ' . join(a:args, ' ')
    let s:rhema_last_command = cmd
    
    " Check cache first
    let cache_key = join(a:args, '|')
    let cached_result = rhema#cache#get(cache_key)
    if !empty(cached_result)
        call rhema#log#message('Using cached result for: ' . cmd)
        return cached_result
    endif

    call rhema#log#message('Executing: ' . cmd)
    call rhema#performance#start('command_execution')

    if has('nvim')
        " Neovim async execution
        let job_id = jobstart(cmd, {
            \ 'on_stdout': function('rhema#job#handle_output'),
            \ 'on_stderr': function('rhema#job#handle_error'),
            \ 'on_exit': function('rhema#job#handle_exit'),
            \ 'timeout': a:timeout * 1000
            \ })
        return job_id
    else
        " Vim synchronous execution
        let output = system(cmd)
        let s:rhema_last_output = output
        call rhema#performance#end('command_execution')
        
        if v:shell_error
            call rhema#log#error('Command failed: ' . output)
            let s:rhema_error_count += 1
            return ''
        else
            call rhema#log#message('Command succeeded')
            " Cache the result
            call rhema#cache#set(cache_key, output)
            
            if !empty(a:callback)
                call call(a:callback, [output])
            endif
            return output
        endif
    endif
endfunction

" Job handling for Neovim
function! rhema#job#handle_output(job_id, data, event)
    let s:rhema_last_output = join(a:data, "\n")
    call rhema#log#message('Job output received')
endfunction

function! rhema#job#handle_error(job_id, data, event)
    let error = join(a:data, "\n")
    call rhema#log#error('Job error: ' . error)
    let s:rhema_error_count += 1
endfunction

function! rhema#job#handle_exit(job_id, data, event)
    call rhema#performance#end('command_execution')
    
    if a:data == 0
        call rhema#log#message('Job completed successfully')
    else
        call rhema#log#error('Job failed with exit code: ' . a:data)
        let s:rhema_error_count += 1
    endif
endfunction

" Error handling
function! rhema#error#handle(error, context = '')
    let error_info = {
        \ 'error': a:error,
        \ 'context': a:context,
        \ 'timestamp': localtime(),
        \ 'file': expand('%:p'),
        \ 'line': line('.'),
        \ 'column': col('.')
        \ }
    
    call rhema#log#error('Error in ' . a:context . ': ' . a:error)
    let s:rhema_error_count += 1
    
    return error_info
endfunction

" Validation system
function! rhema#validation#validate_file(file = '')
    let target_file = empty(a:file) ? expand('%:p') : a:file
    
    if !filereadable(target_file)
        call rhema#log#error('File not readable: ' . target_file)
        return 0
    endif
    
    if !rhema#filetype#is_rhema_file()
        call rhema#log#warning('File is not a Rhema file: ' . target_file)
        return 0
    endif
    
    let output = rhema#command#execute(['validate', target_file])
    if !empty(output)
        call rhema#log#info('File validated successfully')
        return 1
    else
        call rhema#log#error('File validation failed')
        return 0
    endif
endfunction

" Auto-validation
function! rhema#validation#auto_validate()
    if g:rhema_auto_validate && rhema#filetype#is_rhema_file()
        call rhema#log#message('Auto-validating Rhema file...')
        call rhema#validation#validate_file()
    endif
endfunction

" Plugin initialization
function! rhema#init()
    call rhema#log#message('Initializing Rhema plugin...')
    
    " Check if Rhema executable is available
    if !executable(g:rhema_executable)
        call rhema#log#error('Rhema executable not found: ' . g:rhema_executable)
        return 0
    endif
    
    " Set up autocommands
    augroup RhemaAutoGroup
        autocmd!
        autocmd BufWritePost * call rhema#validation#auto_validate()
        autocmd BufRead,BufNewFile *.rhema.yml set filetype=rhema
        autocmd BufRead,BufNewFile rhema.yml set filetype=rhema
        autocmd BufRead,BufNewFile scope.yaml set filetype=rhema
        autocmd BufRead,BufNewFile knowledge.yaml set filetype=rhema
        autocmd BufRead,BufNewFile todos.yaml set filetype=rhema
        autocmd BufRead,BufNewFile decisions.yaml set filetype=rhema
        autocmd BufRead,BufNewFile patterns.yaml set filetype=rhema
        autocmd BufRead,BufNewFile conventions.yaml set filetype=rhema
    augroup END
    
    call rhema#log#info('Rhema plugin initialized successfully')
    return 1
endfunction

" Plugin cleanup
function! rhema#cleanup()
    call rhema#cache#clear()
    let s:rhema_cache = {}
    let s:rhema_context = {}
    let s:rhema_last_command = ''
    let s:rhema_last_output = ''
    let s:rhema_error_count = 0
    let s:rhema_warning_count = 0
    let s:rhema_performance_stats = {}
    
    call rhema#log#info('Rhema plugin cleaned up')
endfunction

" Get plugin status
function! rhema#status()
    return {
        \ 'enabled': g:rhema_enabled,
        \ 'executable': g:rhema_executable,
        \ 'executable_found': executable(g:rhema_executable),
        \ 'cache_stats': rhema#cache#stats(),
        \ 'error_count': s:rhema_error_count,
        \ 'warning_count': s:rhema_warning_count,
        \ 'last_command': s:rhema_last_command,
        \ 'context': rhema#context#get_current()
        \ }
endfunction 