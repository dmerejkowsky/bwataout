" ugly hack to keep comments indented properly
:inoremap # X#

function! UseMypy(module)
  let &errorformat =
    \ '%f:%l:%c:%t:%m,' .
    \ '%f:%l:%t:%m'
  let &makeprg='mypy --strict --ignore-missing-imports  --show-column-numbers ' . a:module
endfunction

command! -nargs=1 -complete=dir UseMypy :call UseMypy(<f-args>)
