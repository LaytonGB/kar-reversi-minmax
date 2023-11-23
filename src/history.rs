use crate::player::Player;

#[derive(Clone, Default, Debug)]
pub struct History {
    history: Vec<(Player, (usize, usize), Vec<(usize, usize)>)>,
}

impl History {
    pub(crate) fn push(
        &mut self,
        player: Player,
        coord: (usize, usize),
        captured_pieces: Vec<(usize, usize)>,
    ) {
        self.history.push((player, coord, captured_pieces.to_vec()));
    }

    pub(crate) fn pop(&mut self) -> Option<(Player, (usize, usize), Vec<(usize, usize)>)> {
        self.history.pop()
    }
}
