use crate::{
    board::Board, bot::Bot, bot_algorithm::BotAlgorithm, history::History, player::Player,
};

#[derive(Debug)]
pub struct Reversi {
    board: Board,
    bot_player: Option<(Player, Bot)>,
    current_player: Player,
    history: History,
    valid_moves: Vec<(usize, usize)>,
}

impl Default for Reversi {
    fn default() -> Self {
        Self {
            board: Board::new(8),
            bot_player: Default::default(),
            current_player: Player::Red,
            history: Default::default(),
            valid_moves: Default::default(),
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

    pub fn start(&mut self) {
        while self.someone_can_move() {
            self.update_valid_moves();
            if self
                .bot_player
                .as_ref()
                .is_some_and(|(p, _)| *p == self.current_player)
            {
                let (_, bot) = self.bot_player.as_ref().unwrap();
                bot.take_turn(self);
            } else {
                let coord = self.get_valid_coordinate_input();
                self.place_piece(coord);
            }
        }

        let winner = self.get_winner();
        self.show_winner(winner);
    }

    fn update_valid_moves(&self) {
        todo!()
    }

    fn get_winner(&self) -> Player {
        todo!()
    }

    fn show_winner(&self, winner: Player) {
        todo!()
    }

    fn get_valid_coordinate_input(&self) -> (usize, usize) {
        todo!()
    }

    fn someone_can_move(&self) -> bool {
        todo!()
    }

    fn place_piece(&self, coord: (usize, usize)) {
        todo!()
    }
}
