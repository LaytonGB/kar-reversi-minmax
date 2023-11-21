use crate::bot_algorithm::BotAlgorithm;

#[derive(Debug)]
pub struct Bot {
    algorithm: BotAlgorithm,
}

impl Bot {
    pub fn new(algorithm: BotAlgorithm) -> Self {
        Self { algorithm }
    }
}
