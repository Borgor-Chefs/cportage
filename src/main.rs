
// global modules
extern crate pancurses;

// global imports
use pancurses::*;

// local modules
mod gui;
mod core;

// local imports
use self::core::data::random_data;
use self::gui::display::Display;

fn main() {
		// making the root window
        let root: Window = initscr();
		noecho();
        cbreak();

        // creating a display object from the main window
        let d = Display::new(&root, String::from("Title"), random_data(300, 32));
        d.refresh();

        // awaiting keyboard input
        d.getch();

        // terminating the window
		endwin();
}

