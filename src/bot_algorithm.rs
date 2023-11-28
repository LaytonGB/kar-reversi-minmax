#[derive(
    Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumIter, strum::EnumString, Debug,
)]
pub enum BotAlgorithm {
    MinMax,
    AlphaBeta,
    NegaMax,
}
