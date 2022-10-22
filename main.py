#!/usr/bin/python

"""
this is the main script for cportage
"""

## module imports

import curses
import random
import string

## local imports

from atoms import display
from atoms import keys

#############
# file body #
#############

limit = 128
data = ["".join([string.ascii_lowercase[i % len(string.ascii_lowercase)] * random.randint(1,16)]) for i in range(limit)]

def main(stdscr):
    # declaring limit
    scrolloffset = 0
    (maxheight, maxwidth) = stdscr.getmaxyx()

    # main loop
    while True:
        # cleaning up the screeen
        stdscr.clear()
        stdscr.border(0)

        # calculating the number of elements to display
        limit = maxheight + scrolloffset - 2 # 2 for border
        if limit >= len(data): limit = len(data)
        subset = data[scrolloffset:limit]

        # filling the screen
        for index, string in enumerate(subset):
            stdscr.addstr(index+1, 0+1, "%2d: %s" % (index+scrolloffset, string))

        # key switch
        key = stdscr.getkey()
        shift = 0
        if key == "KEY_DOWN":
            shift = 1
        if key == "KEY_UP":
            shift = -1
        if key == keys.PAGEDOWN:
            shift = maxheight
        if key == keys.PAGEUP:
            shift = -maxheight
        if key == "q":
            break

        scrolloffset += shift
        # scrolloffset sanity check
        if scrolloffset > (len(data) - maxheight + 2):
            scrolloffset = len(data) - maxheight + 2
        if scrolloffset < 0:
            scrolloffset = 0

        # refreshing the screen
        stdscr.refresh()

    return 0

if __name__ == "__main__":
    curses.wrapper(main)

