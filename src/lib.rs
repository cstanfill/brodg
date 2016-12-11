pub mod data;

use data::{Contract, ContractDoubled, ContractNumber, ContractSuit,
            score_game};

#[test]
fn score_3s_v_p3() {
    assert!(score_game(Contract::new(ContractSuit::Spades,
                                     ContractNumber::Three,
                                     ContractDoubled::Undoubled),
                        3,
                        true) == 230);
}

#[test]
fn score_2nt_d_p4() {
    assert!(score_game(Contract::new(ContractSuit::NoTrump,
                                     ContractNumber::Two,
                                     ContractDoubled::Doubled),
                        4,
                        false) == 890);
}

#[test]
fn score_d_m6() {
    assert!(score_game(Contract::new(ContractSuit::NoTrump,
                                     ContractNumber::Two,
                                     ContractDoubled::Doubled),
                        -6,
                        false) == -1400);
}

#[test]
fn score_6c_v_rd_p1() {
    assert!(score_game(Contract::new(ContractSuit::Clubs,
                                     ContractNumber::Six,
                                     ContractDoubled::Redoubled),
                        1,
                        true) == 2230);
}
