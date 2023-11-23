use core::time;
use std::{cmp::Ordering, collections::HashSet, thread};

use crate::{
    board::Board, bot::Bot, bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty,
    constants::DIRECTIONS, history::History, player::Player,
};

#[cfg(feature = "terminal")]
use crate::utils;
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
    pub fn new(bot_player: Option<(Player, BotAlgorithm, BotDifficulty)>) -> Self {
        Self {
            board: Board::new(8),
            bot_player: bot_player.map_or(None, |(p, al, dif)| {
                Some((
                    p,
                    Bot::new(
                        al,
                        match dif {
                            BotDifficulty::Easy => Some(1),
                            BotDifficulty::Medium => Some(3),
                            BotDifficulty::Hard => Some(8),
                            BotDifficulty::Insane => None,
                        },
                    ),
                ))
            }),
            current_player: Player::Green,
            ..Default::default()
        }
    }

    #[cfg(feature = "terminal")]
    pub fn show_board(&self) {
        utils::clear_terminal();
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
        while self.anyone_can_move() {
            self.show_board();

            self.update_valid_moves();
            if self
                .bot_player
                .as_ref()
                .is_some_and(|(p, _)| *p == self.current_player)
            {
                let sleep_time = time::Duration::from_millis(1500);
                thread::sleep(sleep_time);
                let game = self.clone();
                let (_, bot) = self.bot_player.as_mut().unwrap();
                let coord = bot.get_move(game);
                self.place_piece(coord);
            } else if self.can_move(self.current_player) {
                let coord =
                    self.get_valid_coordinate_input(Some(|| println!("ERROR: Invalid input")));
                self.place_piece(coord);
            }

            self.switch_players();
        }

        self.show_board();
        self.show_winner(self.get_winner());
    }

    pub(crate) fn update_valid_moves(&mut self) {
        self.valid_moves = self
            .get_valid_moves_for_player(self.current_player)
            .collect();
    }

    fn get_valid_moves_for_player(
        &self,
        player: Player,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let n = self.board.size();
        let in_bounds = |(a, b): (usize, usize)| a < n && b < n;
        self.board
            .pieces_for_player(player)
            .fold(HashSet::new(), |mut s, start| {
                for (i, j) in DIRECTIONS {
                    let increment = |(a, b): (usize, usize)| (a.wrapping_add(i), b.wrapping_add(j));

                    let mut coord = increment(start);
                    if !in_bounds(coord) || self.board.get(coord).map_or(true, |p| p == player) {
                        continue;
                    }

                    while in_bounds(coord) {
                        let p = self.board.get(coord);
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

    pub(crate) fn anyone_can_move(&self) -> bool {
        self.can_move(Player::Green) || self.can_move(Player::Red)
    }

    pub(crate) fn can_move(&self, player: Player) -> bool {
        self.get_valid_moves_for_player(player).count() > 0
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

    pub(crate) fn place_piece(&mut self, coord: (usize, usize)) {
        self.board.set(coord, Some(self.current_player));

        let captured_pieces = self.get_captures_for_position(coord).to_vec();
        for &c in captured_pieces.iter() {
            self.board.switch_piece(c);
        }

        self.history
            .push(self.current_player, coord, captured_pieces);
    }

    pub(crate) fn switch_players(&mut self) {
        self.current_player = self.current_player.other();
    }

    pub fn bot_player(&self) -> Option<&(Player, Bot)> {
        self.bot_player.as_ref()
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

    fn get_captures_for_position(&self, coord: (usize, usize)) -> Vec<(usize, usize)> {
        let player = self.current_player;
        let n = self.board.size();
        let in_bounds = |(a, b): (usize, usize)| a < n && b < n;
        let mut res = Vec::new();

        for (i, j) in DIRECTIONS {
            let increment = |(a, b): (usize, usize)| (a.wrapping_add(i), b.wrapping_add(j));
            let mut coord = increment(coord);
            let mut v = Vec::new();
            while in_bounds(coord) {
                let p = self.board.get(coord);
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

        assert_eq!(game.get_captures_for_position((4, 2)), vec![(4, 3)]);
    }
}
