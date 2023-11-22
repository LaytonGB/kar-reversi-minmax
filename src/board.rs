use std::fmt::Display;

use tabled::tables::IterTable;

use crate::player::Player;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    board: Vec<Vec<Option<Player>>>,
    size: usize,
}

impl Board {
    pub(crate) fn new(size: usize) -> Self {
        if size < 6 || size % 2 == 1 {
            panic!("size must be even")
        }

        let mut board = vec![vec![None; size]; size];
        let mid = size / 2;
        board[mid][mid] = Some(Player::Black);
        board[mid - 1][mid - 1] = Some(Player::Black);
        board[mid][mid - 1] = Some(Player::Red);
        board[mid - 1][mid] = Some(Player::Red);

        Self { board, size }
    }

    pub(crate) fn size(&self) -> usize {
        self.size
    }

    pub(crate) fn get(&self, coord: (usize, usize)) -> Option<Player> {
        debug_assert!(coord.0 < self.size && coord.1 < self.size);

        self.board[coord.0][coord.1]
    }

    pub(crate) fn set(&mut self, coord: (usize, usize), player: Option<Player>) {
        self.board[coord.0][coord.1] = player;
    }

    pub(crate) fn pieces_for_player(
        &self,
        player: Player,
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.board.iter().enumerate().flat_map(move |(i, row)| {
            row.iter().enumerate().filter_map(move |(j, val)| {
                if val.is_some_and(|p| p == player) {
                    Some((i, j))
                } else {
                    None
                }
            })
        })
    }

    pub(crate) fn switch_piece(&mut self, coord: (usize, usize)) {
        self.board[coord.0][coord.1] = Some(self.board[coord.0][coord.1].unwrap().other());
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let player_to_string = move |player: Option<Player>| match player {
            Some(Player::Black) => "⚫",
            Some(Player::Red) => "🔴",
            None => " ",
        };

        let val_iter = (0..=self.size).map(|i| {
            (0..=self.size).map(move |j| {
                if i == 0 {
                    j.to_string()
                } else if j == 0 {
                    i.to_string()
                } else {
                    player_to_string(self.board[i - 1][j - 1]).to_string()
                }
            })
        });

        let table = IterTable::new(val_iter).to_string();

        write!(f, "{}", table)
    }
}
