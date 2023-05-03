extern crate ncurses;

mod gui;
mod core;
use ncurses::*;

use self::core::data::random_data;
use self::gui::display::Display;

fn main() {
		/* Setup ncurses. */
		initscr();
		raw();

        // the main window
        let win: WINDOW = stdscr();

		/* Allow for extended keyboard (like F1). */
		keypad(win, true);
		noecho();

        // creating a display object from the main window
        let d = Display::from_window(win, String::from("Title"), random_data(50, 16));
        d.draw();

        wrefresh(win);

        wgetch(win);
		endwin();
}

