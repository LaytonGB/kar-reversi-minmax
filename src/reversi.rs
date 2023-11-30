use std::{cmp::Ordering, collections::HashSet};

use crate::{
    board::Board, bot::Bot, bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty,
    bot_heuristic::BotHeuristic, constants::DIRECTIONS, history::History, player::Player,
};

#[cfg(feature = "terminal")]
use crate::utils;
#[cfg(feature = "terminal")]
use if_chain::if_chain;
#[cfg(feature = "terminal")]
use text_io::try_read;

#[derive(Clone, Debug)]
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
    pub fn new(bot_player: Option<(Player, BotDifficulty, BotAlgorithm, BotHeuristic)>) -> Self {
        Self {
            board: Board::new(8),
            bot_player: bot_player.map_or(None, |(p, difficulty, algorithm, heuristic)| {
                type D = BotDifficulty;
                Some((
                    p,
                    Bot::new(
                        algorithm,
                        match difficulty {
                            D::Easy => Some(1),
                            D::Medium => Some(4),
                            D::Hard => Some(8),
                            D::Insane => Some(12),
                        },
                        heuristic,
                    ),
                ))
            }),
            current_player: Player::Green,
            ..Default::default()
        }
    }

    #[cfg(feature = "terminal")]
    pub fn show_board(&self, with_metrics: bool) {
        utils::clear_terminal();
        if_chain!(if with_metrics;
            if let Some((_, bot)) = self.bot_player.as_ref();
            then {
                bot.show_metrics();
            }
        );
        println!(
            "{}: {} | {}: {}",
            Player::Green,
            self.board.pieces_for_player(Player::Green).count(),
            Player::Red,
            self.board.pieces_for_player(Player::Red).count()
        );
        println!("{}", self.board);
    }

    #[cfg(feature = "terminal")]
    pub fn start(&mut self) {
        use core::time;
        use std::thread;

        while Self::anyone_can_move(&self.board) {
            let current_player_is_bot = self
                .bot_player
                .as_ref()
                .is_some_and(|(p, _)| *p == self.current_player);
            self.show_board(!current_player_is_bot);

            self.update_valid_moves();
            if current_player_is_bot {
                let sleep_time = time::Duration::from_millis(1500);
                thread::sleep(sleep_time);
                let game = self.clone();
                let (_, bot) = self.bot_player.as_mut().unwrap();
                let coord = bot.get_move(game);
                self.place_piece_and_add_history(coord);
            } else if Self::can_move(&self.board, self.current_player) {
                let coord =
                    self.get_valid_coordinate_input(Some(|| println!("ERROR: Invalid input")));
                self.place_piece_and_add_history(coord);
            }

            self.switch_players();
        }

        self.show_board(true);
        self.show_winner(self.get_winner());
    }

    pub(crate) fn update_valid_moves(&mut self) {
        self.valid_moves =
            Self::get_valid_moves_for_player(&self.board, self.current_player).collect();
    }

    pub(crate) fn get_valid_moves_for_player(
        board: &Board,
        player: Player,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let n = board.size();
        let in_bounds = |(a, b): (usize, usize)| a < n && b < n;
        board
            .pieces_for_player(player)
            .fold(HashSet::new(), |mut s, start| {
                for (i, j) in DIRECTIONS {
                    let increment = |(a, b): (usize, usize)| (a.wrapping_add(i), b.wrapping_add(j));

                    let mut coord = increment(start);
                    if !in_bounds(coord) || board.get(coord).map_or(true, |p| p == player) {
                        continue;
                    }

                    while in_bounds(coord) {
                        let p = board.get(coord);
                        match p {
                            Some(p) if p == player => break,
                            Some(_) => coord = increment(coord),
                            None => {
                                s.insert(coord);
                                break;
                            }
                        }
                    }
                }
                s
            })
            .into_iter()
    }

    pub(crate) fn anyone_can_move(board: &Board) -> bool {
        Self::can_move(board, Player::Green) || Self::can_move(board, Player::Red)
    }

    pub(crate) fn can_move(board: &Board, player: Player) -> bool {
        Self::get_valid_moves_for_player(board, player).count() > 0
    }

    pub fn get_winner(&self) -> Option<Player> {
        let (black_pieces, red_pieces) = (
            self.board.pieces_for_player(Player::Green).count(),
            self.board.pieces_for_player(Player::Red).count(),
        );
        match black_pieces.cmp(&red_pieces) {
            Ordering::Less => Some(Player::Red),
            Ordering::Equal => None,
            Ordering::Greater => Some(Player::Green),
        }
    }

    #[cfg(feature = "terminal")]
    fn show_winner(&self, winner: Option<Player>) {
        match winner {
            Some(winner) => println!("!!! {} WINS !!!", winner),
            None => println!("!!! IT'S A DRAW !!!"),
        }
    }

    #[cfg(feature = "terminal")]
    fn get_valid_coordinate_input(&self, error_msg: Option<impl Fn()>) -> (usize, usize) {
        loop {
            let row: Result<usize, _> = try_read!();
            let col: Result<usize, _> = try_read!();

            // TODO clean
            if row
                .as_ref()
                .is_ok_and(|&r| r.wrapping_sub(1) < self.board.size())
                && col
                    .as_ref()
                    .is_ok_and(|&c| c.wrapping_sub(1) < self.board.size())
                && self.valid_moves.contains(&(
                    row.as_ref().unwrap().wrapping_sub(1),
                    col.as_ref().unwrap().wrapping_sub(1),
                ))
            {
                return (row.unwrap().wrapping_sub(1), col.unwrap().wrapping_sub(1));
            } else if let Some(ref _error_msg) = error_msg {
                #[cfg(feature = "terminal")]
                _error_msg()
            }
        }
    }

    pub(crate) fn place_piece_and_add_history(&mut self, coord: (usize, usize)) {
        let captured_pieces =
            Self::place_piece_on_board(&mut self.board, coord, self.current_player);
        self.history
            .push(self.current_player, coord, captured_pieces);
    }

    pub(crate) fn place_piece_on_board(
        board: &mut Board,
        coord: (usize, usize),
        player: Player,
    ) -> Vec<(usize, usize)> {
        board.set(coord, Some(player));

        let captured_pieces = Self::get_captures_for_position(board, coord).to_vec();
        for &c in captured_pieces.iter() {
            board.switch_piece(c);
        }

        captured_pieces
    }

    pub(crate) fn switch_players(&mut self) {
        self.current_player = self.current_player.other();
    }

    pub fn bot_player(&self) -> Option<&(Player, Bot)> {
        self.bot_player.as_ref()
    }

    pub fn bot_player_mut(&mut self) -> Option<&mut (Player, Bot)> {
        self.bot_player.as_mut()
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub(crate) fn valid_moves(&self) -> &[(usize, usize)] {
        &self.valid_moves
    }

    pub(crate) fn undo_turn(&mut self) {
        let (player, coord, captured_pieces) = self.history.pop().unwrap();
        self.board.set(coord, None);
        for coord in captured_pieces {
            self.board.set(coord, Some(player.other()));
        }
        self.current_player = player;
    }

    pub(crate) fn board(&self) -> &Board {
        &self.board
    }

    fn get_captures_for_position(board: &Board, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let player = board.get(coord);
        if player.is_none() {
            return vec![];
        }

        let player = player.unwrap();
        let n = board.size();
        let in_bounds = |(a, b): (usize, usize)| a < n && b < n;
        let mut res = Vec::new();

        for (i, j) in DIRECTIONS {
            let increment = |(a, b): (usize, usize)| (a.wrapping_add(i), b.wrapping_add(j));
            let mut coord = increment(coord);
            let mut v = Vec::new();
            while in_bounds(coord) {
                let p = board.get(coord);
                match p {
                    Some(p) if p == player => {
                        res.append(&mut v);
                        break;
                    }
                    Some(_) => v.push(coord),
                    _ => {
                        v.clear();
                        break;
                    }
                }
                coord = increment(coord);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_update_valid_moves() {
        let mut game = Reversi::new(None);
        game.update_valid_moves();

        assert_eq!(
            HashSet::<(usize, usize)>::from_iter(game.valid_moves.into_iter()),
            HashSet::from_iter([(2, 4), (4, 2), (3, 5), (5, 3)])
        );
    }

    #[test]
    fn test_get_captures_for_position() {
        let game = Reversi::new(None);

        assert_eq!(
            Reversi::get_captures_for_position(game.board(), (4, 2)),
            vec![(4, 3)]
        );
    }
}
