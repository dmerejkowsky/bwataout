My dotfiles
===========

You can find here the configuration for most of the tools I use.

The ``install.py`` is meant to be run only once, on a newly created account.
It does not try to be clever and only write files if they don't already exist,
so it's safe.

It also tries to keep things in XDG directories (``~/.config``, ``~/.local``)
and so on.

You can run it with ``Python2`` or ``Python3``

Use::

  python install.py --help

to see available options
