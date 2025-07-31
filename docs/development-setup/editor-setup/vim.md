# Setting Up Vim/Neovim for Rhema Development


This guide will help you configure Vim or Neovim to work effectively with Rhema (Git-Based Agent Context Protocol) projects. Vim's extensibility, terminal integration, and powerful text editing capabilities make it an excellent choice for Rhema development.

## Prerequisites


- [Vim](https://www.vim.org/) or [Neovim](https://neovim.io/) installed on your system

- [Rhema CLI](../README.md#installation) installed

- A Git repository (or create one for testing)

## Installation


### 1. Install Rhema CLI


First, ensure you have the Rhema CLI installed:

```bash
# From Cargo (recommended)


cargo install rhema-cli

# Or build from source


git clone https://github.com/fugue-ai/rhema.git
cd rhema
cargo build --release
```

### 2. Verify Installation


```bash
rhema --version
```

## Vim/Neovim Configuration


### 1. Install Recommended Plugins


Vim/Neovim works best with Rhema when you have these plugins installed:

#### Essential Plugins


- **[vim-yaml](https://github.com/stephpy/vim-yaml)** - YAML syntax highlighting and indentation

- **[vim-gitgutter](https://github.com/airblade/vim-gitgutter)** - Git diff indicators

- **[vim-fugitive](https://github.com/tpope/vim-fugitive)** - Git integration

- **[coc.nvim](https://github.com/neoclide/coc.nvim)** - Language server support (Neovim)

- **[ale](https://github.com/dense-analysis/ale)** - Linting and fixing (Vim)

#### Recommended Plugins


- **[nerdtree](https://github.com/preservim/nerdtree)** - File explorer

- **[ctrlp](https://github.com/ctrlpvim/ctrlp.vim)** - Fuzzy file finder

- **[vim-commentary](https://github.com/tpope/vim-commentary)** - Comment/uncomment lines

- **[vim-surround](https://github.com/tpope/vim-surround)** - Surround text with brackets/quotes

- **[vim-repeat](https://github.com/tpope/vim-repeat)** - Repeat plugin commands with `.`

### 2. Plugin Manager Setup


#### Using vim-plug (Recommended)


Add to your `.vimrc` or `init.vim`:

```vim
" Plugin manager
call plug#begin('~/.vim/plugged')

" Essential plugins
Plug 'stephpy/vim-yaml'
Plug 'airblade/vim-gitgutter'
Plug 'tpope/vim-fugitive'

" Language server support (Neovim)
if has('nvim')
  Plug 'neoclide/coc.nvim', {'branch': 'release'}
else
  " Linting for Vim
  Plug 'dense-analysis/ale'
endif

" Recommended plugins
Plug 'preservim/nerdtree'
Plug 'ctrlpvim/ctrlp.vim'
Plug 'tpope/vim-commentary'
Plug 'tpope/vim-surround'
Plug 'tpope/vim-repeat'

" YAML schema validation
Plug 'redhat-developer/vscode-yaml'

call plug#end()
```

#### Using packer.nvim (Neovim)


Add to your `init.lua`:

```lua
require('packer').startup(function(use)
  -- Essential plugins
  use 'stephpy/vim-yaml'
  use 'airblade/vim-gitgutter'
  use 'tpope/vim-fugitive'
  use 'neoclide/coc.nvim'
  
  -- Recommended plugins
  use 'preservim/nerdtree'
  use 'ctrlpvim/ctrlp.vim'
  use 'tpope/vim-commentary'
  use 'tpope/vim-surround'
  use 'tpope/vim-repeat'
  
  -- YAML schema validation
  use 'redhat-developer/vscode-yaml'
end)
```

### 3. Basic Configuration


Add these settings to your `.vimrc` or `init.vim`:

```vim
" Basic settings
set nocompatible
set number
set relativenumber
set autoindent
set smartindent
set expandtab
set tabstop=2
set shiftwidth=2
set softtabstop=2
set backspace=indent,eol,start
set incsearch
set hlsearch
set ignorecase
set smartcase
set wildmenu
set wildmode=list:longest
set ruler
set showcmd
set showmatch
set laststatus=2
set encoding=utf-8
set fileencoding=utf-8

" File type specific settings
filetype plugin indent on
syntax on

" YAML specific settings
autocmd FileType yaml setlocal ts=2 sts=2 sw=2 expandtab
autocmd FileType yaml setlocal foldmethod=indent
autocmd FileType yaml setlocal foldlevel=20

" Rhema file associations
autocmd BufNewFile,BufRead *.rhema.yaml set filetype=yaml
autocmd BufNewFile,BufRead rhema.yaml set filetype=yaml
autocmd BufNewFile,BufRead knowledge.yaml set filetype=yaml
autocmd BufNewFile,BufRead todos.yaml set filetype=yaml
autocmd BufNewFile,BufRead decisions.yaml set filetype=yaml
autocmd BufNewFile,BufRead patterns.yaml set filetype=yaml
autocmd BufNewFile,BufRead conventions.yaml set filetype=yaml

" Git integration
let g:gitgutter_enabled = 1
let g:gitgutter_signs = 1
let g:gitgutter_highlight_lines = 1

" NERDTree settings
let g:NERDTreeShowHidden = 1
let g:NERDTreeIgnore = ['\.git$', '\.rhema/temp$', '\.rhema/cache$', 'target$']

" CtrlP settings
let g:ctrlp_working_path_mode = 'ra'
let g:ctrlp_custom_ignore = {
  \ 'dir':  '\v[\/]\.(git|hg|svn)$',
  \ 'file': '\v\.(exe|so|dll)$',
  \ 'link': 'some_bad_symbolic_links',
  \ }
```

### 4. Language Server Configuration


#### COC.nvim Configuration (Neovim)


Create `~/.config/nvim/coc-settings.json`:

```json
{
  "yaml.schemas": {
    "schemas/rhema.json": [
      "**/rhema.yaml",
      "**/knowledge.yaml",
      "**/todos.yaml",
      "**/decisions.yaml",
      "**/patterns.yaml",
      "**/conventions.yaml"
    ]
  },
  "yaml.validate": true,
  "yaml.format.enable": true,
  "yaml.hover": true,
  "yaml.completion": true,
  "yaml.customTags": [
    "!reference sequence"
  ]
}
```

#### ALE Configuration (Vim)


Add to your `.vimrc`:

```vim
" ALE settings
let g:ale_enabled = 1
let g:ale_linters = {
  \ 'yaml': ['yamllint'],
  \ 'rust': ['cargo', 'rustc'],
  \ }
let g:ale_fixers = {
  \ 'yaml': ['prettier'],
  \ 'rust': ['rustfmt'],
  \ }
let g:ale_fix_on_save = 1
let g:ale_sign_error = '✗'
let g:ale_sign_warning = '⚠'
```

## Workflow Integration


### 1. Initialize a Rhema Scope


1. Open your project in Vim/Neovim

2. Use `:!rhema init` to initialize a scope

3. Or use the terminal: `rhema init`

This creates the initial `.rhema/` directory with template files.

### 2. Configure AI Context


Create a `.copilot` file in your project root:

```
# Rhema Context Integration


This project uses Rhema (Git-Based Agent Context Protocol) for structured context management.

## Key Files to Reference:


- .rhema/rhema.yaml - Scope definition and metadata

- .rhema/knowledge.yaml - Domain knowledge and insights  

- .rhema/todos.yaml - Work items and tasks

- .rhema/decisions.yaml - Architecture decisions

- .rhema/patterns.yaml - Design patterns

- .rhema/conventions.yaml - Coding standards

## When Providing Assistance:


1. Check .rhema/knowledge.yaml for existing insights and domain knowledge

2. Review .rhema/decisions.yaml for architectural decisions

3. Consider .rhema/patterns.yaml for established design patterns

4. Follow .rhema/conventions.yaml for coding standards

5. Update relevant Rhema files when making significant changes

## Common Rhema Commands:


- rhema query "todos WHERE status='in_progress'" - Find active work

- rhema insight record "finding" - Record new insights

- rhema decision record "title" - Record architectural decisions

- rhema validate --recursive - Validate all Rhema files
```

### 3. Create Custom Snippets


#### Using UltiSnips


Install UltiSnips and create `~/.vim/UltiSnips/yaml.snippets`:

```snippets
snippet rhema-todo "Rhema Todo Item"

- id: "todo-${1:001}"
  title: "${2:Todo title}"
  description: "${3:Detailed description}"
  status: ${4|pending,in_progress,completed,blocked|}
  priority: ${5|low,medium,high,critical|}
  assigned_to: "${6:assignee}"
  created_at: "${7:`strftime('%Y-%m-%dT%H:%M:%SZ')`}"
  tags: [${8:tag1, tag2}]
  related_components: [${9:component1, component2}]
endsnippet

snippet rhema-insight "Rhema Insight"

- finding: "${1:Insight finding}"
  impact: "${2:Impact description}"
  solution: "${3:Proposed solution}"
  confidence: ${4|low,medium,high|}
  evidence: [${5:evidence1, evidence2}]
  related_files: [${6:file1, file2}]
  category: ${7|performance,security,architecture,user_experience|}
  recorded_at: "${8:`strftime('%Y-%m-%dT%H:%M:%SZ')`}"
endsnippet

snippet rhema-decision "Rhema Decision"

- id: "decision-${1:001}"
  title: "${2:Decision title}"
  description: "${3:Detailed description}"
  status: ${4|proposed,approved,rejected,deprecated|}
  rationale: "${5:Decision rationale}"
  alternatives_considered: [${6:alt1, alt2}]
  impact: "${7:Impact description}"
  decided_at: "${8:`strftime('%Y-%m-%dT%H:%M:%SZ')`}"
endsnippet
```

## Git Integration


### 1. Fugitive Configuration


Add to your `.vimrc`:

```vim
" Fugitive settings
let g:fugitive_git_executable = 'git'
let g:fugitive_legacy_commands = 0

" Git status in statusline
set statusline=%<%f\ %h%m%r%{fugitive#statusline()}%=%-14.(%l,%c%V%)\ %P
```

### 2. Git Hooks Setup


Configure Git hooks for Rhema validation:

#### Pre-commit Hook


Create a `.git/hooks/pre-commit` file:

```bash
#!/bin/sh


# Rhema Pre-commit Hook


echo "Running Rhema validation..."

# Run Rhema validation


if command -v rhema >/dev/null 2>&1; then
    if ! rhema validate --recursive; then
        echo "Rhema validation failed. Please fix issues before committing."
        exit 1
    fi
    echo "Rhema validation passed."
else
    echo "Rhema CLI not found. Skipping validation."
fi
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Custom Commands and Functions


### 1. Rhema Commands


Add these to your `.vimrc`:

```vim
" Rhema commands
command! RhemaInit :!rhema init
command! RhemaValidate :!rhema validate --recursive
command! RhemaHealth :!rhema health
command! RhemaScopes :!rhema scopes
command! RhemaQuery :call RhemaQuery()

" Rhema query function
function! RhemaQuery()
  let query = input('Rhema Query: ')
  if query != ''
    execute '!rhema query "' . query . '"'
  endif
endfunction
```

### 2. File Navigation


Add these mappings to your `.vimrc`:

```vim
" Rhema file navigation
nnoremap <leader>gg :e .rhema/rhema.yaml<CR>
nnoremap <leader>gk :e .rhema/knowledge.yaml<CR>
nnoremap <leader>gt :e .rhema/todos.yaml<CR>
nnoremap <leader>gd :e .rhema/decisions.yaml<CR>
nnoremap <leader>gp :e .rhema/patterns.yaml<CR>
nnoremap <leader>gc :e .rhema/conventions.yaml<CR>

" Rhema commands
nnoremap <leader>gi :RhemaInit<CR>
nnoremap <leader>gv :RhemaValidate<CR>
nnoremap <leader>gh :RhemaHealth<CR>
nnoremap <leader>gs :RhemaScopes<CR>
nnoremap <leader>gq :RhemaQuery<CR>
```

## Terminal Integration


### 1. Terminal Commands


Vim/Neovim integrates well with terminal commands:

```vim
" Terminal integration
if has('nvim')
  " Neovim terminal
  nnoremap <leader>t :terminal<CR>
  tnoremap <Esc> <C-\><C-n>
else
  " Vim terminal (if available)
  if has('terminal')
    nnoremap <leader>t :terminal<CR>
  endif
endif
```

### 2. Async Commands


For Neovim, you can use async commands:

```vim
" Async Rhema commands (Neovim)
if has('nvim')
  function! RhemaValidateAsync()
    call jobstart(['rhema', 'validate', '--recursive'], {
      \ 'on_exit': function('RhemaValidateCallback')
      \ })
  endfunction

  function! RhemaValidateCallback(job_id, data, event)
    if a:event == 'exit'
      if a:data == 0
        echo "Rhema validation passed"
      else
        echo "Rhema validation failed"
      endif
    endif
  endfunction

  command! RhemaValidateAsync :call RhemaValidateAsync()
endif
```

## Best Practices


### 1. Regular Context Maintenance


- Run `rhema validate --recursive` before commits

- Update knowledge files when discovering new insights

- Record decisions as they're made, not after the fact

- Keep todos current and accurate

### 2. Vim/Neovim Specific


- Use `:RhemaValidate` before saving important changes

- Leverage Vim's powerful text editing for Rhema files

- Use macros for repetitive Rhema file operations

- Take advantage of Vim's search and replace capabilities

### 3. Team Coordination


- Commit Rhema files with related code changes

- Use Rhema context in code reviews

- Share insights and decisions through Rhema files

- Use cross-scope queries for project-wide coordination

## Troubleshooting


### Common Issues


1. **YAML validation errors**: Ensure your Rhema files follow the schema in `schemas/rhema.json`

2. **Missing context**: Run `rhema health` to check scope completeness

3. **Plugin not working**: Check plugin installation and configuration

4. **Schema not loading**: Check that `schemas/rhema.json` path is correct

5. **Language server issues**: Ensure COC.nvim or ALE is properly configured

### Getting Help


- Run `rhema --help` for command documentation

- Check the [Rhema README](../README.md) for protocol details

- Use `rhema validate --recursive` to identify issues

- Review the [protocol schemas](../schemas/) for file formats

- Check Vim/Neovim documentation and plugin help

## Next Steps


1. **Initialize your first scope**: `rhema init`

2. **Explore existing context**: `rhema scopes` and `rhema query`

3. **Start recording knowledge**: Use `rhema insight record`

4. **Set up team workflows**: Share Rhema practices with your team

5. **Integrate with CI/CD**: Add Rhema validation to your build pipeline

For more advanced usage, see the [Rhema CLI Reference](../README.md#cli-command-reference), [Protocol Documentation](../schemas/), and [Rust Development Setup](../development/rust-setup.md). 