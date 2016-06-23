#!/bin/sh

if [ -d /etc/X11/xinit/xinitrc.d ]; then
  for f in /etc/X11/xinit/xinitrc.d/*; do
    [ -x "$f" ] && . "$f"
  done
  unset f
fi

syndaemon -i 1 -d
setxkbmap dvorak us-intl
setxkbmap -option ctrl:nocaps

. $HOME/.xinitrc.local
