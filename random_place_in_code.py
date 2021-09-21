#!/usr/bin/env python

import sys
from bisect import bisect_right
from random import randint
from subprocess import check_output, call

if len(sys.argv) > 1:
    command = sys.argv[1]
else:
    command = 'vi'

edit_commands = {
    'vi': 'vi %s +%s',
    'mate': 'mate %s -l %s',
}

if command not in edit_commands:
    print '%s is not a valid command for opening files, valid commands are: %s' % (command, edit_commands.keys())
    exit(1)

if sys.platform == 'darwin':
    find = 'find -E'
else:
    find = 'find -regextype posix-extended'

line_counts = check_output(find + " . -iregex '.*\.(py|php|c|cpp|h|hpp|java|js|css|html|htm|pl|rb|cs|m|r|go|swift|clj|cljs|hs)' | xargs wc -l", shell=True)

result = []
cumulative_count = 0
for x in line_counts.split('\n'):
    foo = x.strip().split(' ')
    if len(foo) == 2 and foo[1] != 'total':
        count, filename = foo
        count = int(count)
        cumulative_count += count
        result.append((count, cumulative_count, filename))

total_count = sum(x[0] for x in result)

random_place = randint(0, total_count)
print 'Picked random place %s in %s lines of code' % (random_place, total_count)
print '---'

index = bisect_right([x[1] for x in result], random_place)
filename = result[index][-1]
line_number = result[index][1] - random_place

print '%s:%s' % (filename,  line_number)

call(edit_commands[command] % (filename, line_number), shell=True)
