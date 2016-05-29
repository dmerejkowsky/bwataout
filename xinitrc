#!/bin/sh
#
# ~/.xinitrc
#
# Executed by startx (run your window manager from here)


if [ -d /etc/X11/xinit/xinitrc.d ]; then
  for f in /etc/X11/xinit/xinitrc.d/*; do
    [ -x "$f" ] && . "$f"
  done
  unset f
fi

syndaemon -i 1 -d
setxkbmap dvorak us-intl
setxkbmap -option ctrl:nocaps
/usr/bin/openbox-session
#exec i3
