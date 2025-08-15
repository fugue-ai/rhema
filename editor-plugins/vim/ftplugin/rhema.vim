" Rhema File Type Plugin
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

" Only load once
if exists('b:loaded_rhema_ftplugin')
    finish
endif

let b:loaded_rhema_ftplugin = 1

" Set buffer-local options
setlocal expandtab
setlocal shiftwidth=2
setlocal tabstop=2
setlocal softtabstop=2
setlocal autoindent
setlocal smartindent
setlocal foldmethod=indent
setlocal foldlevel=1
setlocal textwidth=80
setlocal wrap
setlocal linebreak
setlocal nolist

" Enable syntax highlighting
if has('syntax')
    syntax on
endif

" Set comment format for YAML
setlocal comments=:#
setlocal commentstring=#%s

" Set format options
setlocal formatoptions=croql

" Enable spell checking for comments and strings
if has('spell')
    setlocal spell
    setlocal spelllang=en
endif

" Set up indentation for YAML
setlocal indentexpr=GetRhemaIndent()

" Function to get proper indentation for Rhema YAML files
function! GetRhemaIndent()
    let lnum = v:lnum
    let line = getline(lnum)
    let prev_line = getline(lnum - 1)
    
    " If this is the first line, no indentation
    if lnum == 1
        return 0
    endif
    
    " If line starts with #, align with previous comment or start at 0
    if line =~ '^\s*#'
        let prev_comment = 0
        let i = lnum - 1
        while i > 0
            let prev = getline(i)
            if prev =~ '^\s*#'
                let prev_comment = indent(i)
                break
            elseif prev =~ '^\s*$'
                let i -= 1
                continue
            else
                break
            endif
            let i -= 1
        endwhile
        return prev_comment
    endif
    
    " If previous line ends with :, increase indentation
    if prev_line =~ ':\s*$'
        return indent(lnum - 1) + 2
    endif
    
    " If this line starts with -, align with previous list item
    if line =~ '^\s*-'
        let i = lnum - 1
        while i > 0
            let prev = getline(i)
            if prev =~ '^\s*-'
                return indent(i)
            elseif prev =~ '^\s*$'
                let i -= 1
                continue
            else
                break
            endif
            let i -= 1
        endwhile
        return indent(lnum - 1) + 2
    endif
    
    " Default to previous line's indentation
    return indent(lnum - 1)
endfunction

" Set up omni-completion
if has('autocmd')
    autocmd BufEnter <buffer> setlocal omnifunc=rhema#complete#omni
endif

" Buffer-local mappings for Rhema files
if !exists('g:rhema_no_mappings') || !g:rhema_no_mappings
    " Quick validation
    nnoremap <buffer> <silent> <leader>v :call rhema#validation#validate_file()<CR>
    
    " Quick context show
    nnoremap <buffer> <silent> <leader>c :call rhema#context#show_current()<CR>
    
    " Quick search in current file
    nnoremap <buffer> <silent> <leader>s :call rhema#search#in_current_file()<CR>
    
    " Format current file
    nnoremap <buffer> <silent> <leader>f :call rhema#format#current_file()<CR>
    
    " Show file statistics
    nnoremap <buffer> <silent> <leader>i :call rhema#info#show_file_stats()<CR>
    
    " Navigate to related files
    nnoremap <buffer> <silent> <leader>n :call rhema#navigation#next_related_file()<CR>
    nnoremap <buffer> <silent> <leader>p :call rhema#navigation#prev_related_file()<CR>
    
    " Insert templates
    nnoremap <buffer> <silent> <leader>tt :call rhema#template#insert_todo()<CR>
    nnoremap <buffer> <silent> <leader>ti :call rhema#template#insert_insight()<CR>
    nnoremap <buffer> <silent> <leader>tp :call rhema#template#insert_pattern()<CR>
    nnoremap <buffer> <silent> <leader>td :call rhema#template#insert_decision()<CR>
endif

" Auto-format on save if enabled
if exists('g:rhema_auto_format') && g:rhema_auto_format
    autocmd BufWritePre <buffer> call rhema#format#auto_format()
endif

" Auto-validate on save if enabled
if exists('g:rhema_auto_validate') && g:rhema_auto_validate
    autocmd BufWritePost <buffer> call rhema#validation#auto_validate()
endif

" Set up status line for Rhema files
if exists('g:rhema_show_status') && g:rhema_show_status
    setlocal statusline=%<%f\ %h%m%r%=%{rhema#status#get_file_status()}\ %-14.(%l,%c%V%)\ %P
endif

" Set up signs for validation errors if supported
if has('signs')
    autocmd BufRead <buffer> call rhema#validation#show_signs()
    autocmd BufWritePost <buffer> call rhema#validation#update_signs()
endif

" Set up quickfix list for validation errors
if has('quickfix')
    autocmd BufWritePost <buffer> call rhema#validation#update_quickfix()
endif

" Set up local variables for this buffer
let b:rhema_file_type = fnamemodify(expand('%:t'), ':r')
let b:rhema_context = rhema#context#get_current()

" Function to get file status for status line
function! rhema#status#get_file_status()
    let status = []
    
    " Add file type
    call add(status, b:rhema_file_type)
    
    " Add validation status
    if exists('b:rhema_validated') && b:rhema_validated
        call add(status, '✓')
    else
        call add(status, '✗')
    endif
    
    " Add context info
    if !empty(b:rhema_context)
        call add(status, fnamemodify(b:rhema_context.file, ':t'))
    endif
    
    return join(status, ' ')
endfunction

" Function to show validation signs
function! rhema#validation#show_signs()
    if !has('signs')
        return
    endif
    
    " Clear existing signs
    sign unplace *
    
    " Add signs for validation errors
    let lines = getline(1, '$')
    let line_num = 1
    
    for line in lines
        if line =~ 'ERROR:' || line =~ 'FAILED:'
            execute 'sign place ' . line_num . ' line=' . line_num . ' name=RhemaError'
        elseif line =~ 'WARNING:' || line =~ 'DEPRECATED:'
            execute 'sign place ' . line_num . ' line=' . line_num . ' name=RhemaWarning'
        endif
        let line_num += 1
    endfor
endfunction

" Function to update validation signs
function! rhema#validation#update_signs()
    call rhema#validation#show_signs()
endfunction

" Function to update quickfix list
function! rhema#validation#update_quickfix()
    if !has('quickfix')
        return
    endif
    
    let qf_list = []
    let lines = getline(1, '$')
    let line_num = 1
    
    for line in lines
        if line =~ 'ERROR:' || line =~ 'FAILED:'
            call add(qf_list, {
                \ 'filename': expand('%:p'),
                \ 'lnum': line_num,
                \ 'text': line,
                \ 'type': 'E'
                \ })
        elseif line =~ 'WARNING:' || line =~ 'DEPRECATED:'
            call add(qf_list, {
                \ 'filename': expand('%:p'),
                \ 'lnum': line_num,
                \ 'text': line,
                \ 'type': 'W'
                \ })
        endif
        let line_num += 1
    endfor
    
    if !empty(qf_list)
        call setqflist(qf_list)
    endif
endfunction

" Define signs for validation
if has('signs')
    sign define RhemaError text=>> texthl=Error
    sign define RhemaWarning text=>> texthl=WarningMsg
endif 