##
# zsh config
#
# Taken from github.com/cgestes/ctafconf.git


# Shell is non-interactive.  Be done now
if [[ $- != *i* ]] || [ a$HOST = agate-ssh ]; then
    return
fi

# Options {{{
setopt correct

setopt equals
setopt magic_equal_subst

setopt numericglobsort
setopt extendedglob             # weird & wacky pattern matching - yay zsh!
setopt no_nomatch               # but when pattern matching fails, simply use the command as is

setopt interactivecomments      # date # comment will work

# Completion {{{
autoload -U compinit
compinit

# Filename suffixes to ignore during completion
fignore=(.o .pyc __pycache__)
setopt list_ambiguous           # complete as much of a completion until it gets ambiguous.
setopt norecexact               # recognise exact match even if there are ambiguous matches
setopt complete_aliases
setopt completeinword           # not just at the end
setopt no_always_to_end         # when complete from middle, move cursor

#Menu
setopt no_menu_complete		# jump through menus
setopt automenu			# don't cycle completions
setopt auto_list                # display a list of completion
setopt auto_param_keys
setopt auto_param_slash         # auto add slash when completing path
setopt auto_remove_slash

# }}}

# History handling {{{
setopt append_history           # append to history
setopt inc_append_history       # append immediately
setopt hist_expire_dups_first   # expire duplicates in history first
setopt hist_ignore_all_dups     # don't add dupes to history
setopt hist_verify              # when using ! cmds, confirm first
setopt hist_no_store            # dont add 'history' command (fc -l) to the history
# }}}

# Directory history (use +/- to navigate) {{{
setopt autopushd	      # automatically append dirs to the push/pop list
setopt pushd_minus	      # exchange meaning of + and - for pushd
setopt pushd_silent	      # don't tell me about automatic pushd
setopt pushd_to_home	      # use $HOME when no arguments specified
setopt pushd_ignoredups	      # ignore duplicates
setopt noautocd               # dont change to dirs without cd
alias -- +='pushd +0'
alias -- -='pushd -1'
# }}}

# }}}

# Aliases {{{

alias psgrep='ps aux | grep -v grep | grep'

alias cux='chmod +x'


# Better safe than sorry
alias mv="mv -iv"
alias rm="rm -i"
alias cp="cp -iv"

alias vi="vim"

# Open pdf files with ./path/to/pdf
alias -s pdf=evince
# }}}

# Pretty ls {{{
if [[ $(uname) == "Darwin" ]] ; then
# for mac
  export LSCOLORS='dxgxfxexcxegedabagacad'
  alias ls="ls -FG"
else
# for linux
  export LS_COLORS='no=00:fi=00:di=0;33:ln=01;36:pi=40;33:so=01;35:bd=40;33;01:cd=40;33;01:or=40;31;01:ex=01;32:*.tar=01;31:*.tgz=01;31:*.arj=01;31:*.taz=01;31:*.lzh=01;31:*.zip=01;31:*.z=01;31:*.Z=01;31:*.gz=01;31:*.bz2=01;31:*.deb=01;31:*.rpm=01;31:*.jpg=01;35:*.gif=01;35:*.bmp=01;35:*.pgm=01;35:*.pbm=01;35:*.ppm=01;35:*.tga=01;35:*.png=01;35:*.GIF=01;35:*.JPG=01;35:*.xbm=01;35:*.xpm=01;35:*.tif=01;35:*.mpg=01;37:*.avi=01;37:*.gl=01;37:*.dl=01;37:*.mly=01;37:*.mll=01;37:*.mli=01;37:*.ml=01;37:*.cpp=01;37:*.cc=01;37:*.c=01;37:*.hh=01;37:*.h=01;37:*Makefile=4;32:*.pl=4;32:*.sh=4;32:*.ps=4;34:*.pdf=4;36:*.txt=01;37:*.tex=01;37:*TODO=01;37'
  # Use the same escape as zsh completion when there are spaces
  # in file names (foo\ bar)
  alias ls="ls --classify --quoting-style=escape --color=auto"
fi

# }}}

# Misc {{{
autoload run-help

# Use emacs-mode
bindkey -e

bindkey '^[[A' history-beginning-search-backward
bindkey '^[[B' history-beginning-search-forward
# }}}

# Global settings {{{
export HISTSIZE=50000
export HISTFILE=~/.zshhistory
export SAVEHIST=10000
export BLOCK_SIZE=human-readable
# depth of the directory history
DIRSTACKSIZE=30

export VISUAL="vim"
export EDITOR="vim"

this_dir=$(dirname $0)
export PATH="${this_dir}/bin:$HOME/.local/bin:$PATH"

# required for pure-prompt
fpath=("$HOME/.local/share/zsh/functions" $fpath)

# Prevent fork bomb
limit maxproc 1042 2>/dev/null
# }}}

# Functions {{{

# cd /path/to/file -> cd /path/to
alias orig_cd=cd
function smart_cd {
  if [[ -z "$1" ]]; then
    orig_cd "$HOME"
  elif [[ -f "$1" ]]; then
    orig_cd $(dirname "$1")
  else
    orig_cd "$1"
  fi
}

alias cd=smart_cd

# go to a path relative to the git top dir
function gitcd {
  topdir=$(git rev-parse --show-toplevel)
  if [[ $? -ne 0 ]]; then
    return 1
  fi
  cd "${topdir}/${1}"
}

# create an archive with a sensible name
function gitar {
  toplevel=$(git rev-parse --show-toplevel)
  desc=$(git describe --always --tags)
  name="$(basename ${toplevel})-${desc}"
  output="${name}.tar.gz"
  git archive --prefix "${name}/" --output "${name}.tar.gz"  HEAD
  echo "archive generated in: ${output}"
}

# Open all the conflicting files in $EDITOR
function resolve() {
  (gitcd && git diff --name-only --diff-filter=U | xargs $EDITOR)
}

# remove all breakpoints from Python code
function rm-breakpoints {
  for file in $(git grep -l debug_here); do
    sed -i '/debug_here/d' $file
  done
}

# remove all nvim swap files
function rm-swap {
  rm -fr "$HOME/.local/share/nvim/swap"
}

# open the script you just run in $EDITOR
function vibin {
  # which is zsh is a builtin that works with aliases and functions,
  # prefer /usr/bin/which
  full_path=$(/usr/bin/which $1 2> /dev/null)
  if [[ $? -eq 0 ]] ; then
    $EDITOR ${full_path}
  else
    echo "$1 not found"
    return 1
  fi
}

# extract various archives given their names
function xt() {
 if [ -f "$1" ]; then
       case "$1" in
      *.7z)        7z x "$1"         ;;
      *.Z)         uncompress "$1"   ;;
      *.aar)       jar xf "$1"       ;;
      *.bz2)       bunzip2 "$1"      ;;
      *.jar)       jar xf "$1"       ;;
      *.rar)       unrar "$1"        ;;
      *.rar)       unrar x "$1"      ;;
      *.tar)       tar xvf "$1"      ;;
      *.tar.bz2)   tar xvjf "$1"     ;;
      *.tar.gz)    tar xvzf "$1"     ;;
      *.tbz)       tar xvjf "$1"     ;;
      *.tbz2)      tar xvjf "$1"     ;;
      *.tgz)       tar xvzf "$1"     ;;
      *.xz)        tar xvJf "$1"     ;;
      *.zip)       unzip "$1"        ;;
      # Must be after tar.gz
      *.gz)        gunzip "$1"       ;;
      *)           echo "'$1' cannot be extracted via extract" ;;
    esac
  else
    echo "'$1' is not a valid file"
  fi
}
# }}}

# Prompt {{{
autoload -U promptinit
promptinit

# It's not slow, it's just I don't mind having
# non-clean repos with untracked files
export PURE_GIT_UNTRACKED_DIRTY=0
prompt pure
# }}}

# vim: set foldmethod=marker:
