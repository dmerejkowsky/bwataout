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
    nvim = neovim.attach("socket", path=SOCKET_PATH)
    to_open = parse_filespecs_for_remote(filespecs)
    nvim.command(":tabnew")
    for fullpath, line, column in to_open:
        nvim.command(":e %s" % fullpath)
        nvim.feedkeys("%iG" % line)
        nvim.feedkeys("%i|" % column)

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
    parsed = parse_filespecs_for_cmdline(filespecs)
    cmd = [nvim_path]
    if diff:
        cmd.append("-d")
    cmd.extend(parsed)
    rc = subprocess.call(cmd, env=env)
    sys.exit(rc)

def parse_filespecs_for_cmdline(filespecs):
    """ Return a list of command line arguments
    suitable to open the file at the correct line
    number.

    Due to vim command line API limitations, this
    only works for lists of one element...

    >>> parse_filespecs_for_cmdline(["foo.cpp:42"])
    ["foo.cpp", "+42"]

    """
    if len(filespecs) == 1:
        filespec = filespecs[0]
        if ":" in filespec:
            parts = filespec.split(":")
            line = parts[1]
            filename = parts[0]
            return ["+%s" % line, filename]
        else:
            return [filespec]
    else:
        return filespecs

def parse_filespecs_for_remote(filespecs):
    """ Return a list of tuples
    (full_path, line_number, column_number)

    >>> parse_filespecs_for_remote(["foo.cpp:42:3"])
    [("/path/to/foo.cpp", 42, 3)]
    >>> parse_filespecs_for_remote(["foo.cpp:42"])
    [("/path/to/foo.cpp", 42, 1)]
    >>> parse_filespecs_for_remote(["foo.cpp"])
    [("/path/to/foo.cpp", 1)]

    """
    res = list()
    for filespec in filespecs:
        parts = filespec.split(":")
        parts += ["1"] * (3 - len(parts))
        parts[0] = os.path.abspath(parts[0])
        for i in (1, 2):
            try:
                parts[i] = int(parts[i])
            except ValueError:
                sys.exit("Failed to parse %s" % filespec)
        res.append(parts)
    return res

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
            sys.exit("Please install neovim Python package before using --remote")
        remote_nvim(filespecs)
    else:
        main_nvim(nvim_path, filespecs, diff=args.diff)

if __name__ == "__main__":
    main()
