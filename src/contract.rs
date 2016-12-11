use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Seat {
    North, East, South, West
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContractSuit {
    Clubs, Diamonds, Hearts, Spades, NoTrump
}

impl fmt::Display for ContractSuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match *self {
                   ContractSuit::Clubs    => "C",
                   ContractSuit::Diamonds => "D",
                   ContractSuit::Hearts   => "H",
                   ContractSuit::Spades   => "S",
                   ContractSuit::NoTrump  => "NT",
               })
    }
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

impl fmt::Display for ContractNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.into_i32())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ContractDoubled {
    Undoubled, Doubled, Redoubled
}

impl fmt::Display for ContractDoubled {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match *self {
                   ContractDoubled::Undoubled   => "",
                   ContractDoubled::Doubled     => "X",
                   ContractDoubled::Redoubled   => "XX",
               })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Contract {
    pub suit : ContractSuit,
    pub number : ContractNumber,
    pub doubled : ContractDoubled,
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.number, self.suit, self.doubled)
    }
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

