use crate::round::{Round, Rps};

#[derive(Debug)]
pub struct Game {
    pub my_plays: Vec<Rps>,
    pub oponent_plays: Vec<Rps>,
}

impl Game {
    pub fn new(my_plays: Vec<Rps>, oponent_plays: Vec<Rps>) -> Game {
        Game {
            my_plays,
            oponent_plays,
        }
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
