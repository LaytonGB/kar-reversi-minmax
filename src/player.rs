#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
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
