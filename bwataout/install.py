import argparse
from io import BytesIO
from pathlib import Path
import subprocess
import shutil
import tarfile
from typing import IO, List, Optional
import sys

import httpx
from ruamel.yaml import YAML  # type: ignore

import cli_ui as ui
import gzip


TOP_DIR = Path(__file__).parent.parent.resolve()


class Installer:
    def __init__(self, force: bool = False):
        yaml = YAML(typ="safe")
        conf_path = TOP_DIR / "programs.yml"
        self.conf = yaml.load(Path(conf_path).read_text())
        self.home = Path("~").expanduser()
        self.force = force

    def pretty_path(self, p: Path) -> str:
        return f"~/{p.relative_to(self.home)}"

    def do_clone(
        self,
        url: str,
        dest: str,
        branch: str = "main",
        submodules: bool = False,
    ) -> None:
        dest_path = Path(dest).expanduser()
        pretty_dest = self.pretty_path(dest_path)
        dest_path.parent.mkdir(parents=True, exist_ok=True)
        if dest_path.exists():
            if self.force:
                shutil.rmtree(dest_path)
            else:
                ui.info_2("Skipping", pretty_dest)
                return
        ui.info_2("Cloning", url, "->", pretty_dest)
        subprocess.check_call(["git", "clone", url, str(dest_path), "--branch", branch])

    def do_copy(self, src: str, dest: str) -> None:
        dest_path = Path(dest).expanduser()
        pretty_dest = self.pretty_path(dest_path)
        dest_path.parent.mkdir(parents=True, exist_ok=True)
        if dest_path.exists() and not self.force:
            ui.info_2("Skipping", pretty_dest)
            return
        src_path = TOP_DIR / "configs/" / src
        ui.info_2("Copy", src, "->", self.pretty_path(src_path))
        shutil.copy(src_path, dest_path)

    def do_download(
        self,
        *,
        url: str,
        dest: str,
        executable: bool = False,
        extract_member: Optional[str] = None,
        unzip: bool = False,
    ) -> None:
        dest_path = Path(dest).expanduser()
        dest_path.parent.mkdir(parents=True, exist_ok=True)
        if extract_member:
            self._download_and_extract_member(url, dest_path, member=extract_member)
        else:
            self._download(url, dest_path, unzip=unzip)
        if executable:
            dest_path.chmod(0o755)

    def _download(self, url: str, dest_path: Path, unzip: bool = False) -> None:
        if unzip:
            self._download_and_unzip(url, dest_path)
        else:
            self._simple_download(url, dest_path)

    def _download_and_unzip(self, url: str, dest_path: Path) -> None:
        pretty_dest = self.pretty_path(dest_path)
        if dest_path.exists() and not self.force:
            ui.info_2("Skipping", pretty_dest)
        else:
            ui.info_2("Fetching", url, "->", pretty_dest)
            r = httpx.get(url)
            if r.is_error:
                ui.fatal(f"Got status {r.status_code} when fetching {url}")
            r = httpx.get(url)
            if r.is_error:
                ui.fatal(f"Got status {r.status_code} when fetching {url}")
            buffer = gzip.GzipFile(fileobj=BytesIO(r.content))
            with open(dest_path, "wb") as output_file:
                shutil.copyfileobj(buffer, output_file)

    def _simple_download(self, url: str, dest_path: Path) -> None:
        pretty_dest = self.pretty_path(dest_path)
        if dest_path.exists() and not self.force:
            ui.info_2("Skipping", pretty_dest)
        else:
            ui.info_2("Fetching", url, "->", pretty_dest)
            with open(dest_path, "wb") as o:
                with httpx.stream("GET", url) as r:
                    if r.is_error:
                        ui.fatal(f"Got status {r.status_code} when fetching {url}")
                    for buffer in r.iter_bytes():
                        o.write(buffer)

    def _download_and_extract_member(
        self, url: str, dest_path: Path, *, member: str
    ) -> None:
        pretty_dest = self.pretty_path(dest_path)
        if dest_path.exists() and not self.force:
            ui.info_2("Skipping", pretty_dest)
        else:
            ui.info_2("Fetching", url, "->", pretty_dest)
            r = httpx.get(url)
            if r.is_error:
                ui.fatal(f"Got status {r.status_code} when fetching {url}")
            archive = tarfile.open(fileobj=BytesIO(r.content), mode="r:gz")
            if not archive.getmember(member):
                sys.exit(f"No member named '{member}' found in the archive!")
            decompressed_member: IO[bytes] = archive.extractfile(member)  # type: ignore
            with decompressed_member:
                with open(dest_path, "wb") as output_file:
                    shutil.copyfileobj(decompressed_member, output_file)

    def do_write(self, src_str: str, contents: str) -> None:
        src = Path(src_str).expanduser()
        pretty_src = self.pretty_path(src)
        if src.exists() and not self.force:
            ui.info_2("Skipping", pretty_src)
            return
        ui.info_2("Creating", pretty_src)
        src.parent.mkdir(parents=True, exist_ok=True)
        contents = contents.format(top_dir=TOP_DIR, home=self.home)
        if not contents.endswith("\n"):
            contents += "\n"
        src.write_text(contents)

    def do_symlink(self, src: str, dest: str) -> None:
        self._do_simlink(src, dest, is_dir=False)

    def do_symlink_dir(self, src: str, dest: str) -> None:
        self._do_simlink(src, dest, is_dir=True)

    def _do_simlink(self, src: str, dest: str, *, is_dir: bool) -> None:
        dest_path: Path = Path(dest).expanduser()
        pretty_dest = self.pretty_path(dest_path)
        if is_dir:
            dest_path.parent.parent.mkdir(parents=True, exist_ok=True)
        else:
            dest_path.parent.mkdir(parents=True, exist_ok=True)

        if dest_path.exists() and not self.force:
            ui.info_2("Skipping", pretty_dest)
            return
        if dest_path.is_symlink():
            # if self.force is True, we want to re-create the symlink
            # else, we know it's a broken symlink
            # in any case: remove it
            dest_path.unlink()
        ui.info_2("Symlink", pretty_dest, "->", src)
        src_path = TOP_DIR / "configs" / src
        dest_path.symlink_to(src_path)

    def do_run(self, *args: str) -> None:
        ui.info_2("Running", "`%s`" % " ".join(args))
        fixed_args = [x.format(home=self.home) for x in args]
        subprocess.check_call(fixed_args)

    def install_program(self, program: str) -> None:
        ui.info(ui.green, program)
        ui.info("-" * len(program))
        todo = self.conf[program]
        for action in todo:
            name = list(action.keys())[0]
            params = action[name]
            func = getattr(self, "do_%s" % name)
            if isinstance(params, dict):
                func(**params)
            else:
                func(*params)
        ui.info()

    def install(self, programs: Optional[List[str]] = None) -> None:
        if not programs:
            programs = sorted(self.conf.keys())
        for program in programs:
            self.install_program(program)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("programs", nargs="*")
    parser.add_argument("--force", action="store_true", help="Overwite existing files")
    parser.add_argument("--list", action="store_true", help="List available programs")
    args = parser.parse_args()

    force = args.force
    programs = args.programs
    installer = Installer(force=force)
    if args.list:
        for key in installer.conf.keys():
            print(key)
        return
    installer.install(programs=programs)


if __name__ == "__main__":
    main()
