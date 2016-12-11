#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Seat {
    North, East, South, West
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContractSuit {
    Clubs, Diamonds, Hearts, Spades, NoTrump
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContractNumber {
    One, Two, Three, Four, Five, Six, SEVEN
}

impl ContractNumber {
    pub fn into_i32(self) -> i32 {
        match self {
            ContractNumber::One   => 1,
            ContractNumber::Two   => 2,
            ContractNumber::Three => 3,
            ContractNumber::Four  => 4,
            ContractNumber::Five  => 5,
            ContractNumber::Six   => 6,
            ContractNumber::SEVEN => 7,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContractDoubled {
    Undoubled, Doubled, Redoubled
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Contract {
    pub suit : ContractSuit,
    pub number : ContractNumber,
    pub doubled : ContractDoubled,
}

impl Contract {
    pub fn new(suit : ContractSuit, number : ContractNumber,
               doubled : ContractDoubled) -> Contract {
        Contract {
            suit : suit,
            number : number,
            doubled : doubled,
        }
    }
}

fn penalty_points(double : ContractDoubled,
                  is_vulnerable: bool) -> (i32, i32, i32) {
    match (double, is_vulnerable) {
        (ContractDoubled::Undoubled, false) => (50,  50,  50),
        (ContractDoubled::Undoubled, true ) => (100, 100, 100),
        (ContractDoubled::Doubled,   false) => (100, 200, 300),
        (ContractDoubled::Doubled,   true ) => (200, 300, 300),
        (ContractDoubled::Redoubled, false) => (200, 400, 600),
        (ContractDoubled::Redoubled, true ) => (400, 600, 600),
    }
}

pub fn score_game(contract : Contract,
                  margin : i32, is_vulnerable : bool) -> i32 {
    if margin < 0 {
        let (first, second, fourth) =
            penalty_points(contract.doubled, is_vulnerable);
        let undertricks = -margin;
        let value = match undertricks {
            1 => first,
            2 | 3 => first + (undertricks - 1) * second,
            _ => first + 2 * second + (undertricks - 3) * fourth,
        };
        -value
    } else {
        // First, compute trick values.
        let trick_value = match contract.suit {
            ContractSuit::Clubs | ContractSuit::Diamonds => 20,
            _                                            => 30,
        };
        let first_trick = match contract.suit {
            ContractSuit::NoTrump => 40,
            _                     => trick_value,
        };
        let doubling_bonus = match contract.doubled {
            ContractDoubled::Undoubled => 1, 
            ContractDoubled::Doubled   => 2, 
            ContractDoubled::Redoubled => 4, 
        };
        // The contract value determines the game or non-game bonus.
        let contract_value = doubling_bonus *
            (first_trick + (contract.number.into_i32() - 1) * trick_value);

        let normal_bonus = 50;
        let game_bonus = if is_vulnerable { 500 } else { 300 };

        let making_bonus_awarded =
            if contract_value >= 100 { game_bonus} else { normal_bonus };

        // These can be computed any time, as they don't impact game scoring.
        let small_slam_bonus = if is_vulnerable { 750 } else { 500 };
        let grand_slam_bonus = if is_vulnerable { 1500 } else { 1000 };
        let slam_bonus_awarded = match contract.number {
            ContractNumber::Six   => small_slam_bonus,
            ContractNumber::SEVEN => grand_slam_bonus,
            _                     => 0,
        };

        let insult = match contract.doubled {
            ContractDoubled::Undoubled => 0,
            ContractDoubled::Doubled   => 50,
            ContractDoubled::Redoubled => 100,
        };

        // This is the base value you got for making the contract.
        let value_for_making =
            contract_value + insult + making_bonus_awarded + slam_bonus_awarded;

        let overtrick_value = match (contract.doubled, is_vulnerable) {
            (ContractDoubled::Undoubled, _)     => trick_value,
            (ContractDoubled::Doubled,   true)  => 200,
            (ContractDoubled::Doubled,   false) => 100,
            (ContractDoubled::Redoubled, true)  => 400,
            (ContractDoubled::Redoubled, false) => 200,
        };

        value_for_making + overtrick_value * margin
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
}

impl Entry {
    pub fn new(table : Table, declarer : Seat, board_num : u32) -> Entry {
        Entry {
            name_ : String::from(table.get_player(declarer)),
            declarer_ : declarer,
            contract_ : None,
            board_num_ : board_num,
            ns_vulnerable_ : (board_num & 1 == 1),
            ew_vulnerable_ : (board_num & 2 == 2),
            result_ : None,
            value_ : None,
        }
    }

    pub fn set_contract(&mut self, c : Contract) {
        self.contract_ = Some(c)
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
        let contract = match self.contract_ {
            None => return Err("set the contract first, doofus."),
            Some(c) => c,
        };
        self.result_ = Some(margin);
        self.value_  = Some(score_game(contract, margin, self.is_vulnerable()));
        Ok(())
    }
}

pub fn score_3s_v_p3() {
    println!("bluh {}", score_game(Contract::new(ContractSuit::Spades,
                                     ContractNumber::Three,
                                     ContractDoubled::Undoubled),
                        3,
                        true));
}
