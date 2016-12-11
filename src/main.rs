extern crate pancurses;
extern crate brodg;

use pancurses::{initscr, init_pair, endwin, Input};
use brodg::data::*;
use brodg::parse::ContractParseError;

const CURSOR_NORMAL     : i16 = 1;
const CURSOR_ERROR      : i16 = 3;
const CURSOR_DOUBLED    : i16 = 2;
const CURSOR_VULNERABLE : i16 = 2;
const CURSOR_REDOUBLED  : i16 = 4;
const CURSOR_CONTRACT   : i16 = 5;
const CURSOR_UNDERTRICK : i16 = 2;

fn get_error_cursor(e : Option<ContractParseError>) -> Option<usize> {
    match e {
        None | Some(ContractParseError::Incomplete)     => None,
        Some(ContractParseError::InvalidNumber(_))      => Some(0),
        Some(ContractParseError::InvalidSuit(_))        => Some(1),
        Some(ContractParseError::InvalidTrailing(i, _)) => Some(i),
    }
}

fn parse_contract_nice(input : &str) -> Result<Contract, ContractParseError> {
    input.to_uppercase().parse()
}

fn draw_table(window : &pancurses::Window, table : &Table,
              dealer : Seat) {
    let north = table.get_player(Seat::North);
    if dealer == Seat::North {
        window.attron(pancurses::A_BOLD);
    }
    window.mvaddstr(1, (window.get_max_x() - north.len() as i32) / 2, north);
    window.attroff(pancurses::A_BOLD);

    let east = table.get_player(Seat::East);
    if dealer == Seat::South {
        window.attron(pancurses::A_BOLD);
    }
    window.mvaddstr(window.get_max_y() / 2,
                    (window.get_max_x() - east.len() as i32 - 1), east);
    window.attroff(pancurses::A_BOLD);

    let south = table.get_player(Seat::South);
    if dealer == Seat::South {
        window.attron(pancurses::A_BOLD);
    }
    window.mvaddstr(window.get_max_y() - 2,
                    (window.get_max_x() - south.len() as i32) / 2, south);
    window.attroff(pancurses::A_BOLD);

    let west = table.get_player(Seat::West);
    if dealer == Seat::South {
        window.attron(pancurses::A_BOLD);
    }
    window.mvaddstr(window.get_max_y() / 2, 1, west);
    window.attroff(pancurses::A_BOLD);
}

fn draw_contract_value(window : &pancurses::Window, contract : Contract,
                       is_vulnerable : bool) {
    window.color_set(
        match contract.doubled {
            ContractDoubled::Undoubled => CURSOR_CONTRACT,
            ContractDoubled::Doubled => CURSOR_DOUBLED,
            ContractDoubled::Redoubled => CURSOR_REDOUBLED,
        });
    window.mvaddstr(1, 1, &contract.to_string());

    let number = contract.number.into_i32();
    for i in 0..8 - number {
        window.mv(2 + i, 2);
        window.color_set(CURSOR_NORMAL);
        window.addstr(&format!("{}", i + number));
        if !is_vulnerable {
            window.attron(pancurses::A_BOLD);
        }
        window.addstr(&format!("{:5}", score_game(contract, i, false)));
        if is_vulnerable {
            window.attron(pancurses::A_BOLD);
        } else {
            window.attroff(pancurses::A_BOLD);
        }
        window.color_set(CURSOR_VULNERABLE);
        window.attroff(pancurses::A_BOLD);
        window.addstr(&format!("{:5}", score_game(contract, i, true)));
    }
    window.color_set(CURSOR_NORMAL);
}

fn main() {
  let window = initscr();
  pancurses::start_color();
  pancurses::noecho();
  pancurses::cbreak();
  init_pair(CURSOR_NORMAL,    pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
  init_pair(CURSOR_DOUBLED,   pancurses::COLOR_RED,   pancurses::COLOR_BLACK);
  init_pair(CURSOR_ERROR,     pancurses::COLOR_WHITE, pancurses::COLOR_RED);
  init_pair(CURSOR_REDOUBLED, pancurses::COLOR_BLUE,  pancurses::COLOR_BLACK);
  init_pair(CURSOR_CONTRACT,  pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
  window.refresh();
  let mut input = String::new();
  let mut table = Table::new();
  let tablewin = window.subwin(9, 15, 1, 0).unwrap();
  let contractwin = window.subwin(10, 15, 0, 14).unwrap();
  loop {
      window.clear();
      contractwin.border('|','|','-','-','+','+','+','+');
      tablewin.border('|','|','-','-','+','+','+','+');
      draw_table(&tablewin, &table, Seat::North);
      let (parse_err, contract) = match parse_contract_nice(&input) {
          Ok(c)  => (None,    Some(c)),
          Err(e) => (Some(e), None),
      };
      if let Some(c) = contract {
          draw_contract_value(&contractwin, c, false);
      };
      let error_loc = get_error_cursor(parse_err);
      window.mv(0, 0);
      window.color_set(CURSOR_NORMAL);
      for (i, c) in input.chars().enumerate() {
          if Some(i) == error_loc {
              window.color_set(CURSOR_ERROR);
          }
          window.addch(c);
      }

      window.refresh();

      match window.getch() {
          None | Some(Input::Character('q'))  => break,
          Some(Input::Character('\n'))        => input = String::new(),
          Some(Input::Character('\x7f')) => { input.pop(); continue},
          Some(Input::Character(c)) => input.push(c),
          Some(_) => continue,
      }
  }
  endwin();
}
