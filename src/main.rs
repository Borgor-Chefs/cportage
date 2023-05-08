// global modules
extern crate pancurses;

// global imports
use pancurses::*;

// local modules
mod core;
mod gui;

// local imports
use self::core::data::random_data;
use self::gui::display::Display;

fn main() {
	// making the root window
	let root: Window = initscr();
	noecho();
	cbreak();

	// creating a display object from the main window
	let mut d = Display::new(&root, String::from("Title"), random_data(50, 32));
    d.keypad(true);
	d.refresh();

	// awaiting keyboard input
    loop {
        let i = d.getch();
        match i {
            Some(Input::KeyF1) => { break; },
            Some(Input::KeyDown) => { d.scroll_down(); },
            Some(Input::KeyUp) => { d.scroll_up(); },
            _ => (),
        }
        d.refresh();
    }

	// terminating the window
	endwin();
}

