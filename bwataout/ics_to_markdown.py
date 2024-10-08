"""
For each event in the input .ics file, generate a markdown file
with the correct title in the given output directory.

I use this so that it's easy to generate notes for each session
"""

import arrow
from ics import Calendar
import sys
import argparse
from pathlib import Path


def parse_ics(contents):
    calendar = Calendar(contents)
    events = sorted(calendar.events)
    now = arrow.now()
    # No need for events in the past
    return [event for event in events if event.begin >= now]


def write_events(markdown_directory, events):
    for event in events:
        date = event.begin
        name = event.name
        suffix = "AM" if date.datetime.hour < 12 else "PM"
        title = f"{date:YYYY-MM-DD ddd} {suffix} - {name}"
        file_path = markdown_directory / (title + ".md")
        file_path.write_text(f"# {title}\n")

    print(f"Generated {len(events)} events")

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-i", "--input", required=True, type=Path)
    parser.add_argument("-o", "--output", required=True, type=Path)
    args = parser.parse_args()

    ics_path = args.input
    markdown_directory = args.output

    if not markdown_directory.is_dir():
        sys.exit("Output must be a directory")

    events = parse_ics(ics_path.read_text())
    write_events(markdown_directory, events)

if __name__ == '__main__':
    main()  
