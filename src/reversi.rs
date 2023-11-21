use crate::{board::Board, bot::Bot, bot_algorithm::BotAlgorithm, player::Player};

#[derive(Debug)]
pub struct Reversi {
    board: Board,
    bot_player: Option<(Player, Bot)>,
    current_player: Player,
}

impl Reversi {
    pub fn new(bot_player: Option<(Player, BotAlgorithm)>) -> Self {
        Self {
            board: Board::new(8),
            bot_player: bot_player.map_or(None, |(p, al)| Some((p, Bot::new(al)))),
            current_player: Player::Red,
        }
    }
}
