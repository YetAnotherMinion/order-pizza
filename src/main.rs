extern crate ncurses;

use std::char;
use std::env;
use std::io::Read;
use std::fs;
use std::path::Path;

fn open_file() -> fs::File {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\t{} <rust file>", args[0]);
        println!("Example:\n\t{} src/main.rs", args[0]);
        panic!("Exiting");
    }

    let reader = fs::File::open(Path::new(&args[1]));
    /* Return the file contents */
    reader.ok().expect("Unable to open file")
}

fn prompt() {
    ncurses::printw("<-Press Any Key->");
    ncurses::getch();
}

fn main() {
    let reader = open_file().bytes();

    /* Start ncurses */
    ncurses::initscr();
    ncurses::raw();
    
    /* Allow for extended keyboard (like F1) */
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::noecho();

    /* Get the screen bounds */
    let mut max_x = 0;
    let mut max_y = 0;
    ncurses::getmaxyx(ncurses::stdscr, &mut max_y, &mut max_x);
    

    /* Read the whole file */
    for ch in reader {
        if ch.is_err() {
            break;
        }
        let ch = ch.unwrap();

        /* Get the current position on screen */
        let mut cur_x = 0;
        let mut cur_y = 0;
        ncurses::getyx(ncurses::stdscr, &mut cur_y, &mut cur_x);

        if cur_y == (max_y - 1) {
            /* Status bar at the bottom */
            prompt();

            /* Once a key is pressed, clear the screen and continue */
            ncurses::clear();
            ncurses::mv(0, 0);
        } else {
            ncurses::addch(ch as ncurses::chtype);
        }
    }
    
    /* Terminate ncurses */
    ncurses::mv(max_y - 1, 0);
    ncurses::getch();
    ncurses::endwin();
}
