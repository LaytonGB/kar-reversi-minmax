use std::{cmp::Ordering, collections::HashMap};

use text_io::try_read;

use crate::{
    board::Board, bot::Bot, bot_algorithm::BotAlgorithm, bot_difficulty::BotDifficulty,
    constants::DIRECTIONS, history::History, player::Player,
};

#[derive(Clone, Debug)]
pub struct Reversi {
    board: Board,
    bot_player: Option<(Player, Bot)>,
    current_player: Player,
    history: History,
    valid_moves: HashMap<(usize, usize), Vec<(usize, usize)>>,
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
                            BotDifficulty::Easy => Some(3),
                            BotDifficulty::Medium => Some(6),
                            BotDifficulty::Hard => Some(12),
                            BotDifficulty::Insane => None,
                        },
                    ),
                ))
            }),
            current_player: Player::Red,
            ..Default::default()
        }
    }

    pub fn show_board(&self) {
        println!("{}", self.board);
    }

    pub fn start(&mut self) {
        while self.anyone_can_move() {
            self.show_board();

            self.update_valid_moves();
            if self
                .bot_player
                .as_ref()
                .is_some_and(|(p, _)| *p == self.current_player)
            {
                let (_, bot) = self.bot_player.as_ref().unwrap();
                let coord = dbg!(bot.get_move(self));
                self.place_piece(coord);
            } else if self.can_move(self.current_player) {
                let coord =
                    self.get_valid_coordinate_input(Some(|| println!("ERROR: Invalid input")));
                self.place_piece(coord);
            }

            self.switch_players();
        }

        self.show_board();

        let winner = self.get_winner();
        self.show_winner(winner);
    }

    pub(crate) fn update_valid_moves(&mut self) {
        self.valid_moves = self.get_valid_moves_for_player(self.current_player);
    }

    fn get_valid_moves_for_player(
        &self,
        player: Player,
    ) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
        let n = self.board.size();
        self.board
            .pieces_for_player(player)
            .fold(HashMap::new(), |mut m, start| {
                for (i, j) in DIRECTIONS {
                    let mut coord = (start.0.wrapping_add(i), start.1.wrapping_add(j));
                    let mut cap_pieces = Vec::new();
                    let increment = |c: (usize, usize)| (c.0.wrapping_add(i), c.1.wrapping_add(j));
                    while coord.0 < n && coord.1 < n {
                        let p = self.board.get(coord);
                        match p {
                            Some(p) if p == player => {
                                break;
                            }
                            Some(_) => {
                                cap_pieces.push(coord);
                                coord = increment(coord);
                            }
                            None => {
                                if !cap_pieces.is_empty() {
                                    m.insert(coord, cap_pieces);
                                }
                                break;
                            }
                        }
                    }
                }
                m
            })
    }

    pub(crate) fn anyone_can_move(&self) -> bool {
        self.can_move(Player::Black) || self.can_move(Player::Red)
    }

    pub(crate) fn can_move(&self, player: Player) -> bool {
        self.get_valid_moves_for_player(player).len() > 0
    }

    fn get_winner(&self) -> Option<Player> {
        let (black_pieces, red_pieces) = (
            self.board.pieces_for_player(Player::Black).count(),
            self.board.pieces_for_player(Player::Red).count(),
        );
        match black_pieces.cmp(&red_pieces) {
            Ordering::Less => Some(Player::Red),
            Ordering::Equal => None,
            Ordering::Greater => Some(Player::Black),
        }
    }

    fn show_winner(&self, winner: Option<Player>) {
        match winner {
            Some(winner) => println!("!!! {} WINS !!!", winner),
            None => println!("!!! IT'S A DRAW !!!"),
        }
    }

    fn get_valid_coordinate_input(&self, error_msg: Option<impl Fn()>) -> (usize, usize) {
        loop {
            let row: Result<usize, _> = try_read!();
            let col: Result<usize, _> = try_read!();
            if row
                .as_ref()
                .is_ok_and(|&r| r.wrapping_sub(1) < self.board.size())
                && col
                    .as_ref()
                    .is_ok_and(|&c| c.wrapping_sub(1) < self.board.size())
                && self.valid_moves.contains_key(&(
                    row.as_ref().unwrap().wrapping_sub(1),
                    col.as_ref().unwrap().wrapping_sub(1),
                ))
            {
                return (row.unwrap().wrapping_sub(1), col.unwrap().wrapping_sub(1));
            } else if let Some(ref error_msg) = error_msg {
                error_msg()
            }
        }
    }

    pub(crate) fn place_piece(&mut self, coord: (usize, usize)) {
        self.board.set(coord, Some(self.current_player));

        let captured_pieces = self.valid_moves.get(&coord).unwrap();
        for &c in captured_pieces {
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

    pub(crate) fn valid_moves(&self) -> Vec<(usize, usize)> {
        self.valid_moves.iter().map(|(m, _)| *m).collect()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_valid_moves() {
        let mut game = Reversi::new(None);
        game.update_valid_moves();

        assert_eq!(
            game.valid_moves,
            HashMap::from([
                ((2, 3), vec![(3, 3)]),
                ((3, 2), vec![(3, 3)]),
                ((4, 5), vec![(4, 4)]),
                ((5, 4), vec![(4, 4)])
            ])
        );
    }
}
