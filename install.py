import argparse
import subprocess
from urllib.request import urlretrieve

import path
import ruamel.yaml

import ui


class Executor:
    def __init__(self):
        self.conf = ruamel.yaml.safe_load(path.Path("conf.yml").text())
        self.this_dir = path.Path(".").abspath()
        self.home = path.Path("~").expanduser()

    def pretty_path(self, p):
        relpath = p.relpath(self.home)
        return "~/" + relpath

    def do_clone(self, url, dest, branch="master"):
        dest = path.Path(dest).expanduser()
        pretty_dest = self.pretty_path(dest)
        dest.parent.makedirs_p()
        if dest.exists():
            ui.info_2("Skipping", pretty_dest)
            return
        ui.info_2("Cloning", url, "->", pretty_dest)
        subprocess.check_call(["git", "clone", url, dest, "--branch", branch])

    def do_fetch(self, url, dest):
        dest = path.Path(dest).expanduser()
        dest.parent.makedirs_p()
        pretty_dest = self.pretty_path(dest)
        if dest.exists():
            ui.info_2("Skipping", pretty_dest)
            return
        ui.info_2("Fetching", url, "->", pretty_dest)
        urlretrieve(url, dest)

    def do_write(self, src, contents):
        src = path.Path(src).expanduser()
        pretty_src = self.pretty_path(src)
        if src.exists():
            ui.info_2("Skipping", pretty_src)
            return
        ui.info_2("Creating", pretty_src)
        src.parent.makedirs_p()
        contents = contents.format(this_dir=self.this_dir, home=self.home)
        if not contents.endswith("\n"):
            contents += "\n"
        src.write_text(contents)

    def do_symlink(self, src, dest):
        src = self.this_dir.joinpath(src)
        dest = path.Path(dest).expanduser()
        pretty_dest = self.pretty_path(dest)
        dest.parent.makedirs_p()
        if dest.exists():
            ui.info_2("Skipping", pretty_dest)
            return
        if dest.islink():
            # we know it's a broken symlink
            dest.remove()
        ui.info_2("Symlink", pretty_dest)
        src.symlink(dest)

    def do_exec(self, args):
        ui.info_2("Running", "`%s`" % " ".join(args))
        fixed_args = [x.format(home=self.home) for x in args]
        subprocess.check_call(fixed_args)

    def install(self, program):
        ui.info(ui.green, program)
        ui.info("-" * len(program))
        todo = self.conf[program]
        for action in todo:
            name = list(action.keys())[0]
            args = action[name]
            func = getattr(self, "do_%s" % name)
            func(*args)
        ui.info()

    def execute(self, programs=None):
        if not programs:
            programs = sorted(self.conf.keys())
        for program in programs:
            self.install(program)


def install(programs=None):
    executor = Executor()
    executor.execute(programs=programs)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("programs", nargs="*")
    args = parser.parse_args()
    install(programs=args.programs)


if __name__ == "__main__":
    main()
