#[derive(Clone, Copy, PartialEq, Eq, strum::EnumString, strum::EnumIter, strum::Display, Debug)]
pub enum BotHeuristic {
    UniformWeighting,
    TacticalWeighting,
}
