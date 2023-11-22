#[derive(Clone, Copy, PartialEq, Eq, Hash, strum::Display, Debug)]
pub enum Player {
    Black,
    Red,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::Black => Player::Red,
            Player::Red => Player::Black,
        }
    }
}
