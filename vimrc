" Yannick LM .vimrc

syntax on
filetype plugin on
set nocompatible
set history=10000
set mouse=a
set encoding=utf-8
set showcmd
set showmode
set ruler
set backspace=2
set virtualedit=block
set background=dark
set autowriteall

" nice menu to navigate through possible completions
set wildmenu
set wildmode=list:longest,full
" smarter behavior of 'J' (join lines)
set nojoinspaces

" highlight search
set hlsearch

" search as you type
set incsearch
" The correct way to remove highlight is:
" :let @/=""

" Tell NerdCommenter to shut up:
let g:NERDShutUp=1

" Prevent YankRing.vim from polluting $HOME:
let g:yankring_history_dir = expand('$HOME') . '/.vim'
if ! isdirectory(g:yankring_history_dir)
  call mkdir(g:yankring_history_dir, "p")
endif

" Allow completion on filenames right after a '='.
" Uber-useful when editing bash scripts
set isfname-==

" No more annoying bell
set visualbell t_vb=

" Disable useless ctrl+space behavior:
imap <Nul> <Space>

" Always display statusline
set laststatus=2

" Always load those useful plugins:
runtime! macros/matchit.vim
runtime! plugin/imaps.vim
runtime! ftplugin/man.vim

" If par is installed, just use it:
if executable("par")
  set formatprg=par
end

""
" A few useful functions:

" Remove trailing whitespace
function! CleanWhiteSpace()
  let _s=@/
  let l = line(".")
  let c = col(".")
  :%s/\s\+$//e
  let @/=_s
  call cursor(l, c)
endfunction()

command! -nargs=0 CleanWhiteSpace :call CleanWhiteSpace()

" Remove trailing whitespaces when saving:
autocmd bufwritepre * :CleanWhiteSpace

" Convert DOS line endings to UNIX line endings

function! FromDos()
  %s/\r//e
endfunction

command! FromDos call FromDos()

" Remove menu bar from GUI
let did_install_default_menus = 1

" I've always find it weird that it was not this way ...
set splitbelow

" More logical, but not vi-compatible
noremap Y y$

" Tabulations
set shiftwidth=2
set expandtab
set smarttab
set smartindent
set tabstop=4

" Nicer scrolling
set scroll=5
set scrolloff=2


" Status line (requires VimBuddy plugin to be present)
set statusline=%{VimBuddy()}\ [%n]\ %<%f\ %{fugitive#statusline()}%h%m%r%=%-14.(%l,%c%V%)\ %P\ %a


""
" Few keybindings

" 'cd' towards the dir in which the current file is edited
" but only change the path for the current window
map ,cd :lcd %:h<CR>

" I prefer , for mapleader rather than \
let mapleader=","

" Open files located in the same dir in with the current file is edited
map <leader>ew :e <C-R>=expand("%:p:h") . "/" <CR>
map <leader>es :sp <C-R>=expand("%:p:h") . "/" <CR>
map <leader>ev :vsp <C-R>=expand("%:p:h") . "/" <CR>
map <leader>et :tabe <C-R>=expand("%:p:h") . "/" <CR>

" Navigate through the buffer's list with alt+up, alt+down
nnoremap <M-Down>  :bp<CR>
nnoremap <M-Up>    :bn<CR>

" Man page for work under cursor
nnoremap K :Man <cword> <CR>
" Spell check
cmap spc setlocal spell spelllang=

" Bubble single lines
nmap <C-Up> ddkP
nmap <C-Down> ddp

" Bubble multiple lines
vmap <C-Up> xkP`[V`]
vmap <C-Down> xp`[V`]

" Call chmod +x on a file when necessary:
nmap <leader>! :call FixSheBang(@%) <CR>


""
" Misc

" I always make mistakes in these words
abbreviate swith switch
abbreviate wich which
abbreviate lauch launch
abbreviate MSCV MSVC


if has("gui_running")
  colorscheme wonbat
else
  "great, but does not keep a transparent background ...
  "colorscheme xoria256
endif


if has("gui_win32")
  " Only available on windows 7 ...
  set guifont=Consolas:h11:cANSI
endif


" Hard core settings:

" Disable arrows keys:
noremap   <Left>    <nop>
noremap   <Right>   <nop>
noremap   <Up>      <nop>
noremap   <Down>    <nop>

" Simpler way to go to normal mode from insert mode
inoremap jk <Esc>

" « it's one less key to hit every time I want to save a file »
"     -- Steve Losh (again)
nnoremap ; :
vnoremap ; :

