extern crate pancurses;
extern crate brodg;

use pancurses::{initscr, init_pair, endwin};
use brodg::interface::Interface;
use std::panic;

const CURSOR_NORMAL     : i16 = 1;
const CURSOR_ERROR      : i16 = 3;
const CURSOR_DOUBLED    : i16 = 2;
const CURSOR_REDOUBLED  : i16 = 4;
const CURSOR_CONTRACT   : i16 = 5;
const CURSOR_ENTERING   : i16 = 6;

fn real_main() {
    let window = initscr();
    pancurses::start_color();
    pancurses::noecho();
    pancurses::cbreak();
    init_pair(CURSOR_NORMAL,    pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    init_pair(CURSOR_DOUBLED,   pancurses::COLOR_RED,   pancurses::COLOR_BLACK);
    init_pair(CURSOR_ERROR,     pancurses::COLOR_WHITE, pancurses::COLOR_RED);
    init_pair(CURSOR_REDOUBLED, pancurses::COLOR_BLUE,  pancurses::COLOR_BLACK);
    init_pair(CURSOR_CONTRACT,  pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
    init_pair(CURSOR_ENTERING,  pancurses::COLOR_WHITE, pancurses::COLOR_BLUE);
    window.refresh();
    let mut interface = Interface::new(window);
    loop {
        interface.redraw();
        if !interface.get_input() {
            break;
        }
    }
}

fn main() {
    let result = panic::catch_unwind(|| { real_main(); });
    endwin();
    println!("{:?}", result);
}
