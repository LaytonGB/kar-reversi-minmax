use crate::player::Player;

#[derive(Default, Debug)]
pub struct History {
    history: Vec<(Player, (usize, usize))>,
}
