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
    score_ : Option<Score>,
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
            score_ : None,
            board_num_ : board_num,
            ns_vulnerable_ : (board_num & 1 == 1),
            ew_vulnerable_ : (board_num & 2 == 2),
            result_ : None,
            value_ : None,
        }
    }

    pub fn set_contract(&mut self, c : Contract) {
        self.contract_ = Some(c);
        self.score_ = Some(Score::from_contract(c, self.is_vulnerable()));
    }

    pub fn has_contract(&self) -> bool {
        self.contract_.is_some()
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

    pub fn record(&mut self, margin : i32) -> Result<(), &str> {
        let score = match self.score_ {
            None => return Err("set the contract first, doofus."),
            Some(ref s) => s,
        };
        self.result_ = Some(margin);
        self.value_  = Some(score.score_result(margin));
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.name_
    }
}
