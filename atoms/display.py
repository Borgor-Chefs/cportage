"""
this file details the windows that the program will make
e.g., this is a wrapper around curses.window
"""

## module imports

import curses
from typing import List

## local imports

from . import keys

#############
# file body #
#############

class Display:
    """
    this is a window with a border and a pad, where the data put into
    the object is put into its pad and the pad is then rendered
    within the window object which has a border.

    this handles user input, window scroll math, focus, etc.
    """
    def __init__(self, data: List[str], parent,
            yoffset=0, xoffset=0, title=""):
        ## saving the args
        self.data = data
        self.parent = parent
        self.title = title

        # getting the max dimensions of the screen
        (maxheight, _) = parent.getmaxyx()

        ### forming the geometry of the child windows
        self.borderlen = 2 # because borders
        self.yoffset = yoffset
        self.xoffset = xoffset

        ## the pad is where the text will be stored
        self.pad_height = len(data)
        self.pad_width = max(map(len, data))

        ## the window is where the text will be displayed
        self.win_height = maxheight
        self.win_width = self.pad_width + self.borderlen

        ## creating the windows
        self.window = parent.subwin(self.win_height,
                self.win_width, self.yoffset, self.xoffset)
        self.pad = curses.newpad(self.pad_height, self.pad_width)

        ### setting limits for scrolling and where the focus is
        ## focus
        self.focus = 0
        self.prev_focus = 0
        self.focus_min = 0
        self.focus_max = self.win_height - self.borderlen - 1

        ## scrolling
        self.scroll = 0
        self.scroll_min = 0
        self.scroll_max = len(data) - self.win_height + self.borderlen

        ## setting up the windows
        self.__setup__()

        ## this is set so that we can do if-statements
        self.debug_win = None                    # a debug window

    def __setup__(self):
        """
        this function instantiates both the window & pad with a border
        and text respectively
        """
        ### setting up the windows
        
        ## filling the pad with data
        for index, string in enumerate(self.data):
            self.pad.addstr(index, 0, string)
        
        # highlighting text after filling the pad
        self.pad.chgat(0, 0, curses.A_REVERSE)  # highlighting

        # drawing a border before a title
        self.window.border(0)

        ## setting a title if one was passed in
        if self.title:
            self.window.addstr(0, 0+3, self.title)

    ###########
    # methods #
    ###########

    def refresh(self):
        """
        this function refreshes the window (border), the pad
        (the text) and the object's debug window (if enabled)
        """
        self.window.refresh()
        self.pad.refresh(0+self.scroll, 0, 0+1, 0+1,
                self.win_height - 2, self.win_width - 2)
        if self.debug_win:
            self.debug_write()
            self.debug_win.refresh()

    def handle_input(self, keystroke):
        """
        this function handles window scrolling math and keystrokes
        """
        shift = 0
        if keystroke == "KEY_DOWN":
            shift = 1
        if keystroke == "KEY_UP":
            shift = -1
        if keystroke == keys.PAGEDOWN:
            shift = self.win_height
        if keystroke == keys.PAGEUP:
            shift = -self.win_height
        
        ## applying focus (highlight entry)
        self.prev_focus = self.focus
        self.focus += shift

        ## moving window
        if (self.focus >= self.focus_max and \
                self.scroll < self.scroll_max) or \
                (self.focus <= self.focus_min and \
                self.scroll > self.scroll_min):
            self.scroll += shift
            self.focus -= shift

        ## bounds check
        if self.focus < self.focus_min:
            self.focus = self.focus_min
        if self.focus > self.focus_max:
            self.focus = self.focus_max
        if self.scroll < self.scroll_min:
            self.scroll = self.scroll_min
        if self.scroll > self.scroll_max:
            self.scroll = self.scroll_max

        ## asserting the distance between self.focus & prev gte 1
        # this is to fix unhighlighting issues
        if abs(self.focus - self.prev_focus) < 1:
            self.prev_focus = self.focus + (1,-1)[shift > 0]

        ## highlighting
        self.pad.chgat(self.scroll+self.prev_focus, 0, curses.A_NORMAL)
        self.pad.chgat(self.scroll+self.focus, 0, curses.A_REVERSE)
    
    def debug(self):
        """
        this function creates a new window object to put info in
        """
        ## creating the window
        self.debug_win = self.parent.subwin(self.win_height, 60, 0, self.win_width + 2)
        self.debug_win.border(0) # adding a border

        # adding a title
        title = "DEBUG"
        if self.title:
            title += " (%s)" % self.title
        self.debug_win.addstr(0, 20, title)

    def debug_write(self):
        """
        this function writes info into the debug window,
        this has to be called with every refresh so that it
        updates text stored within the window
        """

        ### creating strings (INSERT DATA POINTS BELOW) ###
        # VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV #

        ## single vars you want to see
        single_vars = [
                ("scroll", self.scroll),
                ("focus", self.focus),
                ("prev_focus", self.prev_focus),
                ("focus'd index", self.focus+self.scroll),
                ("focus'd element", self.data[self.focus+self.scroll])
        ]

        ## vars that require two operands and sum together
        compound_vars = [
                ["un-highlight", self.scroll, self.prev_focus],
                ["do highlight", self.scroll, self.focus],
        ]

        ## manipulating compound_vars
        for collection in compound_vars:
            # doing this because manually summing both vars in the list's declaration
            #   aids, doing it here is nicer
            collection.append(collection[1] + collection[2])

        ## vars that have a min/max attached to them
        bounds = [
                ("focus", self.focus, self.focus_max, self.focus_min),
                ("scroll", self.scroll, self.scroll_max, self.scroll_min)
        ]

        ## conditions that need to be observed
        bools = [
                ["focus == focus_min", self.focus == self.focus_min],
                ["focus == focus_max", self.focus == self.focus_max],
                ["scroll == scroll_min", self.scroll == self.scroll_min],
                ["scroll == scroll_max", self.scroll == self.scroll_max],
                ["abs(focus - prev_focus) >= 1", abs(self.focus - self.prev_focus) >= 1]
        ]
        ## template strings render true/false as 1/0, changing it to be string-like
        for collection in bools:
            collection[1] = "true" if collection[1] else "false"

        ### creating a functional array

        templates = (
                "{0:<18s}: {1:>25}",
                "{0:<18s}: {1:<4d} + {2:<4d} = {3:4d}",
                "{0:<18s}: {3:<4d} < {1:<4d} < {2:4d}",
                "{0:<25s} is {1:<5s}"
        )
        labels = ("single vars", "compound vars", "bounds", "booleans")
        writeables = [single_vars, compound_vars, bounds, bools]

        ### writing to the window

        for index, template in enumerate(templates):
            ## gathering labels and calculating offsets for printing
            label = labels[index]
            collection = writeables[index]
            offset = sum(map(len, writeables[0:index]))
            self.debug_win.addstr(index+offset+1, 0+2,
                    '{:-^50}'.format(label))
            
            ## printing datapoints
            for delta, data in enumerate(collection, start=1):
                self.debug_win.addstr(index+offset+delta+1, 0+2, template.format(*data))

