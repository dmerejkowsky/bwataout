Vim conf
========

A simple system to handle vim configuration.

Usage::

  get-vimconf VIMCONF_FORK_URL

VIMCONF_FORK_URL should be a fork of

https://yannicklm/vimconf.git

containing at least::

   vimrc
   vimconf.cfg


vimconf.cfg should at least contain this::

  [plugins]
  pathogen       = git://github.com/tpope/vim-pathogen.git


This command will:

* backup existing .vimrc, and a a new one to use pathogen,
  and the vimrc from this repo

* install pathogen

* get the vimconf.cfg and parse it to:

  * checkout every plugin mentioned in the config file using pathogen

Of course since vimconf is a bundle like any other one bundle,
you can add your syntax files in syntax/, your colorschemes in
colors/ and so on :)

