Yannick LM vim configuration
==============================

Requirements:  ``python2`` or ``python3``

Preferably used with ``neovim``, but can also work with ``vim``


Usage::

  python bin/get-vimconf

This command will:

* Install ``vim-plug`` (https://github.com/junegunn/vim-plug)
* Install this ``vimrc``
* If used with ``--enable-vim``, will create required
  ``~/.vim*`` files for ``vim`` to work
* If not, will create a nice wrapper in ``~/.local/bin/vim``
  to bring back the ``--remote`` option
