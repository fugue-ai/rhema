" Rhema UI System
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

" Global UI state
let s:rhema_output_buffer = -1
let s:rhema_output_window = -1
let s:rhema_sidebar_buffer = -1
let s:rhema_sidebar_window = -1

" Show output in a dedicated buffer
function! rhema#ui#show_output(output, title)
    let bufname = '[RHEMA] ' . a:title
    
    " Check if buffer already exists
    let buf = bufnr(bufname)
    if buf == -1
        " Create new buffer
        execute 'new ' . bufname
        let s:rhema_output_buffer = bufnr('%')
    else
        " Switch to existing buffer
        execute 'buffer ' . buf
        let s:rhema_output_buffer = buf
    endif
    
    " Set buffer properties
    setlocal modifiable
    setlocal buftype=nofile
    setlocal bufhidden=hide
    setlocal noswapfile
    setlocal filetype=yaml
    setlocal readonly
    
    " Clear buffer and insert output
    silent! %delete _
    call setline(1, split(a:output, '\n'))
    
    " Set syntax highlighting
    if has('syntax')
        syntax on
    endif
    
    " Position cursor at top
    normal! gg
    
    " Set up buffer-local mappings
    call rhema#ui#setup_output_mappings()
    
    " Mark buffer as Rhema output
    let b:rhema_output = 1
    let b:rhema_output_title = a:title
    let b:rhema_output_content = a:output
    
    call rhema#log#info('Output displayed in buffer: ' . bufname)
endfunction

" Setup output buffer mappings
function! rhema#ui#setup_output_mappings()
    if !exists('g:rhema_no_mappings') || !g:rhema_no_mappings
        " Navigation
        nnoremap <buffer> <silent> q :bdelete<CR>
        nnoremap <buffer> <silent> <CR> :call rhema#ui#open_selected_item()<CR>
        
        " Search
        nnoremap <buffer> <silent> / :call rhema#ui#search_in_output()<CR>
        nnoremap <buffer> <silent> n :call rhema#ui#next_search_result()<CR>
        nnoremap <buffer> <silent> N :call rhema#ui#prev_search_result()<CR>
        
        " Copy
        nnoremap <buffer> <silent> y :call rhema#ui#copy_selection()<CR>
        nnoremap <buffer> <silent> Y :call rhema#ui#copy_all()<CR>
        
        " Refresh
        nnoremap <buffer> <silent> r :call rhema#ui#refresh_output()<CR>
        
        " Help
        nnoremap <buffer> <silent> ? :call rhema#ui#show_help()<CR>
    endif
endfunction

" Open selected item (for links, file paths, etc.)
function! rhema#ui#open_selected_item()
    let line = getline('.')
    
    " Check if line contains a file path
    let file_path = matchstr(line, '\/[^[:space:]]*\.yaml\?')
    if !empty(file_path)
        if filereadable(file_path)
            execute 'edit ' . file_path
            return
        endif
    endif
    
    " Check if line contains a URL
    let url = matchstr(line, 'https\?://[^\s]*')
    if !empty(url)
        if has('unix')
            call system('open "' . url . '"')
        elseif has('win32')
            call system('start "' . url . '"')
        endif
        return
    endif
    
    " Check if line contains a command
    let command = matchstr(line, 'rhema [a-zA-Z-]\+')
    if !empty(command)
        execute '!' . command
        return
    endif
    
    " Default: just echo the line
    echom 'Selected: ' . line
endfunction

" Search in output
function! rhema#ui#search_in_output()
    let search_term = input('Search in output: ')
    if !empty(search_term)
        let @/ = search_term
        call search(search_term)
        call rhema#log#info('Searching for: ' . search_term)
    endif
endfunction

" Next search result
function! rhema#ui#next_search_result()
    if !empty(@/)
        call search(@/, '')
    endif
endfunction

" Previous search result
function! rhema#ui#prev_search_result()
    if !empty(@/)
        call search(@/, 'b')
    endif
endfunction

" Copy selection
function! rhema#ui#copy_selection()
    if has('clipboard')
        let @" = getline('.')
        call rhema#log#info('Line copied to clipboard')
    else
        call rhema#log#warning('Clipboard not available')
    endif
endfunction

" Copy all output
function! rhema#ui#copy_all()
    if has('clipboard')
        let @" = join(getline(1, '$'), "\n")
        call rhema#log#info('All output copied to clipboard')
    else
        call rhema#log#warning('Clipboard not available')
    endif
endfunction

" Refresh output
function! rhema#ui#refresh_output()
    if exists('b:rhema_output_title')
        let title = b:rhema_output_title
        " Re-run the command that generated this output
        let command = matchstr(title, 'Rhema \zs.*')
        if !empty(command)
            call rhema#commands#execute_interactive(command)
        endif
    endif
endfunction

" Show help
function! rhema#ui#show_help()
    let help_text = [
        \ 'Rhema Output Buffer Help:',
        \ '',
        \ 'Navigation:',
        \ '  <CR>  - Open selected item (file, URL, command)',
        \ '  q     - Close buffer',
        \ '',
        \ 'Search:',
        \ '  /     - Search in output',
        \ '  n     - Next search result',
        \ '  N     - Previous search result',
        \ '',
        \ 'Copy:',
        \ '  y     - Copy current line',
        \ '  Y     - Copy all output',
        \ '',
        \ 'Other:',
        \ '  r     - Refresh output',
        \ '  ?     - Show this help',
        \ ''
        \ ]
    
    call rhema#ui#show_output(join(help_text, "\n"), 'Help')
endfunction

" Show sidebar with Rhema components
function! rhema#ui#show_sidebar()
    let bufname = '[RHEMA] Sidebar'
    
    " Check if sidebar already exists
    let buf = bufnr(bufname)
    if buf == -1
        " Create new buffer
        execute 'vnew ' . bufname
        let s:rhema_sidebar_buffer = bufnr('%')
    else
        " Switch to existing buffer
        execute 'buffer ' . buf
        let s:rhema_sidebar_buffer = buf
    endif
    
    " Set buffer properties
    setlocal modifiable
    setlocal buftype=nofile
    setlocal bufhidden=hide
    setlocal noswapfile
    setlocal filetype=rhema
    setlocal readonly
    
    " Generate sidebar content
    let content = rhema#ui#generate_sidebar_content()
    
    " Clear buffer and insert content
    silent! %delete _
    call setline(1, split(content, '\n'))
    
    " Set syntax highlighting
    if has('syntax')
        syntax on
    endif
    
    " Position cursor at top
    normal! gg
    
    " Set up buffer-local mappings
    call rhema#ui#setup_sidebar_mappings()
    
    " Mark buffer as Rhema sidebar
    let b:rhema_sidebar = 1
    
    call rhema#log#info('Sidebar displayed')
endfunction

" Generate sidebar content
function! rhema#ui#generate_sidebar_content()
    let content = []
    
    " Get current context
    let context = rhema#context#get_current()
    if !empty(context)
        call add(content, 'Current Context:')
        call add(content, '  File: ' . fnamemodify(context.file, ':t'))
        call add(content, '  Root: ' . fnamemodify(context.root, ':t'))
        call add(content, '')
    endif
    
    " Get Rhema files in current project
    let rhema_files = rhema#context#detect()
    if !empty(rhema_files)
        call add(content, 'Rhema Files:')
        for file in rhema_files
            let filename = fnamemodify(file, ':t')
            let relative_path = fnamemodify(file, ':h')
            call add(content, '  ' . filename . ' (' . relative_path . ')')
        endfor
        call add(content, '')
    endif
    
    " Get scopes
    let scopes_output = rhema#command#execute(['scopes'])
    if !empty(scopes_output)
        call add(content, 'Scopes:')
        let lines = split(scopes_output, '\n')
        for line in lines[:10] " Limit to first 10 lines
            call add(content, '  ' . line)
        endfor
        if len(lines) > 10
            call add(content, '  ... (' . (len(lines) - 10) . ' more)')
        endif
        call add(content, '')
    endif
    
    " Get recent todos
    let todos_output = rhema#command#execute(['todo', 'list'])
    if !empty(todos_output)
        call add(content, 'Recent Todos:')
        let lines = split(todos_output, '\n')
        for line in lines[:5] " Limit to first 5 lines
            call add(content, '  ' . line)
        endfor
        if len(lines) > 5
            call add(content, '  ... (' . (len(lines) - 5) . ' more)')
        endif
        call add(content, '')
    endif
    
    " Get recent insights
    let insights_output = rhema#command#execute(['insight', 'list'])
    if !empty(insights_output)
        call add(content, 'Recent Insights:')
        let lines = split(insights_output, '\n')
        for line in lines[:5] " Limit to first 5 lines
            call add(content, '  ' . line)
        endfor
        if len(lines) > 5
            call add(content, '  ... (' . (len(lines) - 5) . ' more)')
        endif
        call add(content, '')
    endif
    
    " Add navigation help
    call add(content, 'Navigation:')
    call add(content, '  <CR> - Open file')
    call add(content, '  q    - Close sidebar')
    call add(content, '  r    - Refresh')
    
    return join(content, "\n")
endfunction

" Setup sidebar buffer mappings
function! rhema#ui#setup_sidebar_mappings()
    if !exists('g:rhema_no_mappings') || !g:rhema_no_mappings
        " Navigation
        nnoremap <buffer> <silent> q :bdelete<CR>
        nnoremap <buffer> <silent> <CR> :call rhema#ui#open_sidebar_item()<CR>
        
        " Refresh
        nnoremap <buffer> <silent> r :call rhema#ui#refresh_sidebar()<CR>
        
        " Help
        nnoremap <buffer> <silent> ? :call rhema#ui#show_sidebar_help()<CR>
    endif
endfunction

" Open sidebar item
function! rhema#ui#open_sidebar_item()
    let line = getline('.')
    
    " Check if line contains a file path
    let file_path = matchstr(line, '([^)]*)')
    if !empty(file_path)
        let full_path = file_path . '/' . matchstr(line, '^\s*\zs[^(]*')
        if filereadable(full_path)
            execute 'edit ' . full_path
            return
        endif
    endif
    
    " Check if line contains a filename
    let filename = matchstr(line, '^\s*\zs[^[:space:]]*\.yaml\?')
    if !empty(filename)
        " Look for the file in the project
        let files = split(globpath(expand('%:p:h'), '**/' . filename), '\n')
        if !empty(files)
            execute 'edit ' . files[0]
            return
        endif
    endif
    
    " Default: just echo the line
    echom 'Selected: ' . line
endfunction

" Refresh sidebar
function! rhema#ui#refresh_sidebar()
    let content = rhema#ui#generate_sidebar_content()
    setlocal modifiable
    silent! %delete _
    call setline(1, split(content, '\n'))
    setlocal nomodifiable
    call rhema#log#info('Sidebar refreshed')
endfunction

" Show sidebar help
function! rhema#ui#show_sidebar_help()
    let help_text = [
        \ 'Rhema Sidebar Help:',
        \ '',
        \ 'Navigation:',
        \ '  <CR>  - Open selected file',
        \ '  q     - Close sidebar',
        \ '  r     - Refresh sidebar',
        \ '  ?     - Show this help',
        \ '',
        \ 'The sidebar shows:',
        \ '  - Current context information',
        \ '  - Rhema files in the project',
        \ '  - Available scopes',
        \ '  - Recent todos and insights',
        \ ''
        \ ]
    
    call rhema#ui#show_output(join(help_text, "\n"), 'Sidebar Help')
endfunction

" Show interactive prompt
function! rhema#ui#show_prompt(prompt, options = [])
    let prompt_text = a:prompt
    if !empty(a:options)
        let prompt_text .= ' (' . join(a:options, '/') . '): '
    else
        let prompt_text .= ': '
    endif
    
    let response = input(prompt_text)
    
    " If options provided, validate response
    if !empty(a:options)
        let valid = 0
        for option in a:options
            if response =~? '^' . option
                let valid = 1
                break
            endif
        endfor
        
        if !valid
            call rhema#log#warning('Invalid option: ' . response . '. Valid options: ' . join(a:options, ', '))
            return ''
        endif
    endif
    
    return response
endfunction

" Show confirmation dialog
function! rhema#ui#show_confirm(message, default = 'n')
    let response = input(a:message . ' (y/n): ')
    if empty(response)
        let response = a:default
    endif
    
    return response =~? '^y'
endfunction

" Show selection menu
function! rhema#ui#show_menu(title, options)
    let menu_text = a:title . ":\n"
    let i = 1
    for option in a:options
        let menu_text .= i . ". " . option . "\n"
        let i += 1
    endfor
    let menu_text .= "\nEnter choice (1-" . (len(a:options)) . "): "
    
    let choice = input(menu_text)
    let choice_num = str2nr(choice)
    
    if choice_num >= 1 && choice_num <= len(a:options)
        return a:options[choice_num - 1]
    else
        call rhema#log#warning('Invalid choice: ' . choice)
        return ''
    endif
endfunction

" Show progress indicator
function! rhema#ui#show_progress(message)
    echom '[RHEMA] ' . a:message . '...'
    redraw
endfunction

" Show success message
function! rhema#ui#show_success(message)
    echohl String
    echom '[RHEMA] ✓ ' . a:message
    echohl None
endfunction

" Show error message
function! rhema#ui#show_error(message)
    echohl ErrorMsg
    echom '[RHEMA] ✗ ' . a:message
    echohl None
endfunction

" Show warning message
function! rhema#ui#show_warning(message)
    echohl WarningMsg
    echom '[RHEMA] ⚠ ' . a:message
    echohl None
endfunction

" Show info message
function! rhema#ui#show_info(message)
    echom '[RHEMA] ℹ ' . a:message
endfunction 