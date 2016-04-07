extern crate ncurses;

use std::{char, env, fs};
use std::path::Path;
use std::io::{Read, Bytes};
use std::iter::Peekable;

/* Individual color handles */
static COLOR_BACKGROUND: i16 = 16;
static COLOR_FOREGROUND: i16 = 17;
static COLOR_KEYWORD: i16 = 18;
static COLOR_TYPE: i16 = 19;
static COLOR_STORAGE: i16 = 20;
static COLOR_COMMENT: i16 = 21;
static COLOR_STRING: i16 = 22;
static COLOR_CHAR: i16 = 23;
static COLOR_NUMBER: i16 = 24;

/* Color pairs, foreground && background */
static COLOR_PAIR_DEFAULT: i16 = 1;
static COLOR_PAIR_KEYWORD: i16 = 2;
static COLOR_PAIR_TYPE: i16 = 3;
static COLOR_PAIR_STORAGE: i16 = 4;
static COLOR_PAIR_COMMENT: i16 = 5;
static COLOR_PAIR_STRING: i16 = 6;
static COLOR_PAIR_CHAR: i16 = 7;
static COLOR_PAIR_NUMBER: i16 = 8;

/* Word delimeters */
static WORD_LIMITS: &'static [u8] = &
[
    ' ' as u8,
    '(' as u8,
    ')' as u8,
    ':' as u8,
    ';' as u8,
    '&' as u8,
    '+' as u8,
    '-' as u8,
    ',' as u8,
    '.' as u8,
    '@' as u8,
    '~' as u8,
    '\\' as u8,
    '\n' as u8,
    '\r' as u8,
    '\0' as u8,
    !0 as u8,
];

struct Pager {
    file_reader: Peekable<Bytes<fs::File>>,

    in_comment: bool,
    in_string: bool,
    in_char: bool,

    screen_width: i32,
    screen_height: i32,
    cur_x: i32,
    cur_y: i32,
}

impl Pager {
    pub fn new() -> Pager {
        Pager {
            file_reader: open_file().bytes().peekable(),
            in_comment: false,
            in_string: false,
            in_char: false,
            screen_width: 0,
            screen_height: 0,
            cur_x: 0,
            cur_y: 0,
        }
    }

    pub fn initialize(&mut self) {
        /* Start ncurses */
        ncurses::initscr();
        ncurses::keypad(ncurses::stdscr, true);
        ncurses::noecho();

        /* Start colors */
        /* this appears to use single byte rgb and then multiplies it by
         * four so that the color is 30 bit color instead of 24 bit color ?? */
        ncurses::start_color();
        ncurses::init_color(COLOR_BACKGROUND, 0, 43 * 4, 54 * 4);
        ncurses::init_color(COLOR_FOREGROUND, 142 * 4, 161 * 4, 161 * 4);
        ncurses::init_color(COLOR_KEYWORD, 130 * 4, 151 * 4, 0);
        ncurses::init_color(COLOR_TYPE, 197 * 4, 73 * 4, 27 * 4);
        ncurses::init_color(COLOR_STORAGE, 219 * 4, 51 * 4, 47 * 4);
        ncurses::init_color(COLOR_COMMENT, 33 * 4, 138 * 4, 206 * 4);
        ncurses::init_color(COLOR_STRING, 34 * 4, 154 * 4, 142 * 4);
        ncurses::init_color(COLOR_CHAR, 34 * 4, 154 * 4, 142 * 4);
        ncurses::init_color(COLOR_NUMBER, 236 * 4, 107 * 4, 83 * 4);

        ncurses::init_pair(COLOR_PAIR_DEFAULT, COLOR_FOREGROUND, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_KEYWORD, COLOR_KEYWORD, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_TYPE, COLOR_TYPE, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_STORAGE, COLOR_STORAGE, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_COMMENT, COLOR_COMMENT, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_STRING, COLOR_STRING, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_CHAR, COLOR_CHAR, COLOR_BACKGROUND);
        ncurses::init_pair(COLOR_PAIR_NUMBER, COLOR_NUMBER, COLOR_BACKGROUND);

        /* Set the window's background color */
        ncurses::bkgd(' ' as ncurses::chtype | ncurses::COLOR_PAIR(COLOR_PAIR_DEFAULT) as ncurses::chtype);

        /* Get the screen bounds */
        ncurses::getmaxyx(ncurses::stdscr, &mut self.screen_height, &mut self.screen_width);
    }

    pub fn read_word(&mut self) -> (String, char) {
        let mut s: Vec<u8> = vec![];
        let mut ch: u8 = self.file_reader.next()
            .unwrap()
            .ok()
            .expect("Unable to read byte");
        /* Read until we hit a word delimeter */
        while !WORD_LIMITS.contains(&ch) {
            s.push(ch);
            ch = self.file_reader.next()
                .unwrap()
                .ok()
                .expect("Unable to read byte");
        }

        /* Return the word string and the terminating delimeter */
        match char::from_u32(ch as u32) {
            Some(ch) => (String::from_utf8(s).ok().expect("utf-8 conversion failed"), ch),
            None => (String::from_utf8(s).ok().expect("utf-8 conversion failed"), ' '),
        }
    }

    /* Returns the attribute the given word requires */
    pub fn highlight_word(&mut self, word: &str) -> ncurses::attr_t {
        /* Match block comments */
        if self.in_comment && !word.contains("*/") {
            return ncurses::COLOR_PAIR(COLOR_PAIR_COMMENT);
        } else if self.in_comment && word.contains("*/") {
            self.in_comment = false;
            return ncurses::COLOR_PAIR(COLOR_PAIR_COMMENT);
        }
        /* Match string literals */
        if !self.in_char {
            if self.in_string && !word.contains("\"") {
                return ncurses::COLOR_PAIR(COLOR_PAIR_STRING);
            } else if self.in_string && word.contains("\"") {
                self.in_string = false;
                return ncurses::COLOR_PAIR(COLOR_PAIR_STRING);
            } else if !self.in_string && word.contains("\"") {
                /* If the same quote is found from either direction
                 * then it's the only quote in the string */
                if word.find('\"') == word.rfind('\"') {
                    self.in_string = true;
                }
                return ncurses::COLOR_PAIR(COLOR_PAIR_STRING);
            }
        }
        /* Match character literals */
        if self.in_char && !word.contains("\'") {
            return ncurses::COLOR_PAIR(COLOR_PAIR_CHAR);
        } else if self.in_char && word.contains("\'") && !word.contains("static") {
            /* If the same quote is found from either direction
             * the it's the only quote in the string */
            if word.find('\'') == word.rfind('\'') {
                return ncurses::COLOR_PAIR(COLOR_PAIR_CHAR);
            }
        }
        /* Trim the word of all delimeters */
        let word = word.trim_matches(|ch: char|
                                     { WORD_LIMITS.contains(&(ch as u8)) });
        if word.len() == 0 {
            return 0;
        }
        /* If it starts with a number then it is a number */
        if word.as_bytes()[0] >= '0' as u8 && word.as_bytes()[0] <= '9' as u8 {
            return ncurses::COLOR_PAIR(COLOR_PAIR_NUMBER);
        }

        match word {
            /* Key words */
            "break" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "continue" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "do" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "else" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "extern" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "in" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "if" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "impl" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "let" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "log" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "loop" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "match" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "once" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "priv" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "pub" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "return" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "unsafe" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "while" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "use" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "mod" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "trait" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "struct" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "enum" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "type" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            "fn" => {ncurses::COLOR_PAIR(COLOR_PAIR_KEYWORD)},
            /* Types */
            "int" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "uint" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "char" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "bool" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "u8" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "u16" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "u32" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "u64" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "i16" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "i32" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "i64" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "f32" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "f64" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "str" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "self" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            "Self" => {ncurses::COLOR_PAIR(COLOR_PAIR_TYPE)},
            /* Storage */
            "const" => {ncurses::COLOR_PAIR(COLOR_PAIR_STORAGE)},
            "mut" => {ncurses::COLOR_PAIR(COLOR_PAIR_STORAGE)},    
            "ref" => {ncurses::COLOR_PAIR(COLOR_PAIR_STORAGE)},
            "static" => {ncurses::COLOR_PAIR(COLOR_PAIR_STORAGE)},
            /* Not something we need to highlight */
            _ => 0,
        }
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        /* Final prompt before closing */
        ncurses::mv(self.screen_height - 1, 0);
        prompt();
        ncurses::endwin();
    }
}

fn prompt() {
    ncurses::attron(ncurses::A_BOLD());
    ncurses::printw("<-Press Space->");
    while ncurses::getch() != ' ' as i32 {}
    ncurses::attroff(ncurses::A_BOLD());
}

fn open_file() -> fs::File {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\t{} <rust file>", args[0]);
        println!("Example:\n\t{} src/main.rs", args[0]);
        panic!("Exiting");
    }
    let reader = fs::File::open(Path::new(&args[1]));
    reader.ok().expect("Unable to open file")
}

fn main() {

}
