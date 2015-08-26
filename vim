#!/usr/bin/env python2.7

import argparse
import os
import sys
import subprocess

import neovim

SOCKET_PATH="/tmp/neovim"

def remote_nvim(filespecs):
    if not len(filespecs) == 1:
        sys.exit("Expecting exactly one argument")
    file = filespecs[0]
    file = os.path.abspath(file)
    nvim = neovim.attach("socket", path=SOCKET_PATH)
    nvim.command(":e %s" % file)


def main_nvim(filespecs):
    env = os.environ.copy()
    env["NVIM_LISTEN_ADDRESS"] = SOCKET_PATH
    parsed = parse_filespecs(filespecs)
    nvim_path = "/usr/local/bin/nvim"
    if not os.path.exists(nvim_path):
        nvim_path = "/usr/bin/nvim"
    cmd = [nvim_path]
    cmd.extend(parsed)
    rc = subprocess.call(cmd, env=env)
    sys.exit(rc)

def parse_filespecs(filespecs):
    if len(filespecs) == 1:
        filespec = filespecs[0]
        if ":" in filespec:
            splitted = filespec.split(":")
            line  = splitted[1]
            filename = splitted[0]
            return ["+%s" % line, filename]
        else:
            return [filespec]
    else:
        return filespecs

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--remote", action="store_true")
    parser.add_argument("filespecs", nargs="*")
    args = parser.parse_args()
    remote = args.remote
    filespecs = args.filespecs
    if not filespecs:
        filespecs = list()
    if args.remote:
        remote_nvim(filespecs)
    else:
        main_nvim(filespecs)

if __name__ == "__main__":
    main()
