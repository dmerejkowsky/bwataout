" Special definitions for python files

setlocal omnifunc=pythoncomplete#Complete


let s:py_version = system("python --version")
let s:py_version = strpart(s:py_version, 7) " Remove 'Python '
let s:py_version = strpart(s:py_version, 0, 3) " Keep only the major version

let s:py_path="/usr/lib/python" . s:py_version .
  \ "," . "/usr/lib/python" . s:py_version . "/site-packages"

let &path=&path . "," . s:py_path

" This is incredibly helpful...
abbreviate <buffer> sefl self
abbreviate <buffer> slef self


" Indent with 4 spaces
setlocal expandtab
setlocal smarttab
setlocal shiftwidth=4
setlocal tabstop=4
setlocal softtabstop=4

" Don't use smartindent, it messes up the 3rd part indent
" script
setlocal nosmartindent

""
" Using custom python plugins in plugins/py.*.vim

" Add an "import" line using work under cursor
nmap <buffer> <leader>I :call AddMissingImport('<C-R><C-W>') <CR>

" Run pytest
nmap <buffer> <leader>k :call vimpytest#run()<CR>

" Switch between test and production code
command! -nargs=0 A :call vimpytest#alternate()

function! SetPythonPath()
  let l:cwd = getcwd()
  let l:pythonpath = l:cwd . ":" . $PYTHONPATH
  let $PYTHONPATH = l:pythonpath
endfunction

command! -nargs=0 SetPythonPath :call SetPythonPath()

function! UsePylint()
  set makeprg=pylint\ --reports=n\ --errors-only\ --output-format=parseable\ %:p
  set errorformat=%f:%l:\ %m
endfunction

command! -nargs=0 UsePylint :call UsePylint()

nnoremap <buffer> <leader>d :bufdo :g/debug_here/d <bar> update<cr>
