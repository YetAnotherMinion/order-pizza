extern crate ncurses;

use ncurses::*;

#[cfg(feature="menu")]
fn main() {
    /* Intialize curses */
    ncurses::initscr();
    ncurses::start_color();
    ncurses::cbreak();
    ncurses::noecho();
    /* Make the cursor invisible */
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    ncurses::keypad(ncurses::stdscr, true);
    ncurses::init_pair(1, ncurses::COLOR_RED, ncurses::COLOR_BLACK);

    /* Create items */
    let mut items: Vec<ncurses::ITEM> = Vec::new();
    items.push(new_item("Choice 1", "Choice 1 description"));
    items.push(ncurses::new_item("Choice 2", "Choice 2 description"));
    items.push(ncurses::new_item("Choice 3", "Choice 3 description"));
    items.push(ncurses::new_item("Choice 4", "Choice 4 description"));
    items.push(ncurses::new_item("Choice 5", "Choice 5 description"));
    
    /* Crate menu */
    let my_menu = ncurses::new_menu(&mut items);
    ncurses::menu_opts_off(my_menu, ncurses::O_SHOWDESC);
    
    let my_menu_win = ncurses::newwin(9, 18, 4, 4);
    ncurses::keypad(my_menu_win, true);

    /* Set main window and sub window */
    ncurses::set_menu_win(my_menu, my_menu_win);
    ncurses::set_menu_sub(my_menu, ncurses::derwin(my_menu_win, 5, 0, 2, 2));

    /* Set menu mark to the string " * " */
    ncurses::set_menu_mark(my_menu_win, " * ");
    ncurses::box_(my_menu_win, 0, 0);
    ncurses::mvprintw(ncurses::LINES - 3, 0, "Press <ENTER> to see the option selected");
    ncurses::mvprintw(ncurses::LINES - 2, 0, "F1 to exit");
    ncurses::refresh();

    /* Post the menu */
    ncurses::post_menu(my_menu);
    ncurses::wrefresh(my_menu_win);

    let mut ch = ncurses::getch();
    while ch != ncurses::KEY_F(1) {
        match ch {
            ncurses::KEY_UP => {
                ncurses::menu_driver(my_menu, ncurses::REQ_UP_ITEM);
            },
            ncurses::KEY_DOWN => {
                ncurses::menu_driver(my_menu, ncurses::REQ_UP_ITEM);
            },
            10 => { /* Enter key */
                ncurses::mv(20, 0);
                ncurses::clrtoeol();
                ncurses::mvprintw(20, 0, &format!("Item selected is: {}", ncurses::item_name(ncurses::current_item(my_menu)))[..]);
                ncurses::pos_menu_cursor(my_menu);
            },
            _ => {}
        }
        ncurses::wrefresh(my_menu_win);
        ch = ncurses::getch();
    }

    ncurses::unpost_menu(my_menu);

    /* Free items */
    for &item in items.iter() {
        free_item(item);
    }

    ncurses::free_menu(my_menu);

    ncurses::endwin();
}

#[cfg(not(feature="menu"))]
fn main() {
    println!("Hello world");
}

