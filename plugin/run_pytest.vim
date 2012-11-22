if exists("loaded_runpytest")
  finish
endif
let loaded_runpytest = 1

let t:pytest_latest_test = ""

function! RunPyTest()
    " Look for run_pytest.py in runtimepath
python << EOF
import vim
import os
paths = vim.eval('&runtimepath').split(",")
for path in paths:
    run_pytest_py = os.path.join(path, "python", "run_pytest.py")
    if os.path.exists(run_pytest_py):
        vim.command(':pyfile %s' % run_pytest_py)

EOF
endfunction
