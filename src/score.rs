use contract::{Contract, ContractNumber, ContractSuit, ContractDoubled};

pub struct Score {
    pub first_trick_value : i32,
    pub trick_value : i32,
    pub making_bonus : i32,
    pub insult : i32,
    pub slam_bonus : i32,
    pub contract_value : i32,
    pub overtricks : i32,
    pub setting : i32,
    pub next_undertricks : i32,
    pub rest_undertricks : i32,
}

impl Score {
    fn score_undertricks(&self, num : i32) -> i32{
        let extra = match num {
            1 => 0,
            2 | 3 => (num - 1) * self.next_undertricks,
            _ => 2 * self.next_undertricks + (num - 3) * self.rest_undertricks,
        };
        self.setting + extra
    }

    pub fn score_result(&self, margin : i32) -> i32 {
        if margin < 0 {
            -self.score_undertricks(-margin)
        } else {
            self.contract_value + self.overtricks * margin
        }
    }

    pub fn from_contract(contract : Contract, is_vulnerable : bool) -> Score {
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

        let making_bonus =
            if contract_value >= 100 { game_bonus} else { normal_bonus };

        // These can be computed any time, as they don't impact game scoring.
        let small_slam_bonus = if is_vulnerable { 750 } else { 500 };
        let grand_slam_bonus = if is_vulnerable { 1500 } else { 1000 };
        let slam_bonus = match contract.number {
            ContractNumber::Six   => small_slam_bonus,
            ContractNumber::SEVEN => grand_slam_bonus,
            _                     => 0,
        };

        let insult = match contract.doubled {
            ContractDoubled::Undoubled => 0,
            ContractDoubled::Doubled   => 50,
            ContractDoubled::Redoubled => 100,
        };

        let making_value = contract_value + insult + making_bonus + slam_bonus;

        let overtrick_value = match (contract.doubled, is_vulnerable) {
            (ContractDoubled::Undoubled, _)     => trick_value,
            (ContractDoubled::Doubled,   true)  => 200,
            (ContractDoubled::Doubled,   false) => 100,
            (ContractDoubled::Redoubled, true)  => 400,
            (ContractDoubled::Redoubled, false) => 200,
        };

        // Undertrick valuations: First, second and third, fourth and beyond.
        let penalties = match (contract.doubled, is_vulnerable) {
            (ContractDoubled::Undoubled, false) => (50,  50,  50),
            (ContractDoubled::Undoubled, true ) => (100, 100, 100),
            (ContractDoubled::Doubled,   false) => (100, 200, 300),
            (ContractDoubled::Doubled,   true ) => (200, 300, 300),
            (ContractDoubled::Redoubled, false) => (200, 400, 600),
            (ContractDoubled::Redoubled, true ) => (400, 600, 600),
        };

        Score {
            first_trick_value: first_trick * doubling_bonus,
            trick_value : trick_value * doubling_bonus,
            making_bonus : making_bonus,
            insult : insult,
            slam_bonus : slam_bonus,
            contract_value : making_value,
            overtricks : overtrick_value,
            setting : penalties.0,
            next_undertricks : penalties.1,
            rest_undertricks : penalties.2,
        }
    }
}

pub fn score_game(contract : Contract, margin : i32, is_vulnerable : bool)
    -> i32 {
    Score::from_contract(contract, is_vulnerable).score_result(margin)
}

