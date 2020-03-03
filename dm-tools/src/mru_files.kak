define-command mru-jump -hidden %{
  execute-keys -with-hooks 'x H gf <ret>'
}

define-command mru -docstring "open a buffer containing list of recently opened files" %{
  execute-keys ': edit -scratch *mru-files* <ret>
  % <ret>
  ! mru-files list --reversed <ret>
  gg
  '
  hook buffer -group mru-hooks NormalKey <ret> mru-jump
}
