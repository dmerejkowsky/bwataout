#!/usr/bin/python

# A simple script to get a vim configuration
# from a git repository
# Note that this only works for a fresh install

from __future__ import print_function

import argparse
import os
import sys
import posixpath

import subprocess
import urllib

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


def fetch_plug_vim(dest):
    if os.path.exists(dest):
        print("Plug.vim already installed, skipping")
        return
    print("Fetching vimplug ...")
    urllib.urlretrieve(VIMPLUG_URL, dest)

def create_vimrc_files():
    vimrclocal = os.path.join(NVIM_CONF_DIR, "vimrc.local")
    if not os.path.exists(vimrclocal):
        print("creating", vimrclocal)
        with open(vimrclocal, "w") as fp:
            fp.write('" Put your local settings here\n"')

    this_dir = os.path.dirname(__file__)
    vimrc_src = os.path.join(this_dir, "..", "vimrc")
    vimrc_src = os.path.abspath(vimrc_src)
    to_write = VIMRC_TEMPLATE.format(vimrc=vimrc_src,
                                     vimrclocal=vimrclocal)

    vimrc_dest = os.path.expanduser(INIT_VIM)
    if os.path.exists(vimrc_dest):
        print(INIT_VIM, "already exists, skpping")
        return
    with open(vimrc_dest, "w") as fp:
        print("creating", vimrclocal)
        fp.write(to_write)

def enable_vim():
    """ Enable vim usage, by creating symlinks to the neovim
    locations

    """
    vimrc = os.path.expanduser("~/.vimrc")
    if not os.path.exists(vimrc):
        print(vimrc, "->", INIT_VIM)
        os.symlink(INIT_VIM, vimrc)

    autoload = os.path.expanduser("~/.vim/autoload")
    mkdir_p(autoload)
    src = os.path.join(autoload, "plug.vim")
    dest = os.path.expanduser("~/.config/nvim/autoload/plug.vim")
    if not os.path.exists(src):
        print(src, "->", dest)
        os.symlink(dest, src)
    return "vim"

def enable_nvim():
    """ Enable nvim usage, by creating a wrapper
    in ~/.local/bin

    """
    this_dir = os.path.dirname(__file__)
    top_dir = os.path.join(this_dir, "..")
    top_dir = os.path.abspath(top_dir)
    src = os.path.expanduser("~/.local/bin/vim")
    dest = os.path.join(top_dir, "vim")
    if not os.path.exists(src):
        print(src, "->", dest)
        os.symlink(dest, src)
    return "nvim"

def main():
    """ Main entry point

    """
    parser = argparse.ArgumentParser()
    parser.add_argument("--enable-vim", action="store_true",
                        help="Enable vim usage. Useful when neovim is not "
                             "available")
    args = parser.parse_args()
    nvim_conf_dir = os.path.dirname(INIT_VIM)
    autoload = os.path.join(nvim_conf_dir, "autoload")
    mkdir_p(autoload)
    vim_plug = os.path.join(autoload, "plug.vim")
    fetch_plug_vim(vim_plug)
    create_vimrc_files()
    if args.enable_vim:
        vim_exec = enable_vim()
    else:
        vim_exec = enable_nvim()
    # Run vim once with :PlugUpdate to perform frist installation
    cmd = [vim_exec, "-c", ":PlugUpdate"]
    subprocess.call(cmd)

if __name__ == "__main__":
    main()
