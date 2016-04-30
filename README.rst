dmerej Vim / Neovim configuration
==================================

Requirements:  ``python2`` or ``python3``

Preferably used with ``Neovim``, but can also work with ``vim``

Usage::

  python bin/get-vimconf

Or, if you don't want (or can't) use Neovim::

  python bin/get-vimconf --enable-vim


This command will:

* Install ``vim-plug`` (https://github.com/junegunn/vim-plug)
* Install this ``vimrc``
* If used with ``--enable-vim``, will create required
  ``~/.vim*`` files for ``vim`` to work
* If not, will create a nice wrapper in ``~/.local/bin/vim``
  to bring back the ``--remote`` option and set ``NVIM_LISTEN_ADDRESS``
