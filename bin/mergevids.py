import sys
import subprocess
from pathlib import Path


def main():
    # Prerequesite : one mp4 per segment
    # Do it with `ffmpeg -i <>.wmv <>.mp4` for instance
    files = [Path(p) for p in sys.argv[1:]]
    output_name = files[0].stem[:-1]  # remove number at the end
    with open("list.txt", "w") as f:
        for file in files:
            f.write(f"file '{file}'\n")
    cmd = f"ffmpeg -f concat -i list.txt {output_name}.mp4"
    print(cmd)
    subprocess.run(cmd, shell=True)


if __name__ == "__main__":
    main()
