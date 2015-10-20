#!/usr/bin/python

# A simple script to get a vim configuration
# from a git repository

from __future__ import print_function

import os
import sys
import posixpath

import subprocess

ON_WIN = sys.platform.startswith("win")

VIM_BUNDLE = os.path.expanduser("~/.vim/bundle")

VIMRC_TEMPLATE = """\
" Auto-generated code. Do not edit
source {vimrc}
source {vimrclocal}
"""


def mkdir_p(dest_dir):
    """ Contrary to os.makedirs, this
    wont't fail if dest already exists

    """
    if os.path.exists(dest_dir):
        return
    os.makedirs(dest_dir)

def get_backup_name(filename):
  id = 0
  while True:
    backup = "%s.%d" % (filename, id)
    id += 1
    if not os.path.exists(backup):
      return backup

def backup_conf():
    """ Backup vim configuration

    """
    if ON_WIN:
        vimrc = os.path.expanduser(r"~\_vimrc")
    else:
        vimrc = os.path.expanduser("~/.vimrc")
    if os.path.exists(vimrc):
        backup = get_backup_name(vimrc)
        os.rename(vimrc, backup)
        print(vimrc, "backuped to", backup)

def main():
    """ Main entry point

    """
    clone_neobundle()
    create_vimrc_files()

def clone_neobundle():
    mkdir_p(VIM_BUNDLE)
    readme_md = os.path.join(VIM_BUNDLE, "neobundle.vim", "README.md")
    if not os.path.exists(readme_md):
        cmd = ["git", "clone", "https://github.com/Shougo/neobundle.vim"]
        subprocess.check_call(cmd, cwd=VIM_BUNDLE)

def create_vimrc_files():
    backup_conf()

    # Create ~/.vimrc.local
    vimrclocal = os.path.expanduser("~/.vimrc.local")
    if not os.path.exists(vimrclocal):
        with open(vimrclocal, "w") as fp:
            fp.write('" Put your local settings here\n"')

    # Copy the vimrc file in this repo
    this_dir = os.path.dirname(__file__)
    vimrc_src = os.path.join(this_dir, "..", "vimrc")
    vimrc_src = os.path.abspath(vimrc_src)
    to_write = VIMRC_TEMPLATE.format(vimrc=vimrc_src,
                                     vimrclocal=vimrclocal)

    vimrc_dest = os.path.expanduser("~/.vimrc")
    with open(vimrc_dest, "w") as fp:
        fp.write(to_write)


if __name__ == "__main__":
    main()
