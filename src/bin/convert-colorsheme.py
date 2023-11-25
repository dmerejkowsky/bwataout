from pathlib import Path
from configparser import ConfigParser


def rgb_to_hex(rgb):
    return '%02x%02x%02x' % rgb

def parse_konsole_theme(path):
    colors = {}
    parser = ConfigParser()
    parser.read([path])
    for section_name in parser.sections():
        if not section_name.startswith("Color") and not section_name in ["Background", "Foreground"]:
            continue
        color_name = section_name.lower()
        for (key, value) in parser[section_name].items():
            if "intense" in color_name or "faint" in color_name:
                continue
            r,g,b = [int(x) for x in value.split(",")]
            colors[color_name] = rgb_to_hex((r,g,b))
    return colors


def main():
    konsole_theme_path = Path("/usr/share/konsole/BlackOnWhite.colorscheme")
    colors = parse_konsole_theme(konsole_theme_path)
    with open("theme.conf", "w") as f:
        for (name, value) in colors.items():
            f.write(f"{name} #{value}\n")


main()
