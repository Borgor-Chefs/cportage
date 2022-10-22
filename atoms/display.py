"""
this file details the windows that the program will make
e.g., this is a wrapper around curses.window
"""

def Display():
    def __init__(self, screen, data):
        self.screen = screen
        self.data = data

    ###########
    # methods #
    ###########

    def refresh():
        if self.border:
            stdscr.border()

    ##############
    # properties #
    ##############

    @property
    def border(self) -> bool:
        return self.border

    @property
    def border(self, value: bool):
        self.border = value

