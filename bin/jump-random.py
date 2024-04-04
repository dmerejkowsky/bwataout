"""
Jump to a random line in a random file

Useful for *synchronous* code reviews
"""

import random
import subprocess
from argparse import ArgumentParser
from pathlib import Path


def main():
    parser = ArgumentParser(prog="jump-random", description=__doc__)
    parser.add_argument(
        "--open-with",
        choices=["code", "kakoune", "vim"],
        default=None,
        help="Open with the given editor",
    )
    args = parser.parse_args()

    open_with = args.open_with

    source_file = get_random_source_file()
    line_number = get_random_line_number(source_file)

    print(f"{source_file}:{line_number}")

    if open_with == "code":
        subprocess.run(["code", "--goto", f"{source_file}:{line_number}"])
    elif open_with == "vim":
        subprocess.run(["vim", f"+{line_number}", source_file])
    elif open_with == "kakoune":
        subprocess.run(["kak", "-e", f"edit -existing %[{source_file}]  {line_number}"])


def get_random_source_file():
    git_ls = subprocess.run(
        ["git", "ls-files"],
        capture_output=True,
        text=True,
        check=True,
    )
    sources = git_ls.stdout.splitlines(keepends=False)
    random_source = random.choice(sources)
    return Path(random_source)


def get_random_line_number(source_file: Path):
    lines = source_file.read_text().splitlines()
    return random.randint(1, len(lines) + 1)


if __name__ == "__main__":
    main()
