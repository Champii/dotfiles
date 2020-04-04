" General
set number	" Show line numbers
set showmatch	" Highlight matching brace
set visualbell	" Use visual bell (no beeping)

set hlsearch	" Highlight all search results
set smartcase	" Enable smart-case search
set ignorecase	" Always case-insensitive
set incsearch	" Searches for strings incrementally

set nowrap
set autoindent	" Auto-indent new lines
" set shiftwidth=4	" Number of auto-indent spaces
set smartindent	" Enable smart-indent
" set smarttab	" Enable smart-tabs
" set softtabstop=2	" Number of spaces per Tab
set tabstop=8 softtabstop=0 expandtab shiftwidth=4 smarttab

set smartcase
set ruler
set cursorline
set background=dark
set autoread

set undolevels=1000	" Number of undo levels
set backspace=indent,eol,start	" Backspace behavior
set spell spelllang=en_us
set spell!

set ttyfast
set relativenumber
set undofile

let mapleader = ","

set clipboard=unnamedplus

"" Generated by VimConfig.com

call plug#begin('~/.config/nvim/plugged')

Plug 'terryma/vim-multiple-cursors'

Plug 'tpope/vim-fugitive'

Plug 'StanAngeloff/php.vim'

Plug 'autozimu/LanguageClient-neovim', {
    \ 'branch': 'next',
    \ 'do': 'bash install.sh',
    \ }

" (Optional) Multi-entry selection UI.
Plug 'junegunn/fzf', { 'do': './install --bin' }
Plug 'junegunn/fzf.vim'

" Use release branch (Recommend)
Plug 'neoclide/coc.nvim', {'branch': 'release'}

Plug 'scrooloose/nerdtree'

Plug 'Xuyuanp/nerdtree-git-plugin'

Plug 'timonv/vim-cargo'

Plug 'joshdick/onedark.vim'

Plug 'liuchengxu/vim-which-key'

Plug 'tpope/vim-surround'

Plug 'easymotion/vim-easymotion'

" Plug 'frazrepo/vim-rainbow'
Plug 'luochen1990/rainbow'

Plug 'mattn/emmet-vim'

" Plug 'ctrlpvim/ctrlp.vim'

Plug 'vim-airline/vim-airline'

Plug 'airblade/vim-gitgutter'

Plug 'camspiers/animate.vim'

" Plug 'camspiers/lens.vim'

Plug 'AndrewRadev/undoquit.vim'

Plug 'vim-syntastic/syntastic'

Plug 'vim-scripts/taglist.vim'

Plug 'preservim/nerdcommenter'

Plug 'jiangmiao/auto-pairs'

Plug 'mileszs/ack.vim'

" Plug 'nathanaelkane/vim-indent-guides'
Plug 'Yggdroot/indentLine'

Plug 'rust-lang/rust.vim'

Plug 'tpope/vim-obsession'

Plug 'majutsushi/tagbar'

Plug 'kassio/neoterm'

Plug 'glacambre/firenvim', { 'do': { _ -> firenvim#install(0) } }

Plug 'brettanomyces/nvim-editcommand'

Plug 'Vigemus/nvimux'

Plug 'leafgarland/typescript-vim'

Plug 'pangloss/vim-javascript'

Plug 'prettier/vim-prettier', { 'do': 'npm install' }

Plug 'ntpeters/vim-better-whitespace'

Plug 'vim-scripts/matchit.zip'

Plug 'mbbill/undotree'

Plug 'andrewradev/splitjoin.vim'

Plug 'mhinz/vim-startify'

Plug 'junegunn/goyo.vim'

Plug 'pechorin/any-jump.vim'

Plug 'mboughaba/i3config.vim'

Plug 'termhn/i3-vim-nav'

" Always last
Plug 'ryanoasis/vim-devicons'

call plug#end()

source ~/.config/nvim/coc.vim

nnoremap <esc> :noh<return><esc>

nmap <space>e :NERDTreeToggle<CR>
"map <C-n> :NERDTreeToggle<CR>

let g:mapleader = "\<Space>"
let g:maplocalleader = ','
nnoremap <silent> <leader>      :<c-u>WhichKey '<Space>'<CR>
nnoremap <silent> <localleader> :<c-u>WhichKey  ','<CR>

map <up> <nop>
map <down> <nop>
map <left> <nop>
map <right> <nop>

" Disable Arrow keys in Insert mode
imap <up> <nop>
imap <down> <nop>
imap <left> <nop>
imap <right> <nop>


inoremap <C-h> <Left>
inoremap <C-j> <Down>
inoremap <C-k> <Up>
inoremap <C-l> <Right>
cnoremap <C-h> <Left>
cnoremap <C-j> <Down>
cnoremap <C-k> <Up>
cnoremap <C-l> <Right>

colorscheme onedark

map <M-l> :vertical resize +5<CR>
map <M-h> :vertical resize -5<CR>
map <M-j> :res -5<CR>
map <M-k> :res +5<CR>

let g:EasyMotion_do_mapping = 0 " Disable default mappings

" Jump to anywhere you want with minimal keystrokes, with just one key binding.
" `s{char}{label}`
nmap s <Plug>(easymotion-overwin-f)

" Turn on case-insensitive feature
let g:EasyMotion_smartcase = 1

" JK motions: Line motions
map <Leader>j <Plug>(easymotion-j)
map <Leader>k <Plug>(easymotion-k)

" Rainbow
let g:rainbow_active = 1
let g:rainbow_conf = {
\   'guifgs': ['red', 'green', 'cyan', 'magenta'],
\   'ctermfgs': ['darkblue', 'darkmagenta', 'darkcyan', 'darkred', 'darkgreen'],
\ }

" Airline
let g:airline_powerline_fonts = 1
let g:airline#extensions#tabline#enabled = 1
let g:airline#extensions#tagbar#enabled = 1

" Buffer nav
nnoremap <M-[> :bprevious<CR>
nnoremap <M-]> :bnext<CR>

" GitGutter
let g:gitgutter_highlight_lines = 0
let g:gitgutter_highlight_linenrs = 0

" Syntastic
set statusline+=%#warningmsg#
set statusline+=%{SyntasticStatuslineFlag()}
set statusline+=%*

let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_wq = 0

" NerdComment

" Add spaces after comment delimiters by default
let g:NERDSpaceDelims = 1

" Use compact syntax for prettified multi-line comments
let g:NERDCompactSexyComs = 1

" Align line-wise comment delimiters flush left instead of following code indentation
let g:NERDDefaultAlign = 'left'

" Add your own custom formats or override the defaults
let g:NERDCustomDelimiters = { 'c': { 'left': '/**','right': '*/' } }

" Allow commenting and inverting empty lines (useful when commenting a region)
let g:NERDCommentEmptyLines = 1

" Enable trimming of trailing whitespace when uncommenting
let g:NERDTrimTrailingWhitespace = 1

" Enable NERDCommenterToggle to check all selected lines is commented or not
let g:NERDToggleCheckAllLines = 1

" FZF
" command! -bang -nargs=? -complete=dir Files
    " \ call fzf#vim#files(<q-args>, fzf#vim#with_preview({'options': ['--layout=default', '--preview', '--info=inline']}), <bang>0)
command! -bang -nargs=? -complete=dir Files
    \ call fzf#vim#files(<q-args>, {'options': ['--layout=default', '--info=inline', '--preview', 'bat --theme TwoDark --style=numbers --color=always {} | head -500', '--height=10%']}, <bang>0)

" Indent
let g:indent_guides_enable_on_vim_startup = 1
let g:indentLine_char_list = ['|', '¦', '┆', '┊']

map <C-p> :Files<CR>

nnoremap <F12> :Ttoggle<cr><C-w><C-w>A
inoremap <F12> <esc>:Ttoggle<cr><C-w><C-w>A
tnoremap <F12> <C-\><C-n>:Ttoggle<cr>
tnoremap <esc> <C-\><C-n>
tnoremap <C-w><C-w> <C-\><C-n><C-w><C-w>

" TagBar
nmap <F8> :TagbarToggle<CR>

" Custom
" nnoremap <C-c> :set insert

set guifont=Delugia\ Nerd\ Font:h7
" set guifont=xos4\ Terminess\ Powerline:h6
let g:neovide_cursor_vfx_mode = "railgun"


let g:firenvim_config = {
	\ "globalSettings": {
		\ "server": "persistent"
	\}
\}

let g:editcommand_prompt = '❯'

lua require('nvimux').bootstrap()

  " let g:nvimux_prefix = '<C-a>'
let g:nvimux_open_term_by_default = 'true'
let g:nvimux_new_window = 'single'

" Spell
nnoremap <leader>f 1z=
nnoremap <leader>s :set spell!<CR>
nnoremap <leader>d zg

" Windows
nnoremap <leader>w <C-w>v<C-w>l
nnoremap <C-h> <C-w>h
nnoremap <C-j> <C-w>j
nnoremap <C-k> <C-w>k
nnoremap <C-l> <C-w>l

" Elixir fix
au BufRead,BufNewFile *.ex,*.exs set filetype=elixir
au BufRead,BufNewFile *.eex set filetype=eelixir

" Syntastic
let g:syntastic_mode_map = { 'mode': 'active',
                            \ 'active_filetypes': ['python', 'javascript'],
                            \ 'passive_filetypes': [] }

set statusline+=%#warningmsg#
set statusline+=%{SyntasticStatuslineFlag()}
set statusline+=%*

let g:syntastic_always_populate_loc_list = 1
let g:syntastic_auto_loc_list = 1
let g:syntastic_check_on_open = 1
let g:syntastic_check_on_wq = 0
let g:syntastic_javascript_checkers = ['eslint']

command! -bang -nargs=* Find call fzf#vim#grep('rg --column --line-number --no-heading --fixed-strings --ignore-case --no-ignore --hidden --follow --glob "!.git/*" --color "always" '.shellescape(<q-args>), 1, <bang>0)

" Neoterm
let g:neoterm_size = 30
let g:neoterm_autoinsert = 1
let g:neoterm_default_mod = ':rightbelow'

" Whitespace
let g:better_whitespace_enabled=1
let g:strip_whitespace_on_save=1
let g:strip_whitespace_confirm=0
let g:strip_only_modified_lines=1

" UndoTree
nnoremap <F5> :UndotreeToggle<cr>

map  <F13> <M-f>
map! <F13> <M-f>

" i3 integration
nnoremap <silent> <f13-l> :call Focus('right', 'l')<CR>
nnoremap <silent> <c-h> :call Focus('left', 'h')<CR>
nnoremap <silent> <c-k> :call Focus('up', 'k')<CR>
nnoremap <silent> <c-j> :call Focus('down', 'j')<CR>
