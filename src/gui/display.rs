#![allow(dead_code)]

extern crate pancurses;

use super::{pad::Pad, scrollable::Scrollable};
use pancurses::*;

#[derive(Debug)]
pub struct Display {
	parent: Option<Box<Display>>,
	pad: Pad,
	data: Vec<String>,
	title: String,
	xoffset: i32,
	yoffset: i32,
	xsize: i32,
	ysize: i32,

	// scrolling functionality
	index: i32,
}

impl Display {
	/// Creates a new [`Display`] object, this acts as the root [`Display`].
	pub fn new(win: &Window, title: String, data: Vec<String>) -> Self {
		// getting max x& y
		let (y, x) = win.get_max_yx();

		// creating the pad object
		let pad: Pad = match Pad::new(std::cmp::max(y, data.len() as i32), x / 3) {
			Ok(p) => p,
			Err(e) => {
				endwin();
				panic!("There was an error creating a pad\n{:?}", e);
			}
		};

		// creating struct
		let d = Display {
			parent: None,
			pad,
			data,
			title,
			xoffset: 0,
			yoffset: 0,
			xsize: x / 3,
			ysize: y,
			index: 0,
		};

		// drawing the contents of `Data`
		d.setup();

		// returning
		d
	}

	/// This function draws the contents of `Display.data` onto the pad
	fn setup(&self) {
		for i in 0..(self.data.len() as i32) {
			// borrowing the string from the array
			let s: &String = &self.data[i as usize];
			let l = s.len().try_into().unwrap_or(0);

			// writing the slice
			self.pad
				.mvaddnstr(self.yoffset + i, self.xoffset, s, l)
				.expect(&format!(
					"Error in display.setup() calling pad.mvaddnstr({}, {}, {}, {})",
					self.yoffset + i,
					self.xoffset,
					s,
					l
				));
		}

		// highlighting the first line
		self.pad.set_highlight(0).expect({
			endwin();
			"Error in display.setup(), highlighting line 0 of pad"
		});
	}

    /// Makes a call to the internal [`Pad`] to write a `n` bytes of a string to an `(y, x)` location.
    pub fn mvaddnstr(&self, y: i32, x: i32, s: &String, n: i32) -> Result<(), i32> {
        self.pad.mvaddnstr(y, x, s, n)
    }

	/// draws this [`Display`] to the screen.
	pub fn refresh(&self) {
		let scroll_mod = match self.index > (self.ysize - 1) {
			true => std::cmp::min(
				self.index - self.ysize + 1,
				self.data.len() as i32 - self.ysize,
			),
			false => 0,
		};

		self.pad
			.refresh(scroll_mod, 0, 0, 0, self.ysize - 1, self.xsize)
			.expect(&format!(
				"Error in display.refresh() calling pad.refresh({}, {}, {}, {}, {}, {})",
				scroll_mod,
				0,
				0,
				0,
				self.ysize - 1,
				self.xsize
			))
	}

	/// Calls the internal [`Pad`]'s getch.
	pub fn getch(&self) -> Option<Input> {
		self.pad
			.getch()
			.expect("Error in display.getch() calling pad.getch() = {}")
	}

	pub fn keypad(&self, state: bool) {
		self.pad
			.keypad(state)
			.expect("Error in display.keypad() calling pad.keypad()");
	}

    /// Makes a call to the internal `Pad` object which "moves" the cursor up one line (within
    /// the boundaries of the window.
	pub fn scroll_up(&mut self) {
		let new = std::cmp::max(self.index - 1, 0);

		// setting the old index to normal
		self.pad.remove_highlight(self.index).expect({
			endwin();
			&format!(
				"Error in display.scroll_up(), highlighting line {} of pad",
				self.index
			)
		});

		// setting the new index to normal
		self.pad.set_highlight(new).expect({
			endwin();
			&format!(
				"Error in display.scroll_up(), highlighting line {} of pad",
				new
			)
		});

		// storing the new index
		self.index = new;
	}

    /// Makes a call to the internal `Pad` object which "moves" the cursor down one line (within
    /// the boundaries of the window.
	pub fn scroll_down(&mut self) {
		let new = std::cmp::min(self.index + 1, self.pad.dimensions().0 - 1);

		// setting the old index to normal
		self.pad.remove_highlight(self.index).expect({
			endwin();
			&format!(
				"Error in display.scroll_down(), highlighting line {} of pad",
				self.index
			)
		});

		// setting the new index to normal
		self.pad.set_highlight(new).expect({
			endwin();
			&format!(
				"Error in display.scroll_down(), highlighting line {} of pad",
				new
			)
		});

		// storing the new index
		self.index = new;
	}
}

//// Trait implementations for [`Display`]

impl Drop for Display {
	fn drop(&mut self) {
		return match& self.parent {
			Some(win) => {
				drop(win);
			}
			None => {}
		};
	}
}

