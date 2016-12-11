extern crate pancurses;
extern crate brodg;

use pancurses::{initscr, endwin, Input};
use brodg::data::score_game;
use brodg::data::Contract;
use brodg::data::ContractDoubled;
use brodg::parse::parse_contract;
use brodg::parse::ContractParseError;

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

fn main() {
  let window = initscr();
  pancurses::start_color();
  pancurses::noecho();
  pancurses::cbreak();
  pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
  pancurses::init_pair(2, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
  window.refresh();
  let mut input = String::new();
  loop {
      window.clear();
      let (parse_err, contract) = match parse_contract_nice(&input) {
          Ok(c)  => (None,    Some(c)),
          Err(e) => (Some(e), None),
      };
      if let Some(c) = contract {
          window.mv(2, 0);
          window.color_set(1);
          window.addstr(&c.to_string());
          let number = c.number.into_i32();

          for i in 0..8 - number {
              window.mv(3 + i, 0);
              window.color_set(1);
              window.addstr(&format!("{}", i + number));
              window.addstr(&format!("{:5}", score_game(c, i, false)));
              window.color_set(2);
              window.addstr(&format!("{:5}", score_game(c, i, true)));
          }
      };
      let error_loc = get_error_cursor(parse_err);
      window.mv(0, 0);
      window.color_set(1);
      for (i, c) in input.chars().enumerate() {
          if Some(i) == error_loc {
              window.color_set(2);
          }
          window.addch(c);
      }

      window.refresh();

      match window.getch() {
          None | Some(Input::Character('q'))  => break,
          Some(Input::Character('\n'))        => {
              input = String::new();
              continue
          },
          Some(Input::Character('\x7f')) => {
              input.pop();
              continue
          },
          Some(Input::Character(c)) => input.push(c),
          Some(_) => continue,
      }
  }
  endwin();
}
