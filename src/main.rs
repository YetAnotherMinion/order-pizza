extern crate ncurses;

static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

fn main() {
    /* Setup ncurses */
    ncurses::initscr();
    ncurses::raw();

    /* Allow use of extended keyboard */
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::noecho();

    /* Set the cursor to be invisible */
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info */
    ncurses::printw("Use the arrow keys to move");
    ncurses::mvprintw(ncurses::LINES - 1, 0, "Press F1 to exit");
    ncurses::refresh();

    /* Get the screen bounds */
    let mut max_x = 0;
    let mut max_y = 0;
    ncurses::getmaxyx(ncurses::stdscr, &mut max_y, &mut max_x);

    /* Start in the center */
    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;
    let mut win = create_win(start_y, start_x);
    
    let mut ch = ncurses::getch();
    while ch != ncurses::KEY_F(1) {
        match ch {
            ncurses::KEY_LEFT => {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            ncurses::KEY_RIGHT => {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            ncurses::KEY_UP => {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            ncurses::KEY_DOWN => {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            },
            _ => {}
        }
        ch = ncurses::getch();
    }

    ncurses::endwin();
}

fn create_win(start_y: i32, start_x: i32) -> ncurses::WINDOW {
    let win = ncurses::newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    ncurses::box_(win, 0, 0);
    ncurses::wrefresh(win);
    win
}

fn destroy_win(win: ncurses::WINDOW) {
    let ch = ' ' as ncurses::chtype;
    ncurses::wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    ncurses::wrefresh(win);
    ncurses::delwin(win);
}
