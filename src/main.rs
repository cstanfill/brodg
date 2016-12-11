extern crate pancurses;
extern crate brodg;

use pancurses::{initscr, endwin};

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.refresh();
  endwin();
}
