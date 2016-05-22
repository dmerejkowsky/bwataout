#!/usr/bin/env python

import argparse
import os
import sys
import subprocess

HAS_NEOVIM_PYTHON = True
try:
    import neovim
except ImportError:
    HAS_NEOVIM_PYTHON = False

SOCKET_PATH="/tmp/neovim"

def remote_nvim(filespecs):
    if not len(filespecs) == 1:
        sys.exit("Expecting exactly one argument")
    file = filespecs[0]
    file = os.path.abspath(file)
    nvim = neovim.attach("socket", path=SOCKET_PATH)
    nvim.command(":e %s" % file)

def find_nvim():
    """ Try to find nvim in $PATH.
    Return None if not found

    """
    candidates = os.environ["PATH"].split(os.pathsep)
    for candidate in candidates:
        full_path = os.path.join(candidate, "nvim")
        if os.path.exists(full_path):
            return full_path

def main_nvim(nvim_path, filespecs, diff=False):
    env = os.environ.copy()
    env["NVIM_LISTEN_ADDRESS"] = SOCKET_PATH
    parsed = parse_filespecs(filespecs)
    cmd = [nvim_path]
    if diff:
        cmd.append("-d")
    cmd.extend(parsed)
    rc = subprocess.call(cmd, env=env)
    sys.exit(rc)

def parse_filespecs(filespecs):
    if len(filespecs) == 1:
        filespec = filespecs[0]
        if ":" in filespec:
            splitted = filespec.split(":")
            line = splitted[1]
            filename = splitted[0]
            return ["+%s" % line, filename]
        else:
            return [filespec]
    else:
        return filespecs

def main():
    # TODO: parse filespecs (foo.c:42) here too:
    nvim_path = find_nvim()
    if not nvim_path:
        rc = subprocess.call(["vim"] + sys.argv[1:])
        sys.exit(rc)
    parser = argparse.ArgumentParser()
    parser.add_argument("--remote", action="store_true")
    parser.add_argument("-d", "--diff", action="store_true")
    parser.add_argument("filespecs", nargs="*")
    args = parser.parse_args()
    if args.remote and args.diff:
        sys.exit("Can not use --remote with --diff")
    remote = args.remote
    filespecs = args.filespecs
    if not filespecs:
        filespecs = list()
    if args.remote:
        if not HAS_NEOVIM_PYTHON:
            sys.exit("Please insntall neovim Python package before using --remote")
        remote_nvim(filespecs)
    else:
        main_nvim(nvim_path, filespecs, diff=args.diff)

if __name__ == "__main__":
    main()
