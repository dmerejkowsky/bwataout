" Settings for shell scripts

setlocal makeprg=shellcheck\ -f\ gcc\ %
let errorformat =
        \ '%f:%l:%c: %trror: %m,' .
        \ '%f:%l:%c: %tarning: %m,' .
        \ '%f:%l:%c: %tote: %m'
