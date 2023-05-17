"""
the module for the Display object, which is a better wrapper around the ncurses pad object
"""

## global imports
import curses

## local imports
from . import keys

class Display:
	def __init__(self, window, title, data):
		"""
		creates a pad object based off of the window object
		"""

		## from arguments
		self.window = window
		self.title = title
		self.data = data

		# getting the dimensions of the parent window (usually from stdscr)
		(ymax, xmax) = window.getmaxyx()

		## internal vars
		# setting the offset
		self.ysize = ymax
		self.xsize = int(xmax / 3)
		self.yoffset = 0
		self.xoffset = 0

		# scrolling functionality
		self.index = 0

		# creating internal objects
		self.pad = curses.newpad(max(ymax, len(data)), self.xsize)

		# setting up the pad
		self.__setup__()

	def __setup__(self):
		"""
		this function writes the contents of `self.data` to the `self.pad` object
		"""

		# writing self.data to the pad
		for i, s in enumerate(self.data):
			self.pad.addnstr(i, 0, s, self.xsize)

		# highlighting the first line
		self.set_highlight(0)

	def set_highlight(self, line):
		return self.pad.chgat(line, 0, curses.A_REVERSE)

	def remove_highlight(self, line):
		return self.pad.chgat(line, 0, curses.A_NORMAL)

	def handle_input(self, char):
		"""
		this function makes an action based off of the provided input
		"""

		match char:

            ### scrolling keys

			case keys.ARROWDOWN:
				(ypad, _) = self.pad.getmaxyx()
				new = min(self.index + 1, ypad - 1)
				self.remove_highlight(self.index)
				self.set_highlight(new)
				self.index = new

			case keys.ARROWUP:
				new = max(self.index - 1, 0)
				self.remove_highlight(self.index)
				self.set_highlight(new)
				self.index = new

			case keys.PGDOWN:
				(ypad, _) = self.pad.getmaxyx()
				new = min(self.index + self.ysize, ypad - 1)
				self.remove_highlight(self.index)
				self.set_highlight(new)
				self.index = new

			case keys.PGUP:
				new = max(self.index - self.ysize, 0)
				self.remove_highlight(self.index)
				self.set_highlight(new)
				self.index = new

			case _:
				pass

	def refresh(self):
		"""
		refreshes and redraws the pad to the screen
		"""
		
		## calculating the offset from the top
		scroll_mod = 0
		if self.index > (self.ysize - 1):
			scroll_mod = min(self.index - self.ysize + 1, len(self.data) - self.ysize)
	
		# refreshing the pad
		self.pad.refresh(scroll_mod, 0, self.yoffset, self.xoffset, self.ysize - 1, self.xsize)


