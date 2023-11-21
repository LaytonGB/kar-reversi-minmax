use crate::{
    board::Board, bot::Bot, bot_algorithm::BotAlgorithm, history::History, player::Player,
};

#[derive(Debug)]
pub struct Reversi {
    board: Board,
    bot_player: Option<(Player, Bot)>,
    current_player: Player,
    history: History,
}

impl Default for Reversi {
    fn default() -> Self {
        Self {
            board: Board::new(8),
            bot_player: Default::default(),
            current_player: Player::Red,
            history: Default::default(),
        }
    }
}

impl Reversi {
    pub fn new(bot_player: Option<(Player, BotAlgorithm)>) -> Self {
        Self {
            board: Board::new(8),
            bot_player: bot_player.map_or(None, |(p, al)| Some((p, Bot::new(al)))),
            current_player: Player::Red,
            ..Default::default()
        }
    }

    pub fn show_board(&self) {
        println!("{}", self.board);
    }
}
