#[derive(Clone, Copy, PartialEq, Eq, strum::EnumString, strum::Display, strum::EnumIter, Debug)]
pub enum GameMode {
    PlayerVsAi,
    PlayerVsPlayer,
}
