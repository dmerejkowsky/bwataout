define-command mru-jump -hidden %{
  execute-keys -client %val{client} -with-hooks x H gf
}

# Each open file is added to the list
hook global BufOpenFile .* %{ nop %sh{ mru-files add  "${kak_hook_param}" } }
# Each *created* file is added to the list ...
hook global BufNewFile .* %{ nop %sh{ mru-files add  "${kak_hook_param}" } }
# ... but we clean up buffers that were created but not written
# when kakoune stops
hook global KakEnd .* %{ nop %sh{ mru-files clean --max 100 } }

define-command mru -docstring "open a buffer containing list of recently opened files" %{
  execute-keys ': edit -scratch *mru-files* <ret>
  % <ret>
  ! mru-files list --reversed <ret>
  gg
  '
  hook buffer -group mru-hooks NormalKey <ret> mru-jump
}
