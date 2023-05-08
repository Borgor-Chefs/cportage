pub trait Scrollable {
    /// Sets [`ncurses::A_REVERSE`] to a given location on a [`ncurses::WINDOW`]
    fn set_highlight(&self, i: i32) -> Result<(), i32>;
    /// Sets [`ncurses::A_NORMAL`] to a given location on a [`ncurses::WINDOW`]
    fn remove_highlight(&self, i: i32) -> Result<(), i32>;
}

