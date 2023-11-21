use std::fmt::Display;

use tabled::tables::IterTable;

use crate::player::Player;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    board: Vec<Vec<Option<Player>>>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
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
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text_board_iter = self.board.iter().map(|row| {
            row.iter()
                .map(|val| match val {
                    Some(Player::Black) => "⚫",
                    Some(Player::Red) => "🔴",
                    None => " ",
                })
                .to_owned()
        });

        let table = IterTable::new(text_board_iter).to_string();

        write!(f, "{}", table)
    }
}
