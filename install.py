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

    def do_clone(self, url, dest, branch="master"):
        dest = path.Path(dest).expanduser()
        dest.parent.makedirs_p()
        if dest.exists():
            ui.info_2("Skipping", dest.relpath(self.home))
            return
        ui.info_2("Cloning", url, "->", dest.relpath(self.home))
        subprocess.check_call(["git", "clone", url, dest, "--branch", branch])

    def do_fetch(self, url, dest):
        dest = path.Path(dest).expanduser()
        dest.parent.makedirs_p()
        if dest.exists():
            ui.info_2("Skipping", dest.relpath(self.home))
            return
        ui.info_2("Fetching", url, "->", dest.relpath(self.home))
        urlretrieve(url, dest)

    def do_write(self, src, contents):
        src = path.Path(src).expanduser()
        if src.exists():
            ui.info_2("Skipping", src.relpath(self.home))
            return
        ui.info_2("Creating", src.relpath(self.home))
        src.parent.makedirs_p()
        contents = contents.format(this_dir=self.this_dir, home=self.home)
        src.write_text(contents)

    def do_symlink(self, src, dest):
        src = self.this_dir.joinpath(src)
        dest = path.Path(dest).expanduser()
        dest.parent.makedirs_p()
        if dest.exists():
            ui.info_2("Skipping", dest.relpath(self.home))
            return
        ui.info_2("Symlink", dest.relpath(self.home))
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
