use crate::round::{Round, Rps};

pub struct Game {
    my_plays: Vec<Rps>,
    oponent_plays: Vec<Rps>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            my_plays: Vec::new(),
            oponent_plays: Vec::new(),
        }
    }

    pub fn add_my_play(&mut self, my_play: Rps) {
        self.my_plays.push(my_play);
    }

    pub fn add_oponent_play(&mut self, oponent_play: Rps) {
        self.oponent_plays.push(oponent_play);
    }

    pub fn get_score(&self) -> u32 {
        self.my_plays
            .iter()
            .zip(self.oponent_plays.iter())
            .fold(0, |big_score, hands| {
                let game_round = Round::new(*hands.0, *hands.1);
                big_score + game_round.score()
            })
    }
}
