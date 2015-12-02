#!/usr/bin/python

# A simple script to get a vim configuration
# from a git repository

from __future__ import print_function

import os
import sys
import posixpath

import subprocess
import requests

VIMRC_TEMPLATE = """\
" Auto-generated code. Do not edit
source {vimrc}
source {vimrclocal}
"""
NVIM_CONF_DIR = os.path.expanduser("~/.config/nvim")
INIT_VIM = os.path.join(NVIM_CONF_DIR, "init.vim")
VIMPLUG_URL = "https://raw.githubusercontent.com/junegunn/vim-plug/0.7.2/plug.vim"

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
    """ Backup previous nvim configuration """
    if os.path.exists(INIT_VIM):
        backup = get_backup_name(INIT_VIM)
        os.rename(INIT_VIM, backup)
        print(INIT_VIM, "backuped to", backup)

def fetch_plug_vim(dest):
    data = requests.get(VIMPLUG_URL)
    with open(dest, "w") as fp:
        fp.write(data.text)

def create_vimrc_files():
    backup_conf()

    vimrclocal = os.path.join(NVIM_CONF_DIR, "vimrc.local")
    if not os.path.exists(vimrclocal):
        with open(vimrclocal, "w") as fp:
            fp.write('" Put your local settings here\n"')

    this_dir = os.path.dirname(__file__)
    vimrc_src = os.path.join(this_dir, "..", "vimrc")
    vimrc_src = os.path.abspath(vimrc_src)
    to_write = VIMRC_TEMPLATE.format(vimrc=vimrc_src,
                                     vimrclocal=vimrclocal)

    vimrc_dest = os.path.expanduser(INIT_VIM)
    with open(vimrc_dest, "w") as fp:
        fp.write(to_write)

def main():
    """ Main entry point

    """
    nvim_conf_dir = os.path.dirname(INIT_VIM)
    autoload = os.path.join(nvim_conf_dir, "autoload")
    mkdir_p(autoload)
    vim_plug = os.path.join(autoload, "plug.vim")
    fetch_plug_vim(vim_plug)
    backup_conf()
    create_vimrc_files()

if __name__ == "__main__":
    main()
