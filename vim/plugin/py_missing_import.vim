if exists("loaded_py_missing_import")
  finish
endif
let loaded_py_missing_import=1

function! AddMissingImport(module)
  let i = 1
  let last_import = 0
  let lines = getline(1, '.')
  for line in lines
    if line =~ "import"
      let last_import = i
    endif
    if line =~ 'import\s\+' . a:module
      return
    endif
    let i = i + 1
  endfor
  let orig_line = line('.')
  let orig_col  = line('.')
  call cursor(last_import, 1)
  call append('.', "import " . a:module)
  call cursor(orig_line+1, orig_col)
endfunction

