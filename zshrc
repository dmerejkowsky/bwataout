##
# zsh config
#
# Taken from github.com/cgestes/ctafconf.git


# Shell is non-interactive.  Be done now
if [[ $- != *i* ]] || [ a$HOST = agate-ssh ]; then
    return
fi


# Options {{{
setopt prompt_subst             # Need this so the prompt will work.

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
# }}}
# }}}
# Aliases {{{

alias psgrep='ps aux | grep -v grep | grep'

# Better safe than sorry
alias mv="mv -i"
alias rm="rm -i"
alias cp="cp -i"

alias vi="vim"
# }}}
# Title bar handling {{{

function set_title {
    a=${(V)1//\%/\%\%}
    a=$(print -Pn "%40>...>$a" | tr -d "\n")

    case $TERM in
    screen*)
      print -Pn "\ek$a:$3\e\\"      # screen title (in ^A")
      ;;
    xterm*|rxvt)
      print -Pn "\e]2;$2 | $a:$3\a" # plain xterm title
      ;;
    esac
}

function title_precmd {
    set_title "zsh" "$USER@%m" "%55<...<%~"
}

function title_preexec {
    set_title "$1" "$USER@%m" "%35<...<%~"
}

autoload -U add-zsh-hook
add-zsh-hook precmd title_precmd
add-zsh-hook preexec title_preexec

# }}}
# Misc {{{
autoload run-help

bindkey '^[[A' history-beginning-search-backward
bindkey '^[[B' history-beginning-search-forward
# }}}
# Prompt {{{
autoload -U promptinit
promptinit

this_dir=$(readlink -f $(dirname $0))
source "${this_dir}/contrib/prompt_ctaf"
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

export PATH="$HOME/.local/bin:$PATH"

# Prevent fork bomb
limit maxproc 1042 2>/dev/null
# vim: set foldmethod=marker:
# }}}
