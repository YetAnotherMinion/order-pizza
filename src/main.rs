extern crate ncurses;

use std::char;

fn main() {
    /* Start ncurses */
    ncurses::initscr();
    ncurses::raw();
    
    /* Allow for extended keyboard (like F1) */
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::noecho();


    /* Write prompt for a character to buffer */
    ncurses::printw("Enter a character: ");

    /* Wait for a key press */
    let ch = ncurses::getch();
    if ch == ncurses::KEY_DOWN {
        /* Enable attributes and output message */
        ncurses::attron(ncurses::A_BOLD() | ncurses::A_BLINK());
        ncurses::printw("\nDown Arrow Key");
        ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
        ncurses::printw(" pressed");
    } else {
        /* Enable attributes and output message */
        ncurses::attron(ncurses::A_BOLD() | ncurses::A_BLINK());
        ncurses::printw(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
        ncurses::attroff(ncurses::A_BOLD() | ncurses::A_BLINK());
    }

    /* Refresh, showing the previous message */
    ncurses::refresh();
    
    /* Terminate ncurses */
    ncurses::getch();
    ncurses::endwin();
}
