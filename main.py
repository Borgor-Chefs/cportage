#!/usr/bin/python

"""
this is the main script for cportage
"""

## module imports

import curses
import random
import string

## local imports

from atoms.display import Display
from atoms import keys

#############
# file body #
#############

limit = 128
data = ["%3d: %s" % (i, "".join([string.ascii_lowercase[i % len(string.ascii_lowercase)] * random.randint(1,16)])) for i in range(limit)]

def main(stdscr):
    # flushing the screen
    stdscr.refresh()

    # getting maximum display dimensions
    (maxheight, maxwidth) = stdscr.getmaxyx()

    # this view will display packages
    view = Display(data, stdscr, title="Packages")
    view.debug()

    # main loop
    while True:
        view.refresh()
        
        # key switch
        key = stdscr.getkey()
        if key == "q":
            break
        view.handle_input(key)

    return 0

if __name__ == "__main__":
    curses.wrapper(main)

