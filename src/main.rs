extern crate pancurses;
extern crate brodg;

use pancurses::{initscr, init_pair, endwin, Input};
use brodg::data::{Entry, Table};
use brodg::contract::{Contract, ContractDoubled, Seat};
use brodg::parse::ContractParseError;
use brodg::score::Score;

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
    let normal_scorer = Score::from_contract(contract, false);
    let vuln_scorer = Score::from_contract(contract, true);
    for i in 0..8 - number {
        window.mv(2 + i, 2);
        window.color_set(CURSOR_NORMAL);
        window.addstr(&format!("{}", i + number));
        if !is_vulnerable {
            window.attron(pancurses::A_BOLD);
        }
        window.addstr(&format!("{:5}", normal_scorer.score_result(i)));
        if is_vulnerable {
            window.attron(pancurses::A_BOLD);
        } else {
            window.attroff(pancurses::A_BOLD);
        }
        window.color_set(CURSOR_VULNERABLE);
        window.attroff(pancurses::A_BOLD);
        window.addstr(&format!("{:5}", vuln_scorer.score_result(i)));
    }
    window.color_set(CURSOR_NORMAL);
}

fn draw_entry(window : &pancurses::Window, entry : &Entry) {
    window.addstr(&format!("{:<10}", entry.name()));
    window.addch('|');
    window.addch(if entry.is_vulnerable() { 'V' } else { ' ' });
    window.addch('|');
    match entry.contract() {
        Some(c) => window.addstr(&format!("{:6}", c.to_string())),
        None    => window.addstr("      "),
    };
    window.addch('|');
    match entry.result() {
        Some(c) => window.addstr(&format!("{:+3}", c)),
        None    => window.addstr("   "),
    };
    window.addch('|');
    match entry.value() {
        Some(v) =>
            match entry.declarer() {
                Some(Seat::North) | Some(Seat::South) =>
                    window.addstr(&format!("{:+5}|     ", v)),
                Some(Seat::East)  | Some(Seat::West)  =>
                    window.addstr(&format!("     |{:+5}", v)),
                None => window.addstr("     |     "),
            },
        None => window.addstr("     |     ")
    };
    window.addch('|');
    window.addch('\n');
}

fn draw_setting_value(window : &pancurses::Window, contract : Contract,
                       is_vulnerable : bool) {
    let number = contract.number.into_i32();
    let scorer = Score::from_contract(contract, is_vulnerable);
    for i in 1..7 + number {
        window.mv(i, 1);
        window.color_set(CURSOR_UNDERTRICK);
        window.addstr(&format!("{:3}", -i));
        window.addstr(&format!("{:6}", scorer.score_result(-i)));
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
  let table = Table::new();
  let tablewin = window.subwin(9, 15, 1, 0).unwrap();
  let contractwin = window.subwin(10, 15, 0, 14).unwrap();
  let settingwin = window.subwin(15, 12, 0, 28).unwrap();
  let entrieswin = window.subwin(5, 40, 15, 0).unwrap();
  let mut e = Entry::new(&table, Seat::North, 1);
  e.set_contract("3NT".parse().unwrap());
  let mut f = Entry::new(&table, Seat::South, 2);
  f.set_contract("6NTXX".parse().unwrap());
  f.record(-1).unwrap();
  let mut g = Entry::new(&table, Seat::East, 3);
  g.set_contract("4H".parse().unwrap());
  g.record(1).unwrap();
  loop {
      window.clear();
      settingwin.border('|','|','-','-','+','+','+','+');
      contractwin.border('|','|','-','-','+','+','+','+');
      tablewin.border('|','|','-','-','+','+','+','+');
      entrieswin.mv(0,0);
      draw_entry(&entrieswin, &e);
      draw_entry(&entrieswin, &f);
      draw_entry(&entrieswin, &g);
      draw_table(&tablewin, &table, Seat::North);
      let (parse_err, contract) = match parse_contract_nice(&input) {
          Ok(c)  => (None,    Some(c)),
          Err(e) => (Some(e), None),
      };
      if let Some(c) = contract {
          draw_contract_value(&contractwin, c, false);
          draw_setting_value(&settingwin, c, false);
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
