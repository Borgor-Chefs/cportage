#![allow(dead_code)]

extern crate ncurses;

use ncurses::{mvwaddnstr, newpad, prefresh, wgetch, WINDOW, mvwchgat, A_REVERSE, A_NORMAL, keypad};
use pancurses;
use super::scrollable::Scrollable;

#[derive(Debug)]
pub struct Pad {
	ptr: WINDOW,
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
	pub fn refresh(&self, ypad: i32, xpad: i32, ymin: i32, xmin: i32, ymax: i32, xmax: i32) -> Result<(), i32> {
		let i = prefresh(self.ptr, ypad, xpad, ymin, xmin, ymax, xmax);

		// returning an error if there is any
		match i {
			0xff => Err(i),
			_ => Ok(()),
		}
	}

	/// This is a bridge to ncurses' [`wgetch`], returns a [`Result`] object containing
    /// [`pancurses::Input`] on success, otherwise an error code from [`ncurses`]
	pub fn getch(&self) -> Result<Option<pancurses::Input>, i32> {
		let i = wgetch(self.ptr);

		// returning an error if there is any
		match i {
			0xff => Err(i),
			_ => Ok(match i > 255 {
				true => { Some(match i {
                        _ => pancurses::SPECIAL_KEY_CODES[std::cmp::max(i - 256, 0) as usize],
				    })
                },
				false => { Some(pancurses::Input::Character(char::from_u32(i as u32).unwrap_or(char::MAX))) }
			}),
		}
	}

	/// This is a bridge to ncurses' [`mvwchgat`], returns a [`Result`] object containing
	/// the result of the ncurses function call.
	pub fn mvchgat(&self, y: i32, x: i32, n: i32, attribute: u32, color: i16) -> Result<(), i32> {
		let i = mvwchgat(self.ptr, y, x, n, attribute, color);

		// returning an error if there is any
		match i {
			0xff => Err(i),
			_ => Ok(()),
		}
	}

	/// This is a bridge to ncurses' [`keypad`], returns a [`Result`] object containing
	/// the result of the ncurses function call.
	pub fn keypad(&self, state: bool) -> Result<(), i32> {
		let i = keypad(self.ptr, state);

		// returning an error if there is any
		match i {
			0xff => Err(i),
			_ => Ok(()),
		}
	}

    /// Returns the dimensions of the [`Pad`] in `(y, x)` format.
    pub fn dimensions(&self) -> (i32, i32) {
        (self.lines, self.cols)
    }
}

impl Scrollable for Pad {
	fn remove_highlight(&self, i: i32) -> Result<(), i32> {
		self.mvchgat(i, 0, self.cols, A_NORMAL(), 0)
	}

	fn set_highlight(&self, i: i32) -> Result<(), i32> {
		self.mvchgat(i, 0, self.cols, A_REVERSE(), 0)
	}
}


