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

		/* Allow for extended keyboard (like F1). */
		keypad(stdscr(), true);
		noecho();

		/* Status/help info. */
		addstr("Use the arrow keys to move");
		mvprintw(LINES() - 1, 0, "Press F1 to exit");
		refresh();

        let d = Display::new(stdscr(), String::from("Title"), random_data(50, 16));
        d.draw();

        getch();

		endwin();
}

