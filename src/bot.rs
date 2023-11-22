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
        todo!()
    }
}
