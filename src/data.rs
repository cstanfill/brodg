use contract::{Contract, Seat};
use score::Score;

pub struct Table {
    players_ : [String; 4],
}

impl Table {
    pub fn new() -> Table{
        Table {
            players_ : [
                String::from("North"),
                String::from("East"),
                String::from("South"),
                String::from("West"),
            ],
        }
    }

    pub fn get_player(&self, s : Seat) -> &str {
        match s {
            Seat::North => &self.players_[0],
            Seat::East  => &self.players_[1],
            Seat::South => &self.players_[2],
            Seat::West  => &self.players_[3],
        }
    }

    pub fn set_player(&mut self, s : Seat, name : &str) {
        match s {
            Seat::North => self.players_[0] = String::from(name),
            Seat::East  => self.players_[1] = String::from(name),
            Seat::South => self.players_[2] = String::from(name),
            Seat::West  => self.players_[3] = String::from(name),
        }
    }
}

pub struct Entry {
    declarer_ : Seat,
    name_ : String,
    contract_ : Option<Contract>,
    board_num_ : u32,
    ns_vulnerable_ : bool,
    ew_vulnerable_ : bool,
    result_ : Option<i32>,
    value_ : Option<i32>,
}

impl Entry {
    pub fn new(table : &Table, declarer : Seat, board_num : u32) -> Entry {
        Entry {
            name_ : String::from(table.get_player(declarer)),
            declarer_ : declarer,
            contract_ : None,
            board_num_ : board_num,
            ns_vulnerable_ : ((board_num - 1) & 1 == 1),
            ew_vulnerable_ : ((board_num - 1) & 2 == 2),
            result_ : None,
            value_ : None,
        }
    }

    pub fn set_contract(&mut self, c : Contract) {
        self.contract_ = Some(c);
        self.recompute();
    }

    pub fn has_contract(&self) -> bool {
        self.contract_.is_some()
    }

    pub fn contract(&self) -> Option<Contract> {
        self.contract_
    }

    pub fn board_num(&self) -> u32 {
        self.board_num_
    }

    pub fn is_vulnerable(&self) -> bool {
        match self.declarer_ {
            Seat::North | Seat::South => self.ns_vulnerable_,
            Seat::East  | Seat::West  => self.ew_vulnerable_,
        }
    }

    pub fn set_vulnerable(&mut self, status : bool) {
        match self.declarer_ {
            Seat::North | Seat::South => self.ns_vulnerable_ = status,
            Seat::East  | Seat::West  => self.ew_vulnerable_ = status,
        }
        self.recompute();
    }

    pub fn record(&mut self, margin : i32) {
        self.result_ = Some(margin);
        self.recompute();
    }

    fn recompute(&mut self) {
        self.value_ = match (self.contract_.as_ref(), self.result_) {
            (Some(c), Some(r)) =>
                Some(Score::from_contract(c,
                                          self.is_vulnerable()).score_result(r)),
            _ => None,
        };
    }

    pub fn name(&self) -> &str {
        &self.name_
    }

    pub fn set_name(&mut self, name : String) {
        self.name_ = name
    }

    pub fn result(&self) -> Option<i32> {
        self.result_
    }

    pub fn value(&self) -> Option<i32> {
        self.value_
    }

    pub fn declarer(&self) -> Option<Seat> {
        Some(self.declarer_)
    }
}
