""" Install the dot files """

from __future__ import print_function

import argparse
import os
import subprocess
import sys


if sys.version_info.major == 2:
    from urllib import urlretrieve
else:
    from urllib.request import urlretrieve


THIS_DIR = os.path.abspath(os.path.dirname(__file__))

def mkdir_p(path):
    if not os.path.exists(path):
        os.makedirs(path)


def create_symlink_if_missing(src, dest):
    if os.path.exists(src):
        print("Skipping", src)
        return
    dirname = os.path.dirname(src)
    mkdir_p(dirname)
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


def vim_install(enable_vim=False):
    nvim_conf_dir = os.path.expanduser("~/.config/nvim")
    autoload = os.path.join(nvim_conf_dir, "autoload")
    mkdir_p(autoload)

    # Fetch vim plug
    vim_plug = os.path.join(autoload, "plug.vim")
    print("Retrieving", vim_plug)
    urlretrieve("https://raw.githubusercontent.com/junegunn/vim-plug/0.7.2/plug.vim",
                vim_plug)

    # Create vimrc files
    vimrc_local = os.path.join(nvim_conf_dir, "vimrc.local")
    write_file_if_missing(vimrc_local, '" Put your local settings here\n')

    vimrc_src = os.path.join(THIS_DIR, "vim", "vimrc")
    vimrc_src = os.path.abspath(vimrc_src)
    vimrc_template = """\
" Auto-generated code. Do not edit\n"
source {vimrc}\n"
source {vimrc_local}\n"
"""
    to_write = vimrc_template.format(vimrc=vimrc_src,
                                     vimrc_local=vimrc_local)
    init_vim = os.path.join(nvim_conf_dir, "init.vim")
    write_file_if_missing(init_vim, to_write)

    # Create symlinks for vim usage
    if enable_vim:

        # ~/.vimrc -> ~/.config/nvim/init.vim
        vimrc = os.path.expanduser("~/.vimrc")
        create_symlink_if_missing(vimrc, init_vim)

        # ~/.vim/autoload/plug.vim -> ~/.config/nvim/autoload/plug.vim
        autoload = os.path.expanduser("~/.vim/autoload")
        src = os.path.join(autoload, "plug.vim")
        dest = vim_plug
        create_symlink_if_missing(src, dest)

    # Install the Python wrapper:
    src = os.path.expanduser("~/.local/bin/vim_wrapper.py")
    dest = os.path.join(THIS_DIR, "vim", "vim_wrapper.py")
    create_symlink_if_missing(src, dest)

    # Run vim once with :PlugInstall to perform first installation
    if enable_vim:
        vim_exec = "vim"
    else:
        vim_exec = "nvim"
    cmd = [vim_exec, "-c", ":PlugInstall"]
    subprocess.call(cmd)


def zsh_install_pure_prompt():
    """ Install zsh pure prompt from my own fork """
    # All we have to do is copy to files from the repo
    # to ~/.local/share/zsh/pure-prompt/ with the correct name
    zsh_prompt_dir = os.path.expanduser("~/.local/share/zsh/pure-prompt/")
    mkdir_p(zsh_prompt_dir)
    base_url = "https://raw.githubusercontent.com/dmerejkowsky/zsh-pure-prompt"
    base_url += "/master"
    todo = [("pure.zsh", "prompt_pure_setup"),
            ("async.zsh", "async")]
    for (src, dest) in todo:
        url = os.path.join(base_url, src)
        full_dest = os.path.join(zsh_prompt_dir, dest)
        print("Retrieving", full_dest)
        urlretrieve(url, full_dest)


def zsh_install_z():
    dest = os.path.expanduser("~/.local/share/zsh/z")
    if not os.path.exists(dest):
        cmd = ["git", "clone", "--quiet",
               "--branch", "v1.9",
               "--depth", "1",
               "https://github.com/rupa/z",
               dest]
        # Sadly, need to suppress the stderr to hide the
        # 'detached HEAD' warning
        # We could use git-config tu suppress the advice, but
        # it's convenient to have it to copy/paste the SHA1 ...
        subprocess.check_call(cmd, stderr=subprocess.PIPE)


def install_fzf():
    dest = os.path.expanduser("~/.fzf")
    if os.path.exists(dest):
        print("Skipping", dest)
        return
    print("Cloning fzf")
    cmd = ["git", "clone", "--quiet",
           "git@github.com/dmerejkowsky:fzf.git",
           dest]
    subprocess.check_call(cmd)
    install_path = os.path.join(dest, "install")
    cmd = ["bash", install_path, "--bin"]
    subprocess.check_call(cmd)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--enable-vim", action="store_true",
                        help="Enable vim usage. Useful when neovim is not "
                             "available")
    parser.set_defaults(enable_vim=False)
    args = parser.parse_args()

    mkdir_p(os.path.expanduser("~/.local/bin"))
    mkdir_p(os.path.expanduser("~/.config"))

    # conky
    src = os.path.expanduser("~/.config/conky/conky.conf")
    dest = os.path.join(THIS_DIR, "conky.conf")
    if sys.platform.startswith("linux"):
        create_symlink_if_missing(src, dest)

    # git
    gitconfig_dir = os.path.expanduser("~/.config/git")
    gitconfig = os.path.join(gitconfig_dir, "config")
    write_file_if_missing(gitconfig, """\
# Autogenerated. Do not edit
[core]
excludesfile = {0}/gitexcludes

[include]
path = {0}/gitconfig
path = {1}/config.local
""" .format(THIS_DIR, gitconfig_dir))
    gitconfig_local = gitconfig + ".local"
    write_file_if_missing(gitconfig_local, """\
# Configure your user here
[user]
name =
email =


# vim: set filetype=gitconfig:
""")

    # i3
    src = os.path.expanduser("~/.config/i3/config")
    dest = os.path.join(THIS_DIR, "i3")
    create_symlink_if_missing(src, dest)

    # fzf
    install_fzf()

    # mpv
    src = os.path.expanduser("~/.config/mpv/input.conf")
    dest = os.path.join(THIS_DIR, "mpv/input.conf")
    create_symlink_if_missing(src, dest)

    # allow to use `npm install -g` _without_ being root
    src = os.path.expanduser("~/.npmrc")
    write_file_if_missing(src, "prefix=%s\n" % os.path.expanduser("~/.local"))

    # openbox
    src = os.path.expanduser("~/.config/openbox")
    dest = os.path.join(THIS_DIR, "openbox")
    create_symlink_if_missing(src, dest)

    # screen
    src = os.path.expanduser("~/.screenrc")
    dest = os.path.join(THIS_DIR, "screenrc")
    create_symlink_if_missing(src, dest)

    # sway
    src = os.path.expanduser("~/.config/sway/config")
    dest = os.path.join(THIS_DIR, "sway")
    create_symlink_if_missing(src, dest)

    # tint2
    src = os.path.expanduser("~/.config/tint2/tint2rc")
    dest = os.path.join(THIS_DIR, "tint2rc")
    create_symlink_if_missing(src, dest)

    # vim / neovim
    vim_install(enable_vim=args.enable_vim)

    # xinitrc
    src = os.path.expanduser("~/.xinitrc")
    dest = os.path.join(THIS_DIR, "xinitrc")
    create_symlink_if_missing(src, dest)
    xinitrc_local = src + ".local"
    write_file_if_missing(xinitrc_local, """\
# Run your window manager from here
""")

    # zsh
    zshrc = os.path.expanduser("~/.zshrc")
    zshrc_local = zshrc + ".local"
    write_file_if_missing(zshrc, """\
# Auto-generated. Do not edit
source {0}/zshrc
if [ -f "${{HOME}}/.zshrc.local" ] ; then
  source "${{HOME}}/.zshrc.local"
fi
""".format(THIS_DIR))
    zsh_install_pure_prompt()
    zsh_install_z()


if __name__ == "__main__":
    main()
