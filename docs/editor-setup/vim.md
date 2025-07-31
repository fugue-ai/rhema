# Setting Up Vim/Neovim for GACP Development

This guide will help you configure Vim or Neovim to work effectively with GACP (Git-Based Agent Context Protocol) projects. Vim's extensibility, terminal integration, and powerful text editing capabilities make it an excellent choice for GACP development.

## Prerequisites

- [Vim](https://www.vim.org/) or [Neovim](https://neovim.io/) installed on your system
- [GACP CLI](../README.md#installation) installed
- A Git repository (or create one for testing)

## Installation

### 1. Install GACP CLI

First, ensure you have the GACP CLI installed:

```bash
# From Cargo (recommended)
cargo install gacp-cli

# Or build from source
git clone https://github.com/fugue-ai/gacp.git
cd gacp
cargo build --release
```

### 2. Verify Installation

```bash
gacp --version
```

## Vim/Neovim Configuration

### 1. Install Recommended Plugins

Vim/Neovim works best with GACP when you have these plugins installed:

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

" GACP file associations
autocmd BufNewFile,BufRead *.gacp.yaml set filetype=yaml
autocmd BufNewFile,BufRead gacp.yaml set filetype=yaml
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
let g:NERDTreeIgnore = ['\.git$', '\.gacp/temp$', '\.gacp/cache$', 'target$']

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
    "schemas/gacp.json": [
      "**/gacp.yaml",
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

### 1. Initialize a GACP Scope

1. Open your project in Vim/Neovim
2. Use `:!gacp init` to initialize a scope
3. Or use the terminal: `gacp init`

This creates the initial `.gacp/` directory with template files.

### 2. Configure AI Context

Create a `.copilot` file in your project root:

```
# GACP Context Integration

This project uses GACP (Git-Based Agent Context Protocol) for structured context management.

## Key Files to Reference:
- .gacp/gacp.yaml - Scope definition and metadata
- .gacp/knowledge.yaml - Domain knowledge and insights  
- .gacp/todos.yaml - Work items and tasks
- .gacp/decisions.yaml - Architecture decisions
- .gacp/patterns.yaml - Design patterns
- .gacp/conventions.yaml - Coding standards

## When Providing Assistance:
1. Check .gacp/knowledge.yaml for existing insights and domain knowledge
2. Review .gacp/decisions.yaml for architectural decisions
3. Consider .gacp/patterns.yaml for established design patterns
4. Follow .gacp/conventions.yaml for coding standards
5. Update relevant GACP files when making significant changes

## Common GACP Commands:
- gacp query "todos WHERE status='in_progress'" - Find active work
- gacp insight record "finding" - Record new insights
- gacp decision record "title" - Record architectural decisions
- gacp validate --recursive - Validate all GACP files
```

### 3. Create Custom Snippets

#### Using UltiSnips

Install UltiSnips and create `~/.vim/UltiSnips/yaml.snippets`:

```snippets
snippet gacp-todo "GACP Todo Item"
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

snippet gacp-insight "GACP Insight"
- finding: "${1:Insight finding}"
  impact: "${2:Impact description}"
  solution: "${3:Proposed solution}"
  confidence: ${4|low,medium,high|}
  evidence: [${5:evidence1, evidence2}]
  related_files: [${6:file1, file2}]
  category: ${7|performance,security,architecture,user_experience|}
  recorded_at: "${8:`strftime('%Y-%m-%dT%H:%M:%SZ')`}"
endsnippet

snippet gacp-decision "GACP Decision"
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

Configure Git hooks for GACP validation:

#### Pre-commit Hook
Create a `.git/hooks/pre-commit` file:

```bash
#!/bin/sh
# GACP Pre-commit Hook

echo "Running GACP validation..."

# Run GACP validation
if command -v gacp >/dev/null 2>&1; then
    if ! gacp validate --recursive; then
        echo "GACP validation failed. Please fix issues before committing."
        exit 1
    fi
    echo "GACP validation passed."
else
    echo "GACP CLI not found. Skipping validation."
fi
```

Make it executable:
```bash
chmod +x .git/hooks/pre-commit
```

## Custom Commands and Functions

### 1. GACP Commands

Add these to your `.vimrc`:

```vim
" GACP commands
command! GacpInit :!gacp init
command! GacpValidate :!gacp validate --recursive
command! GacpHealth :!gacp health
command! GacpScopes :!gacp scopes
command! GacpQuery :call GacpQuery()

" GACP query function
function! GacpQuery()
  let query = input('GACP Query: ')
  if query != ''
    execute '!gacp query "' . query . '"'
  endif
endfunction
```

### 2. File Navigation

Add these mappings to your `.vimrc`:

```vim
" GACP file navigation
nnoremap <leader>gg :e .gacp/gacp.yaml<CR>
nnoremap <leader>gk :e .gacp/knowledge.yaml<CR>
nnoremap <leader>gt :e .gacp/todos.yaml<CR>
nnoremap <leader>gd :e .gacp/decisions.yaml<CR>
nnoremap <leader>gp :e .gacp/patterns.yaml<CR>
nnoremap <leader>gc :e .gacp/conventions.yaml<CR>

" GACP commands
nnoremap <leader>gi :GacpInit<CR>
nnoremap <leader>gv :GacpValidate<CR>
nnoremap <leader>gh :GacpHealth<CR>
nnoremap <leader>gs :GacpScopes<CR>
nnoremap <leader>gq :GacpQuery<CR>
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
" Async GACP commands (Neovim)
if has('nvim')
  function! GacpValidateAsync()
    call jobstart(['gacp', 'validate', '--recursive'], {
      \ 'on_exit': function('GacpValidateCallback')
      \ })
  endfunction

  function! GacpValidateCallback(job_id, data, event)
    if a:event == 'exit'
      if a:data == 0
        echo "GACP validation passed"
      else
        echo "GACP validation failed"
      endif
    endif
  endfunction

  command! GacpValidateAsync :call GacpValidateAsync()
endif
```

## Best Practices

### 1. Regular Context Maintenance

- Run `gacp validate --recursive` before commits
- Update knowledge files when discovering new insights
- Record decisions as they're made, not after the fact
- Keep todos current and accurate

### 2. Vim/Neovim Specific

- Use `:GacpValidate` before saving important changes
- Leverage Vim's powerful text editing for GACP files
- Use macros for repetitive GACP file operations
- Take advantage of Vim's search and replace capabilities

### 3. Team Coordination

- Commit GACP files with related code changes
- Use GACP context in code reviews
- Share insights and decisions through GACP files
- Use cross-scope queries for project-wide coordination

## Troubleshooting

### Common Issues

1. **YAML validation errors**: Ensure your GACP files follow the schema in `schemas/gacp.json`
2. **Missing context**: Run `gacp health` to check scope completeness
3. **Plugin not working**: Check plugin installation and configuration
4. **Schema not loading**: Check that `schemas/gacp.json` path is correct
5. **Language server issues**: Ensure COC.nvim or ALE is properly configured

### Getting Help

- Run `gacp --help` for command documentation
- Check the [GACP README](../README.md) for protocol details
- Use `gacp validate --recursive` to identify issues
- Review the [protocol schemas](../schemas/) for file formats
- Check Vim/Neovim documentation and plugin help

## Next Steps

1. **Initialize your first scope**: `gacp init`
2. **Explore existing context**: `gacp scopes` and `gacp query`
3. **Start recording knowledge**: Use `gacp insight record`
4. **Set up team workflows**: Share GACP practices with your team
5. **Integrate with CI/CD**: Add GACP validation to your build pipeline

For more advanced usage, see the [GACP CLI Reference](../README.md#cli-command-reference), [Protocol Documentation](../schemas/), and [Rust Development Setup](../development/rust-setup.md). 