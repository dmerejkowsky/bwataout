""" Install the dot files """

from __future__ import print_function

import os

THIS_DIR = os.path.abspath(os.path.dirname(__file__))

def mkdir_p(path):
    if not os.path.exists(path):
        os.makedirs(path)

def create_symlink_if_missing(src, dest):
    if os.path.exists(dest):
        print("Skipping", dest)
        return
    print(src, "->", dest)
    os.symlink(dest, src)

def write_file_if_missing(path, contents):
    if os.path.exists(path):
        print("Skipping", path)
        return
    print("Creating", path)
    dirname = os.path.dirname(path)
    mkdir_p(dirname)
    with open(path, "w") as fp:
        fp.write(contents)

def main():
    mkdir_p(os.path.expanduser("~/.local/bin"))
    mkdir_p(os.path.expanduser("~/.config"))

    # .gitconfig
    gitconfig = os.path.expanduser("~/.config/git/config")
    write_file_if_missing(gitconfig, """\
# Autogenerated. Do not edit
[core]
excludesfile = {0}/gitexcludes

[include]
path = {0}/gitconfig
path = {0}/gitconfig.local


""" .format(THIS_DIR))

    # .zsh
    zshrc = os.path.expanduser("~/.zshrc")
    write_file_if_missing(zshrc, """\
# Auto-generated. Do not edit
# Customization go into {0}/zshrc.local
source {0}/zshrc
[[ -f {0}/zshrc.local ]] && source {0}/zshrc.local
""".format(THIS_DIR))


if __name__ == "__main__":
    main()
