pub mod data;
pub mod parse;

#[cfg(test)]
use data::score_game;
#[cfg(test)]
use parse::parse_contract;

#[test]
fn score_3s_v_p3() {
    assert!(score_game(parse_contract("3S").unwrap(), 3, true) == 230);
}

#[test]
fn score_2nt_d_p4() {
    assert!(score_game(parse_contract("2NTX").unwrap(), 4, false) == 890);
}

#[test]
fn score_d_m6() {
    assert!(score_game(parse_contract("1SX").unwrap(), -6, false) == -1400);
}

#[test]
fn score_6c_v_rd_p1() {
    assert!(score_game(parse_contract("6CXX").unwrap(), 1, true) == 2230);
}
