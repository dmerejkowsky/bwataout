define-command cwd-jump -hidden %{
  execute-keys 'x H <esc>: cd <c-r>. <ret> : e '
}

define-command cwd-history -docstring "open a buffer containing list of recent working dirs" %{
  execute-keys ': edit -scratch *cwd-history* <ret>
  % <ret>
  ! cwd-history list --reversed <ret>
  gg
  '
  hook buffer -group cwd-history NormalKey <ret> cwd-jump
}

hook global ClientClose .* %{ nop %sh{ cwd-history add $(pwd) } }
