pub(crate) fn reversi_coord_to_game_coord((a, b): (usize, usize)) -> (f32, f32) {
    (a as f32 - 3.5, b as f32 - 3.5)
}

pub(crate) fn game_coord_to_reversi_coord((a, b): (f32, f32)) -> (usize, usize) {
    ((a + 3.5).round() as usize, (b + 3.5).round() as usize)
}
