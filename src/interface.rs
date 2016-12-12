extern crate pancurses;

use contract::{Seat, Contract, ContractDoubled};
use data::{Table, Entry};
use parse::{parse_input, get_error_cursor};
use score::Score;
use self::pancurses::{Input, Window};

const CURSOR_NORMAL     : i16 = 1;
const CURSOR_ERROR      : i16 = 3;
const CURSOR_DOUBLED    : i16 = 2;
const CURSOR_VULNERABLE : i16 = 2;
const CURSOR_REDOUBLED  : i16 = 4;
const CURSOR_CONTRACT   : i16 = 5;
const CURSOR_UNDERTRICK : i16 = 2;
const CURSOR_ENTERING   : i16 = 6;

enum EntryField {
    Number,
    Name,
    Vulnerability,
    Contract,
    Result,
}

enum Selection {
    NameSelect(Seat),
    // On an entry
    FieldSelect(i32, EntryField),
}

struct InputState {
    selection_ : Selection,
    entry_ : Option<String>,
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
            selection_ : Selection::NameSelect(Seat::North),
            entry_ : None,
        }
    }
}
pub struct Interface {
    // Contract-related
    table_: Table,
    dealer_ : Seat,
    entries_ : Vec<Entry>,
    input_state_ : Option<InputState>,

    root_window_ : Window,

    table_window_ : Window,
    table_border_window_ : Window,

    values_window_ : Window,
    values_border_window_ : Window,

    entries_window_ : Window,
    entries_border_window_ : Window,
}

fn center_pad(window : &Window, text : &str, y : i32) -> i32 {
    window.mvaddstr(y, (window.get_max_x() - text.len() as i32) / 2, text)
}

fn right_justify(window : &Window, text : &str, y : i32) -> i32 {
    window.mvaddstr(y, window.get_max_x() - text.len() as i32, text)
}

fn left_justify(window : &Window, text : &str, y : i32) -> i32 {
    window.mvaddstr(y, 0, text)
}

fn shrink(window : &Window) -> Window {
    window.derwin(window.get_max_y() - 2, window.get_max_x() - 2, 1, 1).unwrap()
}

fn border(window : &Window) {
    window.border('|','|','-','-','+','+','+','+');
}

fn draw_input(input : &str, window : Window, y : i32, x : i32) {
    let (parse_err, contract) = match parse_input(&input) {
        Ok(c)  => (None,    Some(c)),
        Err(e) => (Some(e), None),
    };
    let error_loc = get_error_cursor(parse_err);
    window.mv(y, x);
    window.color_set(CURSOR_NORMAL);
    for (i, c) in input.chars().enumerate() {
        if Some(i) == error_loc {
            window.color_set(CURSOR_ERROR);
        }
        window.addch(c);
    }
}

impl Interface {
    pub fn new(root_window : Window) -> Interface {
        let tablewin = root_window.derwin(9, 15, 1, 0).unwrap();
        let valueswin = root_window.derwin(10, 15, 0, 14).unwrap();
        let entrieswin = root_window.derwin(40, 40, 10, 0).unwrap();
        Interface {
            table_ : Table::new(),
            dealer_ : Seat::North,
            entries_ : Vec::new(),
            input_state_ : None,
            root_window_ : root_window,
            table_window_ : shrink(&tablewin),
            table_border_window_ : tablewin,
            values_window_ : shrink(&valueswin),
            values_border_window_ : valueswin,
            entries_window_ : shrink(&entrieswin),
            entries_border_window_ : entrieswin,
        }
    }

    pub fn redraw(&self) {
        self.draw_borders();
        self.draw_table();
        self.refresh();
    }

    pub fn get_input(&mut self) -> bool {
        let c = match self.root_window_.getch() {
            Some(Input::Character(cc)) => cc,
            None => { return false; }
            _    => { return true; }
        };
        match c {
            '\t' => self.cycle_input(),
            '\n' => self.enter_input(),
            _    => (),
        };
        return true;
    }

    fn cycle_input(&mut self) {
        if self.input_state_.is_none() {
            self.input_state_ = Some(InputState::new());
        }
    }

    fn enter_input(&mut self) {
    }

    fn north(&self) -> &str {
        self.table_.get_player(Seat::North)
    }

    fn south(&self) -> &str {
        self.table_.get_player(Seat::South)
    }

    fn east(&self) -> &str {
        self.table_.get_player(Seat::East)
    }

    fn west(&self) -> &str {
        self.table_.get_player(Seat::West)
    }

    fn draw_borders(&self) {
        border(&self.values_border_window_);
        border(&self.table_border_window_);
        border(&self.entries_border_window_);
    }

    fn draw_table(&self) {
        let table_win = &self.table_window_;
        let midpoint = table_win.get_max_y() / 2;
        center_pad(&table_win, self.north(), 0);
        center_pad(&table_win, self.south(), table_win.get_max_y() - 1);
        right_justify(&table_win, self.east(), midpoint);
        left_justify(&table_win, self.west(), midpoint);
    }

    fn draw_contract_value(&self, contract : &Contract, is_vulnerable : bool) {
        let val_win = &self.values_window_;
        val_win.color_set(
            match contract.doubled {
                ContractDoubled::Undoubled => CURSOR_CONTRACT,
                ContractDoubled::Doubled   => CURSOR_DOUBLED,
                ContractDoubled::Redoubled => CURSOR_REDOUBLED,
            });
        val_win.mvaddstr(1, 1, &contract.to_string());

        let number = contract.number.into_i32();
        let scorer = Score::from_contract(*contract, is_vulnerable);
        val_win.color_set(
            if is_vulnerable { CURSOR_VULNERABLE } else { CURSOR_NORMAL });
        for i in 0..8 - number {
            val_win.mv(2 + i, 2);
            val_win.addstr(&format!("{} {:5}",
                                    i + number, scorer.score_result(i)));
        }
        val_win.color_set(CURSOR_NORMAL);
    }

    fn draw_entry(&self, entry : &Entry) {
        let entry_win = &self.entries_window_;
        entry_win.addstr(&format!("{:<10}", entry.name()));
        entry_win.addch('|');
        entry_win.addch(if entry.is_vulnerable() { 'V' } else { ' ' });
        entry_win.addch('|');
        match entry.contract() {
            Some(c) => entry_win.addstr(&format!("{:6}", c.to_string())),
            None    => entry_win.addstr("      "),
        };
        entry_win.addch('|');
        match entry.result() {
            Some(c) => entry_win.addstr(&format!("{:+3}", c)),
            None    => entry_win.addstr("   "),
        };
        entry_win.addch('|');
        match entry.value() {
            Some(v) =>
                match entry.declarer() {
                    Some(Seat::North) | Some(Seat::South) =>
                        entry_win.addstr(&format!("{:+5}|     ", v)),
                        Some(Seat::East)  | Some(Seat::West)  =>
                            entry_win.addstr(&format!("     |{:+5}", v)),
                            None => entry_win.addstr("     |     "),
                },
                None => entry_win.addstr("     |     ")
        };
        entry_win.addch('|');
        entry_win.addch('\n');
    }

    fn refresh(&self) {
        self.values_border_window_.refresh();
        self.entries_border_window_.refresh();
        self.table_border_window_.refresh();
        self.values_window_.refresh();
        self.entries_window_.refresh();
        self.table_window_.refresh();
        self.values_window_.refresh();
        self.values_window_.refresh();
        self.root_window_.refresh();
    }
}
