if exists("loaded_py_alternate")
  finish
endif
let loaded_py_alternate = 1

function! PyAlternate()
    " Look for alternate.py in runtimepath
python << EOF
import vim
import os
paths = vim.eval('&runtimepath').split(",")
for path in paths:
    alternate_py = os.path.join(path, "python", "alternate.py")
    if os.path.exists(alternate_py):
        vim.command(':pyfile %s' % alternate_py)

EOF
endfunction
