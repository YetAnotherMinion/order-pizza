extern crate ncurses;


fn main() {
    /* Start ncurses */
    ncurses::initscr();

    /* Print to the back buffer */
    ncurses::printw("Hello, world!");

    /* Update the screen */
    ncurses::refresh();

    /* Wait for a key press */
    ncurses::getch();

    /* Terminate ncurses */
    ncurses::endwin();
}
