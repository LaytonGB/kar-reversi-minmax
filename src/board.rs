use crate::player::Player;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Board {
    board: Vec<Vec<Option<Player>>>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            board: vec![vec![None; size]; size],
            size,
        }
    }
}
