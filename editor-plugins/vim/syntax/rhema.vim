" Rhema Syntax Highlighting
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

" Clear existing syntax
syntax clear

" Rhema-specific keywords
syntax keyword rhemaKeyword context scope todos insights patterns decisions conventions
syntax keyword rhemaKeyword knowledge dependencies impact analysis
syntax keyword rhemaKeyword service app library tool other
syntax keyword rhemaKeyword pending in-progress completed cancelled
syntax keyword rhemaKeyword low medium high
syntax keyword rhemaKeyword proposed approved rejected implemented deprecated
syntax keyword rhemaKeyword recommended required optional deprecated
syntax keyword rhemaKeyword query validate sync generate refactor
syntax keyword rhemaKeyword add list update delete complete record
syntax keyword rhemaKeyword hooks workflow automation monitoring security
syntax keyword rhemaKeyword health debug profile stats documentation

" Rhema functions and commands
syntax keyword rhemaFunction init show search validate scopes tree
syntax keyword rhemaFunction todo insight pattern decision
syntax keyword rhemaFunction dependencies impact sync-knowledge git
syntax keyword rhemaFunction stats health debug profile refactor generate docs

" Rhema data types and structures
syntax keyword rhemaType string integer float boolean array object
syntax keyword rhemaType yaml json table count
syntax keyword rhemaType scope-type scope-name auto-config
syntax keyword rhemaType title description priority assignee due-date
syntax keyword rhemaType status outcome confidence category tags
syntax keyword rhemaType pattern-type usage effectiveness examples anti-patterns
syntax keyword rhemaType context makers alternatives rationale consequences

" Rhema configuration options
syntax keyword rhemaConfig recursive json-schema migrate
syntax keyword rhemaConfig format stats in-file regex
syntax keyword rhemaConfig --scope --format --stats --recursive
syntax keyword rhemaConfig --json-schema --migrate --in-file --regex

" Comments
syntax match rhemaComment "#.*$"

" Strings
syntax region rhemaString start='"' end='"'
syntax region rhemaString start="'" end="'"

" Numbers
syntax match rhemaNumber "\d\+"
syntax match rhemaNumber "\d\+\.\d\+"

" YAML structure
syntax match rhemaKey "^[[:space:]]*[a-zA-Z_][a-zA-Z0-9_]*:"
syntax match rhemaAnchor "&[a-zA-Z_][a-zA-Z0-9_]*"
syntax match rhemaAlias "\*[a-zA-Z_][a-zA-Z0-9_]*"

" Rhema-specific patterns
syntax match rhemaPattern "pattern:"
syntax match rhemaDecision "decision:"
syntax match rhemaInsight "insight:"
syntax match rhemaTodo "todo:"
syntax match rhemaConvention "convention:"

" CQL queries
syntax region rhemaQuery start="query:" end="$"
syntax region rhemaQuery start="SELECT" end=";"
syntax region rhemaQuery start="FROM" end="WHERE"

" Links and references
syntax match rhemaLink "https\?://[^\s]*"
syntax match rhemaReference "@[a-zA-Z_][a-zA-Z0-9_]*"

" Error and warning indicators
syntax match rhemaError "ERROR:"
syntax match rhemaError "FAILED:"
syntax match rhemaWarning "WARNING:"
syntax match rhemaWarning "DEPRECATED:"

" Success indicators
syntax match rhemaSuccess "SUCCESS:"
syntax match rhemaSuccess "OK:"
syntax match rhemaSuccess "VALID:"

" Timestamps and dates
syntax match rhemaDate "\d\{4}-\d\{2}-\d\{2}"
syntax match rhemaTime "\d\{2}:\d\{2}:\d\{2}"

" UUIDs and IDs
syntax match rhemaId "[a-f0-9]\{8}-[a-f0-9]\{4}-[a-f0-9]\{4}-[a-f0-9]\{4}-[a-f0-9]\{12}"
syntax match rhemaId "ID: [a-zA-Z0-9_-]\+"

" Performance and metrics
syntax match rhemaMetric "\d\+ms"
syntax match rhemaMetric "\d\+s"
syntax match rhemaMetric "\d\+%"

" Highlighting groups
highlight default link rhemaKeyword Keyword
highlight default link rhemaFunction Function
highlight default link rhemaType Type
highlight default link rhemaConfig PreProc
highlight default link rhemaComment Comment
highlight default link rhemaString String
highlight default link rhemaNumber Number
highlight default link rhemaKey Identifier
highlight default link rhemaAnchor Special
highlight default link rhemaAlias Special
highlight default link rhemaPattern Statement
highlight default link rhemaDecision Statement
highlight default link rhemaInsight Statement
highlight default link rhemaTodo Statement
highlight default link rhemaConvention Statement
highlight default link rhemaQuery Special
highlight default link rhemaLink Underlined
highlight default link rhemaReference Special
highlight default link rhemaError Error
highlight default link rhemaWarning WarningMsg
highlight default link rhemaSuccess String
highlight default link rhemaDate Constant
highlight default link rhemaTime Constant
highlight default link rhemaId Special
highlight default link rhemaMetric Number

" Custom highlighting for Rhema-specific elements
highlight rhemaKeyword ctermfg=blue guifg=#0066cc
highlight rhemaFunction ctermfg=green guifg=#00cc00
highlight rhemaPattern ctermfg=magenta guifg=#cc00cc
highlight rhemaDecision ctermfg=cyan guifg=#00cccc
highlight rhemaInsight ctermfg=yellow guifg=#cccc00
highlight rhemaTodo ctermfg=red guifg=#cc0000
highlight rhemaConvention ctermfg=white guifg=#ffffff
highlight rhemaQuery ctermfg=green guifg=#00cc00 