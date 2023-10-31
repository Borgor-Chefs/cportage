"""
the main module for the cportage program
"""

# global imports
import curses

# local imports
from gui.display import Display
from gui import keys
from core.data import junk_data


def main(stdscr):

    root = Display(stdscr, "Title", junk_data(150, 30))
    stdscr.refresh()

    # pointer to the current context (window)
    view = root

    while True:
        view.refresh()

        # grabbing input from the user
        key = stdscr.getkey()

        # handling the input
        match key:
            # special cases
            case "q":
                break
            case keys.ARROWRIGHT:
                view = view.spawn_child()
            case keys.ARROWLEFT:
                if view.parent:
                    tmp = view
                    view = view.parent
                    tmp.destroy()
                    del tmp
            case _:
                # letting the display handle the input
                view.handle_input(key)

    return 0


# default start
if __name__ == "__main__":
    curses.wrapper(main)

