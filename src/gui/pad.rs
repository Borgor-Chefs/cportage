#![allow(dead_code)]

extern crate ncurses;

use ncurses::{mvwaddnstr, newpad, prefresh, wgetch, WINDOW};

#[derive(Debug)]
pub struct Pad {
	pub ptr: WINDOW,
	lines: i32,
	cols: i32,
}

impl Pad {
	/// Creates an ncurses `pad` via. [`newpad`], returns a [`Result`] object containing
	/// the result of the ncurses function call.
	pub fn new(lines: i32, cols: i32) -> Result<Self, WINDOW> {
		let p: WINDOW = newpad(lines, cols);

		if p == (0xff as WINDOW) {
			return Err(p);
		}

		// returning the new pad object
		Ok(Pad {
			ptr: p,
			lines,
			cols,
		})
	}

	/// This is a bridge to ncurses' [`mvwaddnstr`], returns a [`Result`] object containing
	/// the result of the ncurses function call.
	pub fn mvaddnstr(&self, y: i32, x: i32, s: &String, n: i32) -> Result<(), i32> {
		let i = mvwaddnstr(self.ptr, y, x, s, n);

		// returning an error if there is any
		match i {
			0 => Ok(()),
			_ => Err(i),
		}
	}

	/// This is a bridge to ncurses' [`prefresh`], returns a [`Result`] object containing
	/// the result of the ncurses function call.
	pub fn refresh(&self, ymin: i32, xmin: i32, ymax: i32, xmax: i32) -> Result<(), i32> {
		let i = prefresh(self.ptr, 0, 0, ymin, xmin, ymax, xmax);

		// returning an error if there is any
		match i {
			0 => Ok(()),
			_ => Err(i),
		}
	}

	/// This is a bridge to ncurses' [`wgetch`], returns a [`Result`] object containing
	/// the result of the ncurses function call.
	pub fn getch(&self) -> Result<(), i32> {
		let i = wgetch(self.ptr);

		// returning an error if there is any
		match i {
			0xff => Err(i),
			_ => Ok(()),
		}
	}
}

