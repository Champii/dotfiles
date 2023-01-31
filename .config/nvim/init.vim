set mouse=a
set clipboard+=unnamedplus
set termguicolors
set number relativenumber
colorscheme doom-two
set shiftwidth=4
set expandtab
let mapleader = "\<Space>"
set nowrap
set cursorline
set noshowmode
set cmdheight=0

let g:dashboard_default_executive = 'fzf'

lua require 'pii'

" au BufRead,BufNewFile *.rk		set filetype=haskell
" autocmd FileType haskell set tabstop=2|set shiftwidth=2|set expandtab

