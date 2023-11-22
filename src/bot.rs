use crate::{bot_algorithm::BotAlgorithm, reversi::Reversi};

#[derive(Debug)]
pub struct Bot {
    algorithm: BotAlgorithm,
}

impl Bot {
    pub fn new(algorithm: BotAlgorithm) -> Self {
        Self { algorithm }
    }

    pub fn take_turn(&self, game: &Reversi) {
        match self.algorithm {
            BotAlgorithm::MinMax => self.take_turn_minmax(game),
            BotAlgorithm::MinMaxAlphaBeta => todo!(),
            BotAlgorithm::NegaMax => todo!(),
        }
    }

    fn take_turn_minmax(&self, game: &Reversi) {
        todo!()
    }
}
