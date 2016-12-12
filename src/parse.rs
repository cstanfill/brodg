use std::str::FromStr;

use contract::{Contract, ContractNumber, ContractSuit, ContractDoubled};

// The idea here is to let the frontend know where to start marking the
// contract name as invalid. `Incomplete` does not count as invalid if you
// still haven't hit enter.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ContractParseError {
    Incomplete,
    InvalidNumber(char),
    InvalidSuit(String),
    InvalidTrailing(usize, char),
}

impl FromStr for Contract {
    type Err = ContractParseError;
    fn from_str(name : &str) -> Result<Contract, ContractParseError> {
        // TODO: Reform this? Maybe?
        let mut chars = name.chars();
        let mut index = 0;
        let value = match chars.next() {
            Some('1') => ContractNumber::One,
            Some('2') => ContractNumber::Two,
            Some('3') => ContractNumber::Three,
            Some('4') => ContractNumber::Four,
            Some('5') => ContractNumber::Five,
            Some('6') => ContractNumber::Six,
            Some('7') => ContractNumber::SEVEN,
            Some(c)   => return Err(ContractParseError::InvalidNumber(c)),
            None      => return Err(ContractParseError::Incomplete),
        };
        index += 1;

        let suit = match chars.next() {
            Some('N') => {
                index += 1;
                match chars.next() {
                    Some('T') => ContractSuit::NoTrump,
                    Some(c)   => return Err(ContractParseError::InvalidSuit(
                            vec!['N', c].into_iter().collect())),
                    None   => return Err(ContractParseError::Incomplete),
                }
            },
            Some('C') => ContractSuit::Clubs,
            Some('D') => ContractSuit::Diamonds,
            Some('H') => ContractSuit::Hearts,
            Some('S') => ContractSuit::Spades,
            Some(c)   =>
                return Err(ContractParseError::InvalidSuit(c.to_string())),
            None      => return Err(ContractParseError::Incomplete),
        };
        index += 1;

        let doubling =
            match chars.next() {
                Some('X') => {
                    index += 1;
                    match chars.next() {
                        Some('X') => {
                            index += 1;
                            ContractDoubled::Redoubled
                        },
                        Some(c)   => return Err(
                            ContractParseError::InvalidTrailing(index, c)),
                        None      => ContractDoubled::Doubled,
                    }
                },
                Some(c) => return Err(
                    ContractParseError::InvalidTrailing(index, c)),
                None => ContractDoubled::Undoubled,
            };
        if let Some(c) = chars.next() {
            return Err(ContractParseError::InvalidTrailing(index, c));
        }
        Ok(Contract::new(suit, value, doubling))
    }
}

// Only use directly in testing
fn parse_contract(name : &str) -> Result<Contract, ContractParseError> {
    name.parse()
}

pub fn get_error_cursor(e : Option<ContractParseError>) -> Option<usize> {
    match e {
        None | Some(ContractParseError::Incomplete)     => None,
        Some(ContractParseError::InvalidNumber(_))      => Some(0),
        Some(ContractParseError::InvalidSuit(_))        => Some(1),
        Some(ContractParseError::InvalidTrailing(i, _)) => Some(i),
    }
}

// Actual users should use this
pub fn parse_input(input : &str) -> Result<Contract, ContractParseError> {
    input.to_uppercase().parse()
}

#[cfg(test)]
fn check_contract(name : &str) {
    assert!(parse_contract(name).unwrap().to_string() == name);
}

#[cfg(test)]
fn check_err(name : &str, err : ContractParseError) {
    assert!(parse_contract(name).unwrap_err() == err);
}

#[test]
fn test_2nt() {
    check_contract("2NT");
}

#[test]
fn test_7cxx() {
    check_contract("7CXX");
}

#[test]
fn test_9c() {
    check_err("9C", ContractParseError::InvalidNumber('9'));
}

#[test]
fn test_4x() {
    check_err("4X",
            ContractParseError::InvalidSuit(String::from_str("X").unwrap()));
}

#[test]
fn test_4n() {
    check_err("4N", ContractParseError::Incomplete);
}

#[test]
fn test_4nty() {
    check_err("4NTY", ContractParseError::InvalidTrailing(3, 'Y'));
}

#[test]
fn test_4ntxxx() {
    check_err("4NTXXX", ContractParseError::InvalidTrailing(5, 'X'));
}
