" Special settings for rst files

setlocal suffixesadd=.rst
setlocal linebreak
setlocal textwidth=80

" Special mapping for rst files
"
noremap <buffer> <leader>h1 yyp^v$r=o<ESC>
noremap <buffer> <leader>h2 yyp^v$r-o<ESC>
noremap <buffer> <leader>h3 yyp^v$r+o<ESC>
noremap <buffer> <leader>h4 yyp^v$r~o<ESC>
noremap <buffer> <leader>h5 yyp^v$r^o<ESC>
noremap <buffer> <leader>h5 yyp^v$r`o<ESC>

" pre is with ``
noremap <buffer> <leader>s Bi``<ESC>Ea``<ESC>
