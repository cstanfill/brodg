extern crate pancurses;
extern crate brodg;

use brodg::data;
use pancurses::{initscr, endwin};

fn main() {
  let window = initscr();
  window.printw("Hello Rust");
  window.refresh();
  window.getch();
  endwin();
}
