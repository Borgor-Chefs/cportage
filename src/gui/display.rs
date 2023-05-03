#![allow(dead_code)]

extern crate ncurses;

use ncurses::*;

#[derive(Debug)]
enum DisplayType {
	Regular(Box<Display>),
	None,
}

#[derive(Debug)]
pub struct Display {
	parent: DisplayType,
	data: Vec<String>,
	title: String,
	xoffset: i32,
	yoffset: i32,
	xsize: i32,
	ysize: i32,
}

impl Display {
	pub fn new(win: WINDOW, title: String, data: Vec<String>) -> Self {
		// this makes a display from the window object
		// getting max x& y
		let mut x = 0;
		let mut y = 0;
		getmaxyx(win, &mut y, &mut x);

		// creating struct
		Display {
			parent: DisplayType::None,
			data,
			title,
			xoffset: 0,
			yoffset: 0,
			xsize: x,
			ysize: y,
		}
	}

	pub fn sized(
		win: WINDOW,
		title: String,
		xpos: i32,
		ypos: i32,
		xlen: i32,
		ylen: i32,
	) -> Display {
		// this makes a display from the window object
		// getting max x& y
		let mut x = 0;
		let mut y = 0;
		getmaxyx(win, &mut y, &mut x);

		// taking the minimum values of the dimensions
		let xreal = std::cmp::min(xlen, x);
		let yreal = std::cmp::min(ylen, y);

		// creating struct
		Display {
			parent: DisplayType::None,
			data: vec![],
			title,
			xoffset: xpos,
			yoffset: ypos,
			xsize: xreal,
			ysize: yreal,
		}
	}

	/// draws this [`Display`] to the screen.
	pub fn draw(&self) {
        // creating the window
		let win = newwin(self.ysize, self.xsize, self.yoffset, self.xoffset);
        let iter_cap: i32 = std::cmp::min(self.ysize, self.data.len().try_into().unwrap());

        // drawing all display's contents
        for i in 0..iter_cap {
            mvprintw(self.yoffset + i, self.xoffset, &self.data[i as usize]);
        }

        // refreshing the screen
		wrefresh(win);
	}
}

