#[derive(Clone, Copy, PartialEq, Eq, strum::Display, strum::EnumIter, strum::EnumString, Debug)]
pub enum BotDifficulty {
    Easy,
    Medium,
    Hard,
}
