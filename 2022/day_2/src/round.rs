#[derive(Debug)]
pub struct Round {
    my_hand: Rps,
    oponent_hand: Rps,
}

impl Round {
    pub fn new(my_hand: Rps, oponent_hand: Rps) -> Round {
        Round {
            my_hand,
            oponent_hand,
        }
    }

    pub fn score(&self) -> u32 {
        calculate_rps_score(&self.my_hand) + calculate_result_score(self.result())
    }

    fn result(&self) -> RoundResult {
        match self.my_hand {
            Rps::Rock => match self.oponent_hand {
                Rps::Rock => RoundResult::Draw,
                Rps::Paper => RoundResult::Defeat,
                Rps::Scissors => RoundResult::Victory,
            },
            Rps::Paper => match self.oponent_hand {
                Rps::Rock => RoundResult::Victory,
                Rps::Paper => RoundResult::Draw,
                Rps::Scissors => RoundResult::Defeat,
            },
            Rps::Scissors => match self.oponent_hand {
                Rps::Rock => RoundResult::Defeat,
                Rps::Paper => RoundResult::Victory,
                Rps::Scissors => RoundResult::Draw,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rps {
    Rock,
    Paper,
    Scissors,
}

pub fn rps_to_win(oponent_play: Rps) -> Rps {
    match oponent_play {
        Rps::Rock => Rps::Scissors,
        Rps::Paper => Rps::Rock,
        Rps::Scissors => Rps::Paper,
    }
}

pub fn rps_to_lose(oponent_play: Rps) -> Rps {
    match oponent_play {
        Rps::Rock => Rps::Paper,
        Rps::Paper => Rps::Scissors,
        Rps::Scissors => Rps::Rock,
    }
}

fn calculate_rps_score(rps: &Rps) -> u32 {
    match rps {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    }
}

#[derive(Debug)]
enum RoundResult {
    Victory,
    Defeat,
    Draw,
}

fn calculate_result_score(result: RoundResult) -> u32 {
    match result {
        RoundResult::Victory => 6,
        RoundResult::Defeat => 0,
        RoundResult::Draw => 3,
    }
}
