#[derive(Clone, Copy, PartialEq, Eq, Hash, strum::Display, Debug)]
pub enum Player {
    Green,
    Red,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::Green => Player::Red,
            Player::Red => Player::Green,
        }
    }
}
