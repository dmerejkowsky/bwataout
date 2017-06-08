# My dotfiles

You can find here the configuration for most of the tools I use.


Usage
-----

* Run `./bootstrap.sh` to install Python dependencies
* Run `python3 install.py`

Notes
-----

The ``install.py`` is meant to be run only once, on a newly created account.
It does not try to be clever and only write files if they don't already exist,
so it's safe.

It also tries to keep things in XDG directories (``~/.config``, ``~/.local``)
and so on.

It uses tricks so that updating the config files work by just pulling this repo.
