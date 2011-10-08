Vim conf
========

A simple system to handle vim configuration and plugins.

Usage::

  python bin/get-vimconf

This command will:

* backup existing .vimrc, and a a new one to use pathogen,
  and the vimrc from this repo

* install pathogen

* get the vimconf.cfg and parse it to

  * checkout every plugin mentioned in the config file using pathogen
  * build every plugin that need to be built

Forks welcome, nothing is more personal than a config file anyway.

Of course since vimconf is a bundle like any other one bundle,
you can add your syntax files in syntax/, your colorschemes in
colors/ and so on :)


