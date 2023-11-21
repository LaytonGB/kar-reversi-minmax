#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BotAlgorithm {
    MinMax,
    MinMaxAlphaBeta,
    NegaMax,
}
