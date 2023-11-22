use crate::player::Player;

#[derive(Default, Debug)]
pub struct History {
    history: Vec<(Player, (usize, usize), Vec<(usize, usize)>)>,
}

impl History {
    pub(crate) fn push(
        &mut self,
        player: Player,
        coord: (usize, usize),
        captured_pieces: &[(usize, usize)],
    ) {
        self.history.push((player, coord, captured_pieces.to_vec()));
    }
}
