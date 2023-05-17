"""
the main module for the cportage program
"""

## global imports
import curses

## local imports
from gui.display import Display
from core.data import junk_data

def main(stdscr):
    
    view = Display(stdscr, "Title", junk_data(150, 30))
    stdscr.refresh()

    while True:
        view.refresh()

        # grabbing input from the user
        key = stdscr.getkey()
        if key == "q":
            break

        # letting the display handle the input
        view.handle_input(key)

    return 0

# default start
if __name__ == "__main__":
    curses.wrapper(main)

