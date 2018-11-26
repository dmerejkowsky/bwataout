if !exists('loaded_snippet') || &cp
    finish
endif

Snippet if if(<{cond}>)<CR><{}><CR><BS>else()<CR><{}><CR><BS>endif()<CR><{}>

Snippet mess( message(STATUS "<{}>")
Snippet mv(   message(STATUS "<{var}>: ${<{var}>}")
