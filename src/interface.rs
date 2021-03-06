extern crate pancurses;

use contract::{Seat, Contract, ContractDoubled};
use data::{Table, Entry};
use parse::{parse_input, get_error_cursor};
use score::Score;
use self::pancurses::{Input, Window};
use std::mem;

const CURSOR_NORMAL     : i16 = 1;
const CURSOR_ERROR      : i16 = 3;
const CURSOR_DOUBLED    : i16 = 2;
const CURSOR_VULNERABLE : i16 = 2;
const CURSOR_REDOUBLED  : i16 = 4;
const CURSOR_CONTRACT   : i16 = 5;
const CURSOR_UNDERTRICK : i16 = 2;
const CURSOR_ENTERING   : i16 = 6;

#[derive(Copy, Clone, PartialEq, Eq)]
enum EntryField {
    Number,
    Name,
    Vulnerability,
    Contract,
    Result,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Selection {
    Unselected,
    NameSelect(Seat),
    // On an entry
    FieldSelect(usize, EntryField),
}

enum FieldStatus {
    NotSelected,
    Selected,
    Entering,
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

    pub fn is(&self, sel : Selection) -> FieldStatus {
        match (self.selection_, self.entry_.as_ref()) {
            (s, Some(_)) if s == sel => FieldStatus::Entering,
            (s, None)    if s == sel => FieldStatus::Selected,
            _ => FieldStatus::NotSelected,
        }
    }

    pub fn is_seat(&self, seat : Seat) -> FieldStatus {
        self.is(Selection::NameSelect(seat))
    }

    pub fn is_north(&self) -> FieldStatus {
        self.is_seat(Seat::North)
    }

    pub fn is_east(&self) -> FieldStatus {
        self.is_seat(Seat::East)
    }

    pub fn is_south(&self) -> FieldStatus {
        self.is_seat(Seat::South)
    }

    pub fn is_west(&self) -> FieldStatus {
        self.is_seat(Seat::West)
    }

    pub fn is_field(&self, x : usize, f : EntryField) -> FieldStatus {
        self.is(Selection::FieldSelect(x, f))
    }
}

pub struct Interface {
    // Contract-related
    table_: Table,
    dealer_ : Seat,
    entries_ : Vec<Entry>,
    input_state_ : InputState,
    partial_entry_ : Option<Entry>,

    root_window_ : Window,
    entry_window_ : Window,

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

fn shrink(win : &Window) -> Window {
    win.derwin(win.get_max_y() - 2, win.get_max_x() - 2, 1, 1).unwrap()
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

fn set_field_cursor(window : &Window, is_selected : FieldStatus) {
    window.attroff(pancurses::A_UNDERLINE);
    window.color_set(CURSOR_NORMAL);
    match is_selected {
        FieldStatus::NotSelected => 0,
        FieldStatus::Selected => window.attron(pancurses::A_UNDERLINE),
        FieldStatus::Entering => window.color_set(CURSOR_ENTERING),
    };
}

fn record(entry : &mut Entry, f : EntryField, input : String) {
    match f {
        EntryField::Name          => entry.set_name(input),
        EntryField::Result        =>
            { input.parse().map(|r| entry.record(r)); },
        EntryField::Contract      =>
            { parse_input(&input).map(|c| entry.set_contract(c)); },
        EntryField::Vulnerability =>
            entry.set_vulnerable(input.to_lowercase() == "v"),
        _ => (),
    }
}

impl Interface {
    pub fn new(root_window : Window) -> Interface {
        let tablewin = root_window.derwin(9, 15, 1, 0).unwrap();
        let valueswin = root_window.derwin(10, 15, 0, 14).unwrap();
        let entrywin = root_window.derwin(1, 14, 0, 0).unwrap();
        let entrieswin = root_window.derwin(40, 40, 10, 0).unwrap();
        Interface {
            table_ : Table::new(),
            dealer_ : Seat::North,
            entries_ : Vec::new(),
            input_state_ : InputState::new(),
            partial_entry_ : None,
            root_window_ : root_window,
            entry_window_ : entrywin,
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
        if self.input_state_.entry_.is_some() {
            pancurses::curs_set(1);
        } else {
            pancurses::curs_set(0);
        }
        self.draw_entries();
        self.draw_input();
        self.refresh();
    }

    pub fn get_input(&mut self) -> bool {
        let c = match self.root_window_.getch() {
            Some(Input::Character(cc)) => cc,
            Some(Input::KeyBackspace) => '\x7f',
            None => { return false; },
            _    => { return true; },
        };
        match c {
            '\t' => self.cycle_input(),
            '\n' => self.enter_input(),
            _    => self.input_char(c),
        };
        self.update_entries();
        return true;
    }

    fn navigate(&mut self, c : char) {
        match c {
            'h' => self.move_left(),
            'j' => self.move_down(),
            'k' => self.move_up(),
            'l' => self.move_right(),
            ' ' => self.activate(),
            _   => (),
        }
    }

    fn cycle_input(&mut self) {
        let new_selection = match self.input_state_.selection_ {
            Selection::Unselected        => Selection::NameSelect(Seat::North),
            Selection::NameSelect(_)     =>
                Selection::FieldSelect(0, EntryField::Contract),
            Selection::FieldSelect(_, _) => Selection::NameSelect(Seat::North),
        };

        self.input_state_.selection_ = new_selection;
        self.input_state_.entry_ = None;
    }

    fn update_entries(&mut self) {
        let entryix = match self.input_state_.selection_ {
            Selection::FieldSelect(ix, _) => ix,
            _ => { self.partial_entry_ = None; return; },
        };

        if entryix > self.entries_.len() {
            panic!("Something has gone wrong.");
        }

        if entryix == self.entries_.len() {
            if self.partial_entry_.is_none() {
                self.partial_entry_ =
                    Some(Entry::new(&self.table_, Seat::North, 1));
            }
        }
    }

    fn move_left(&mut self) {
        self.input_state_.selection_ = match self.input_state_.selection_ {
            Selection::NameSelect(_) => Selection::NameSelect(Seat::West),
            Selection::FieldSelect(x, EntryField::Name) =>
                Selection::FieldSelect(x, EntryField::Name),
            Selection::FieldSelect(x, EntryField::Vulnerability) =>
                Selection::FieldSelect(x, EntryField::Name),
            Selection::FieldSelect(x, EntryField::Contract) =>
                Selection::FieldSelect(x, EntryField::Vulnerability),
            Selection::FieldSelect(x, EntryField::Result) =>
                Selection::FieldSelect(x, EntryField::Contract),
            _ => self.input_state_.selection_
        }
    }

    fn move_right(&mut self) {
        self.input_state_.selection_ = match self.input_state_.selection_ {
            Selection::NameSelect(_) => Selection::NameSelect(Seat::East),
            Selection::FieldSelect(x, EntryField::Name) =>
                Selection::FieldSelect(x, EntryField::Vulnerability),
            Selection::FieldSelect(x, EntryField::Vulnerability) =>
                Selection::FieldSelect(x, EntryField::Contract),
            Selection::FieldSelect(x, EntryField::Contract) =>
                Selection::FieldSelect(x, EntryField::Result),
            Selection::FieldSelect(x, EntryField::Result) =>
                Selection::FieldSelect(x, EntryField::Result),
            _ => self.input_state_.selection_,
        }
    }

    fn move_up(&mut self) {
        self.input_state_.selection_ = match self.input_state_.selection_ {
            Selection::NameSelect(_)     => Selection::NameSelect(Seat::North),
            Selection::FieldSelect(0, y) => Selection::FieldSelect(0, y),
            Selection::FieldSelect(x, y) => Selection::FieldSelect(x - 1, y),
            _ => self.input_state_.selection_,
        }
    }

    fn move_down(&mut self) {
        self.input_state_.selection_ = match self.input_state_.selection_ {
            Selection::NameSelect(_)     => Selection::NameSelect(Seat::South),
            Selection::FieldSelect(x, y) => Selection::FieldSelect(
                if x == self.entries_.len() { x } else { x + 1}, y),
            _ => self.input_state_.selection_,
        }
    }

    fn activate(&mut self) {
        self.input_state_.selection_ = match self.input_state_.selection_ {
            Selection::NameSelect(s) =>
                Selection::FieldSelect(self.add(s), EntryField::Contract),
            _ => self.input_state_.selection_
        };
    }

    fn add(&mut self, s : Seat) -> usize {
        let new_board = self.entries_.len() as u32 + 1;
        self.entries_.push(Entry::new(&self.table_, s, new_board));
        self.entries_.len() - 1
    }

    fn enter_input(&mut self) {
        match self.input_state_.selection_ {
            Selection::Unselected => { return; },
            _ => (),
        };
        match self.input_state_.entry_ {
            None => self.input_state_.entry_ = Some("".to_string()),
            Some(_) => {
                let input = mem::replace(&mut self.input_state_.entry_, None);
                self.process_input(input.unwrap());
            },
        };
    }

    fn process_input(&mut self, input : String) {
        match self.input_state_.selection_ {
            Selection::Unselected => panic!("How did you do that"),
            Selection::NameSelect(s) => self.table_.set_player(s, &input),
            Selection::FieldSelect(x, y) =>
            { self.entry_mut(x).map(|e| record(e, y, input)); },
        }
    }

    fn input_char(&mut self, c : char) {
        if !self.input_state_.entry_.is_some() {
            self.navigate(c);
            return;
        }
        match c {
            '\x7f' => {self.input_state_.entry_.as_mut().unwrap().pop();},
            _      => self.input_state_.entry_.as_mut().unwrap().push(c),
        }
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

    fn input(&self) -> &str {
        // Awkward.
        self.input_state_.entry_.as_ref().map(String::as_str).unwrap_or("")
    }

    fn entry_mut(&mut self, ix : usize) -> Option<&mut Entry> {
        if ix == self.entries_.len() {
            return self.partial_entry_.as_mut();
        }
        self.entries_.get_mut(ix)
    }

    fn draw_borders(&self) {
        border(&self.values_border_window_);
        border(&self.table_border_window_);
        border(&self.entries_border_window_);
    }

    fn draw_table(&self) {
        let table_win = &self.table_window_;
        let midpoint = table_win.get_max_y() / 2;
        let ew_length = self.east().len() + self.west().len();
        let midpoint_offset =
            if ew_length as i32 >= table_win.get_max_x() { 1 } else { 0 };

        &table_win.clear();
        set_field_cursor(&table_win, FieldStatus::NotSelected);
        table_win.mvaddch(midpoint, 0, '^');
        table_win.mvaddch(midpoint, table_win.get_max_x() - 1, 'v');

        set_field_cursor(&table_win, self.input_state_.is_north());
        center_pad(&table_win, self.north(), 0);

        set_field_cursor(&table_win, self.input_state_.is_south());
        center_pad(&table_win, self.south(), table_win.get_max_y() - 1);

        set_field_cursor(&table_win, self.input_state_.is_east());
        right_justify(&table_win, self.east(), midpoint + midpoint_offset);

        set_field_cursor(&table_win, self.input_state_.is_west());
        left_justify(&table_win, self.west(), midpoint - midpoint_offset);
    }

    fn draw_entries(&self) {
        self.entries_window_.clear();
        self.entries_window_.mv(0, 0);
        set_field_cursor(&self.entry_window_, FieldStatus::NotSelected);
        let ref entries : &Vec<_> = &self.entries_;
        for (ix, entry) in entries.iter().enumerate() {
            self.draw_entry(entry, ix);
        };
        if let Some(ref e) = self.partial_entry_ {
            self.entries_window_.addstr("Partial entry detected\n");
            self.draw_entry(e, self.entries_.len());
        };
    }

    fn draw_entry(&self, entry : &Entry, ix : usize) {
        let entry_win = &self.entries_window_;
        let input = &self.input_state_;
        set_field_cursor(entry_win, input.is_field(ix, EntryField::Name));
        entry_win.addstr(&format!("{:<10}", entry.name()));
        set_field_cursor(entry_win, FieldStatus::NotSelected);
        entry_win.addch('|');
        set_field_cursor(entry_win,
                         input.is_field(ix, EntryField::Vulnerability));
        entry_win.addch(if entry.is_vulnerable() { 'V' } else { ' ' });
        set_field_cursor(entry_win, FieldStatus::NotSelected);
        entry_win.addch('|');
        set_field_cursor(entry_win, input.is_field(ix, EntryField::Contract));
        match entry.contract() {
            Some(c) => entry_win.addstr(&format!("{:6}", c.to_string())),
            None    => entry_win.addstr("      "),
        };
        set_field_cursor(entry_win, FieldStatus::NotSelected);
        entry_win.addch('|');
        set_field_cursor(entry_win, input.is_field(ix, EntryField::Result));
        match entry.result() {
            Some(c) => entry_win.addstr(&format!("{:+3}", c)),
            None    => entry_win.addstr("   "),
        };
        set_field_cursor(entry_win, FieldStatus::NotSelected);
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

    fn draw_input(&self) {
        self.entry_window_.clear();
        self.entry_window_.mv(0, 0);
        let mut x = 0;
        for c in self.input().chars() {
            self.entry_window_.addch(c);
            x = x + 1;
        }
        for i in x..self.entry_window_.get_max_x() {
            self.entry_window_.addch(' ');
        }
        self.root_window_.mv(0, x);
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
        self.entry_window_.refresh();
        self.root_window_.refresh();
    }
}
