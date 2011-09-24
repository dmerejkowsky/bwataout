Vim conf
========

A simple system to handle vim configuration.

Usage::

  vim-conf-install VIMCONF_FORK_URL

VIMCONF_FORK_URL should be a fork of

https://yannicklm/vimconf.git

containing at least::

   vimrc
   vimconf.cfg


This command will:

* backup existing .vim and .vimrc

* get the vimrc from the git repo and install it as new .vimrc

* install pathogen

* get the vimconf.cfg and parse it to:

  * checkout every plugin mentioned in the config file using pathogen

More can be done like:

* getting a colorscheme and install it

* getting a list of snippets for snippetEmy and install them

...



