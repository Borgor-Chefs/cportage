#![allow(dead_code)]

extern crate ncurses;

use ncurses::*;

#[derive(Debug)]
pub struct Display {
    parent: Option<Box<Display>>,
	window: WINDOW,
	data: Vec<String>,
	title: String,
	xoffset: i32,
	yoffset: i32,
	xsize: i32,
	ysize: i32,
}

impl Display {
	/// Creates a new [`Display`] object.
    pub fn new(title: String, data: Vec<String>) -> Self {
		// getting max x& y
		let mut x = 0;
		let mut y = 0;
		getmaxyx(stdscr(), &mut y, &mut x);

		// creating struct
		Display {
            parent: None,
			window: stdscr(),
			data,
			title,
			xoffset: 0,
			yoffset: 0,
			xsize: x,
			ysize: y,
		}
    }

    /// Creates a new [`Display`] object from a [`WINDOW`].
    pub fn from_window(win: WINDOW, title: String, data: Vec<String>) -> Self {
		// getting max x& y
		let mut x = 0;
		let mut y = 0;
		getmaxyx(win, &mut y, &mut x);

		// creating struct
		Display {
            parent: None,
			window: win,
			data,
			title,
			xoffset: 0,
			yoffset: 0,
			xsize: x,
			ysize: y,
		}
    }

	/// draws this [`Display`] to the screen.
	pub fn draw(&self) {
		// rendering the data in the window given to the function
		let row_cap: i32 = std::cmp::min(self.ysize, self.data.len().try_into().unwrap_or(0));

		// drawing all display's contents
		for i in 0..row_cap {
            // borrowing the string from the array
            let s: &String = &self.data[i as usize];

            // getting a string slice
            let sslice = &self.slice_upto_window(s);

            // writing the slice
			mvwprintw(self.window, self.yoffset + i, self.xoffset, sslice);
        }

        // refreshing the window
        wrefresh(self.window);
	}

    fn slice_upto_window(&self, s: &String) -> String {
        // taking the minimum value (don't want to write beyond the window)
        let len: usize = std::cmp::min(self.xsize as usize, s.len());

        // forming a slice and returning it
        String::from(&s[0..len])
    }
}

