#![allow(dead_code)]

extern crate pancurses;

use super::pad::Pad;
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
			match self.pad.mvaddnstr(self.yoffset + i, self.xoffset, s, l) {
				Ok(_) => (),
				Err(e) => {
					endwin();
					panic!(
						"Error in display.setup() calling pad.mvaddnstr({}, {}, {}, {}) = {}",
						self.yoffset + i,
						self.xoffset,
						s,
						l,
						e
					);
				}
			};
		}
	}

	/// draws this [`Display`] to the screen.
	pub fn refresh(&self) {
		match self.pad.refresh(0, 0, self.ysize - 1, self.xsize) {
			Ok(_) => (),
			Err(e) => {
				endwin();
				println!(
					"Error in display.refresh() calling pad.refresh({}, {}, {}, {}) = {}",
					0,
					0,
					self.ysize - 1,
					self.xsize,
					e
				);
			}
		}
	}

	/// Calls the internal [`Pad`]'s getch.
	pub fn getch(&self) {
		match self.pad.getch() {
			Ok(_) => (),
			Err(e) => {
				endwin();
				panic!("Error in display.getch() calling pad.getch() = {}", e)
			}
		};
	}
}

